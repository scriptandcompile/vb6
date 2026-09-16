//! Tauri engine: background interpreter thread with IPC channels.
//!
//! This module provides a [`TauriEngine`] that owns an [`Interpreter`] and
//! [`LoadedProject`] shared behind `Arc<Mutex<>>`, along with command/response
//! channels for communicating with the Tauri frontend.
//!
//! The engine runs in a dedicated background thread that processes
//! [`TauriCommand`] messages and produces [`TauriResponse`] events.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};
use std::sync::{Arc, Mutex};

use crate::interpreter::Interpreter;
use crate::project::LoadedProject;
use vb6runtime::VBVariant;

/// Background engine that runs the VB6 interpreter.
///
/// Holds the interpreter and project under `Arc<Mutex<>>` so they can be
/// shared between the main thread (which sends commands via the handle)
/// and the background thread (which executes them).
pub struct TauriEngine {
    /// The VB6 interpreter instance shared with the background thread.
    interpreter: Arc<Mutex<Interpreter>>,
    /// The loaded VB6 project shared with the background thread.
    #[allow(dead_code)]
    project: Arc<Mutex<LoadedProject>>,
    /// Indicates whether the background engine is currently running.
    running: Arc<AtomicBool>,
    /// The sender channel for commands to the background engine.
    cmd_tx: Sender<TauriCommand>,
}

/// Commands that the Tauri frontend can send to the background engine.
pub enum TauriCommand {
    /// Run the entire project from module-level statements through the
    /// startup procedure.
    RunProject,
    /// Call a named sub procedure with the given arguments.
    ExecuteSub {
        /// The procedure name (case-insensitive).
        name: String,
        /// The arguments to pass to the procedure.
        args: Vec<VBVariant>,
    },
    /// Set a global variable to the given value.
    SetVariable {
        /// The variable name (case-insensitive).
        name: String,
        /// The value to assign.
        value: VBVariant,
    },
    /// Get the current value of a global variable.
    GetVariable {
        /// The variable name (case-insensitive).
        name: String,
    },
    /// Show a form by its VB6 name.
    ShowForm {
        /// The form name (from `Attribute VB_Name`).
        name: String,
    },
    /// Hide a form by its VB6 name.
    HideForm {
        /// The form name (from `Attribute VB_Name`).
        name: String,
    },
    /// Unload a form by its VB6 name.
    UnloadForm {
        /// The form name (from `Attribute VB_Name`).
        name: String,
    },
    /// Stop the background engine.
    Stop,
}

/// Responses emitted by the background engine to the frontend.
pub enum TauriResponse {
    /// The engine has started running.
    Running,
    /// The engine finished running (project completed).
    Finished,
    /// A runtime error occurred.
    Error(String),
    /// Captured `Debug.Print`/`Print` output.
    Output(String),
    /// The value of a variable returned in response to `GetVariable`.
    Variable(String, VBVariant),
    /// A form was loaded/shown successfully.
    FormLoaded(String),
}

impl TauriEngine {
    /// Create a new TauriEngine and spawn its background thread.
    ///
    /// The background thread starts in a waiting state, processing commands
    /// from the command channel. Use the [`TauriEngineHandle`] methods to
    /// send commands and receive responses.
    ///
    /// Returns the engine handle (for sending commands) and a receiver for
    /// responses that the caller can poll on the main thread.
    pub fn spawn(project: LoadedProject) -> (Self, Receiver<TauriResponse>) {
        let interpreter = Interpreter::new();
        let cmd_tx: Sender<TauriCommand> = channel().0;
        let cmd_rx: Receiver<TauriCommand> = channel().1;
        let resp_tx: Sender<TauriResponse> = channel().0;
        let resp_rx: Receiver<TauriResponse> = channel().1;

        let interpreter = Arc::new(Mutex::new(interpreter));
        let project = Arc::new(Mutex::new(project));
        let running = Arc::new(AtomicBool::new(true));

        let resp_tx_clone = resp_tx.clone();
        let interpreter_clone = interpreter.clone();
        let project_clone = project.clone();
        let running_clone = running.clone();

        std::thread::spawn(move || {
            loop {
                // Check for commands (non-blocking)
                match cmd_rx.try_recv() {
                    Ok(TauriCommand::RunProject) => {
                        resp_tx_clone.send(TauriResponse::Running).ok();

                        let result = {
                            let mut interp = interpreter_clone.lock().unwrap();
                            let proj = project_clone.lock().unwrap();
                            interp.run_project(&proj)
                        };

                        match result {
                            Ok(()) => {
                                resp_tx_clone.send(TauriResponse::Finished).ok();
                            }
                            Err(e) => {
                                resp_tx_clone
                                    .send(TauriResponse::Error(e.error.to_string()))
                                    .ok();
                            }
                        }
                    }
                    Ok(TauriCommand::ExecuteSub { name, args }) => {
                        let result = {
                            let mut interp = interpreter_clone.lock().unwrap();
                            interp.call_sub(&name, args)
                        };
                        if let Err(e) = result {
                            resp_tx_clone
                                .send(TauriResponse::Error(e.error.to_string()))
                                .ok();
                        }
                    }
                    Ok(TauriCommand::SetVariable { name, value }) => {
                        let mut interp = interpreter_clone.lock().unwrap();
                        interp.set_global(&name, value);
                    }
                    Ok(TauriCommand::GetVariable { name }) => {
                        let interp = interpreter_clone.lock().unwrap();
                        if let Some(value) = interp.global(&name) {
                            resp_tx_clone
                                .send(TauriResponse::Variable(name, value.clone()))
                                .ok();
                        }
                    }
                    Ok(TauriCommand::ShowForm { name }) => {
                        resp_tx_clone.send(TauriResponse::FormLoaded(name)).ok();
                    }
                    Ok(TauriCommand::HideForm { .. }) | Ok(TauriCommand::UnloadForm { .. }) => {
                        // TODO: implement form hiding/unloading
                    }
                    Ok(TauriCommand::Stop) => {
                        break;
                    }
                    Err(TryRecvError::Empty) => {
                        // Poll output
                        let interp = interpreter_clone.lock().unwrap();
                        let output = interp.output_text();
                        if !output.is_empty() {
                            resp_tx_clone.send(TauriResponse::Output(output)).ok();
                        }
                        drop(interp);
                        std::thread::sleep(std::time::Duration::from_millis(50));
                    }
                    Err(TryRecvError::Disconnected) => break,
                }

                if !running_clone.load(Ordering::SeqCst) {
                    break;
                }
            }

            resp_tx_clone.send(TauriResponse::Finished).ok();
        });

        let engine = TauriEngine {
            interpreter,
            project,
            running,
            cmd_tx,
        };

        (engine, resp_rx)
    }

    /// Get a handle for sending commands to the background engine.
    pub fn handle(&self) -> TauriEngineHandle {
        TauriEngineHandle {
            interpreter: self.interpreter.clone(),
            running: self.running.clone(),
            cmd_tx: self.cmd_tx.clone(),
        }
    }

    /// Stop the background engine thread.
    pub fn stop(&self) {
        self.cmd_tx.send(TauriCommand::Stop).ok();
        self.running.store(false, Ordering::SeqCst);
    }

    /// Whether the engine is still running.
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

/// A handle for sending commands to a [`TauriEngine`] from the main thread.
///
/// Holds a reference to the interpreter and a running flag, allowing
/// direct manipulation without going through the command channel.
pub struct TauriEngineHandle {
    interpreter: Arc<Mutex<Interpreter>>,
    running: Arc<AtomicBool>,
    cmd_tx: Sender<TauriCommand>,
}

impl TauriEngineHandle {
    /// Send a command to the background engine via the command channel.
    ///
    /// This is called by Tauri IPC handlers to queue commands for the
    /// background thread.
    pub fn send_command(&self, cmd: TauriCommand) -> bool {
        // The actual command sending is handled by tauri_cmds.rs which
        // uses a global store. This method is provided for direct access
        // from tests or other non-Tauri code paths.
        let mut interp = self.interpreter.lock().unwrap();
        match cmd {
            TauriCommand::SetVariable { name, value } => {
                interp.set_global(&name, value);
                true
            }
            TauriCommand::GetVariable { name } => {
                let _ = interp.global(&name);
                true
            }
            TauriCommand::ExecuteSub { name, args } => {
                matches!(interp.call_sub(&name, args), Ok(_))
            }
            TauriCommand::RunProject
            | TauriCommand::ShowForm { .. }
            | TauriCommand::HideForm { .. }
            | TauriCommand::UnloadForm { .. }
            | TauriCommand::Stop => false,
        }
    }

    /// Send a command to the background thread.
    pub fn send(&self, cmd: TauriCommand) -> bool {
        self.cmd_tx.send(cmd).is_ok()
    }

    /// Get the current output text from the interpreter.
    pub fn get_output(&self) -> String {
        let interp = self.interpreter.lock().unwrap();
        interp.output_text()
    }

    /// Whether the engine is still running.
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}
