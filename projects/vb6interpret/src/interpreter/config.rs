//! Host-facing configuration knobs: environment overrides, staged
//! application settings, the linked resource file, runtime state backends
//! (settings/file/clock/interaction), and the mock-clock controls (initial
//! date/time and `allow_system_time`).
//!
//! Everything here is staged on the interpreter and applied to the shared
//! `vb6runtime` state snapshot at the start of every run, so hosts configure
//! once and re-run freely.

use vb6runtime::state::environment as env_state;
use vb6runtime::state::resources as resources_state;
use vb6runtime::state::settings as settings_state;

use crate::interpreter::Interpreter;

impl Interpreter {
    /// Assign an environment variable before the next run.
    ///
    /// `Environ$`/`Environ` read these values during execution, on top of the
    /// process environment. The assignment survives [`Interpreter::clear`] and
    /// is re-applied at the start of every run, so it can be configured once
    /// before calling [`Interpreter::run_source`] or [`Interpreter::run_module`].
    pub fn set_environment(&mut self, name: &str, value: &str) {
        self.environment.insert(name.to_string(), value.to_string());
    }

    /// Clear all environment variables installed with [`Interpreter::set_environment`].
    pub fn clear_environment(&mut self) {
        for name in self.environment.keys() {
            env_state::remove_env(name);
        }
        self.environment.clear();
    }

    /// Assign an application setting before the next run.
    ///
    /// `GetSetting` reads these values during execution, on top of any values
    /// already present in the settings store (or on disk). The assignment
    /// survives [`Interpreter::clear`] and is re-applied at the start of every
    /// run, so it can be configured once before calling
    /// [`Interpreter::run_source`] or [`Interpreter::run_module`]. A setting
    /// staged later overrides an earlier one with the same
    /// `(appname, section, key)`.
    pub fn set_setting(&mut self, appname: &str, section: &str, key: &str, value: &str) {
        self.settings.push((
            appname.to_string(),
            section.to_string(),
            key.to_string(),
            value.to_string(),
        ));
    }

    /// The value for `(appname, section, key)`, or `None` when unset.
    ///
    /// Staged settings win over values already in the store; among staged
    /// settings the most recently staged value wins.
    pub fn get_setting(&self, appname: &str, section: &str, key: &str) -> Option<String> {
        for (a, s, k, v) in self.settings.iter().rev() {
            if a.eq_ignore_ascii_case(appname)
                && s.eq_ignore_ascii_case(section)
                && k.eq_ignore_ascii_case(key)
            {
                return Some(v.clone());
            }
        }
        settings_state::get(appname, section, key)
    }

    /// Remove a single setting, both staged and from the store.
    pub fn remove_setting(&mut self, appname: &str, section: &str, key: &str) {
        self.settings.retain(|(a, s, k, _)| {
            !(a.eq_ignore_ascii_case(appname)
                && s.eq_ignore_ascii_case(section)
                && k.eq_ignore_ascii_case(key))
        });
        let _ = settings_state::remove_key(appname, section, key);
    }

    /// Remove every setting staged with [`Interpreter::set_setting`], both
    /// staged and from the store.
    pub fn clear_settings(&mut self) {
        for (appname, section, key, _) in &self.settings {
            let _ = settings_state::remove_key(appname, section, key);
        }
        self.settings.clear();
    }

    /// Redirect the settings store to `root` for this interpreter.
    ///
    /// Equivalent to [`vb6runtime::state::settings::set_store_root`], scoped
    /// to the interpreter for convenience.
    pub fn set_settings_store_root(&self, root: impl Into<std::path::PathBuf>) {
        settings_state::set_store_root(root);
    }

    /// Set the active settings backend for this interpreter.
    ///
    /// Equivalent to [`vb6runtime::state::settings::set_backend`], scoped
    /// to the interpreter for convenience. After switching, all settings
    /// are reloaded from the new backend.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use vb6interpret::Interpreter;
    /// use vb6runtime::state::settings::memory::MemoryBackend;
    ///
    /// let mut interp = Interpreter::new();
    /// interp.set_settings_backend(Box::new(MemoryBackend::new()));
    /// ```
    pub fn set_settings_backend(
        &self,
        backend: Box<dyn vb6runtime::state::settings::backend::SettingsBackend>,
    ) {
        settings_state::set_backend(backend);
    }

    /// Reset the settings backend to the platform default.
    ///
    /// Equivalent to [`vb6runtime::state::settings::reset_backend`], scoped
    /// to the interpreter for convenience.
    pub fn reset_settings_backend(&self) {
        settings_state::reset_backend();
    }

    /// Link the `.res` file the program's `LoadRes*` functions read from.
    ///
    /// A VB6 project links exactly one resource file at compile time (the
    /// `ResFile32=` entry in the `.vbp`), and `LoadResData`, `LoadResPicture`,
    /// and `LoadResString` all read from it without naming it. This stages the
    /// equivalent binding: it survives [`Interpreter::clear`] and is applied at
    /// the start of every run, so it can be configured once before calling
    /// [`Interpreter::run_source`] or [`Interpreter::run_module`].
    ///
    /// `path` is resolved through the active file backend at first use, so a
    /// relative path is taken against the runtime file root and a missing file
    /// is not reported until a `LoadRes*` call needs it — matching VB6, where a
    /// broken resource link surfaces as a run-time error.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use vb6interpret::Interpreter;
    ///
    /// let mut interp = Interpreter::new();
    /// interp.set_resource_file("MyApp.res");
    /// interp.run_source("Debug.Print LoadResString(1001)")?;
    /// ```
    pub fn set_resource_file(&mut self, path: impl Into<String>) {
        self.resource_file = Some(path.into());
    }

    /// The staged `.res` file path, or `None` when no resource file is linked.
    pub fn resource_file(&self) -> Option<&str> {
        self.resource_file.as_deref()
    }

    /// Unlink the resource file, as a project with no `ResFile32=` entry.
    ///
    /// Subsequent `LoadRes*` calls raise error 326.
    pub fn clear_resource_file(&mut self) {
        self.resource_file = None;
        resources_state::clear();
    }

    /// Set the active file backend for this interpreter.
    ///
    /// Equivalent to [`vb6runtime::state::file::set_backend`], scoped
    /// to the interpreter for convenience.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use vb6interpret::Interpreter;
    /// use vb6runtime::state::file::memory::MemoryBackend;
    ///
    /// let mut interp = Interpreter::new();
    /// interp.set_file_backend(Box::new(MemoryBackend::new()));
    /// ```
    pub fn set_file_backend(&self, backend: Box<dyn vb6runtime::state::file::FileBackend>) {
        vb6runtime::state::file::set_backend(backend);
    }

    /// Reset the file backend to the platform default.
    ///
    /// Equivalent to [`vb6runtime::state::file::reset_backend`], scoped
    /// to the interpreter for convenience.
    pub fn reset_file_backend(&self) {
        vb6runtime::state::file::reset_backend();
    }

    /// Set the active clock backend for this interpreter.
    ///
    /// Equivalent to [`vb6runtime::state::clock::set_backend`], scoped
    /// to the interpreter for convenience.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use vb6interpret::Interpreter;
    /// use vb6runtime::state::clock::memory::MemoryBackend;
    ///
    /// let mut interp = Interpreter::new();
    /// interp.set_clock_backend(Box::new(MemoryBackend::new()));
    /// ```
    pub fn set_clock_backend(&self, backend: Box<dyn vb6runtime::state::clock::ClockBackend>) {
        vb6runtime::state::clock::set_backend(backend);
    }

    /// Reset the clock backend to the platform default.
    ///
    /// Equivalent to [`vb6runtime::state::clock::reset_backend`], scoped
    /// to the interpreter for convenience.
    pub fn reset_clock_backend(&self) {
        vb6runtime::state::clock::reset_backend();
    }

    /// Set the active interaction backend for this interpreter.
    ///
    /// Equivalent to [`vb6runtime::state::interaction::set_backend`],
    /// scoped to the interpreter for convenience. Hosts use this to install
    /// a memory backend whose `MsgBox` dialogs are answered from a scripted
    /// response list instead of showing real windows.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use vb6interpret::Interpreter;
    /// use vb6runtime::state::interaction::{memory::MemoryBackend, MsgBoxButton};
    ///
    /// let mut interp = Interpreter::new();
    /// let backend = MemoryBackend::with_msgbox_responses([MsgBoxButton::Yes]);
    /// interp.set_interaction_backend(Box::new(backend));
    /// ```
    pub fn set_interaction_backend(
        &self,
        backend: Box<dyn vb6runtime::state::interaction::InteractionBackend>,
    ) {
        vb6runtime::state::interaction::set_backend(backend);
    }

    /// Reset the interaction backend to the platform default.
    ///
    /// Equivalent to [`vb6runtime::state::interaction::reset_backend`],
    /// scoped to the interpreter for convenience.
    pub fn reset_interaction_backend(&self) {
        vb6runtime::state::interaction::reset_backend();
    }

    /// Control whether `Date` and `Time` statements may modify the real
    /// system clock.
    ///
    /// - `true` (default): statements write to the real system clock.
    /// - `false`: statements write to an internal mock clock that advances
    ///   in real time from the set point.  The real clock is never touched.
    ///
    /// When set to `false`, the current real date/time is captured as the
    /// mock clock's starting point (unless overridden with
    /// [`set_initial_date`] or [`set_initial_time`]).
    pub fn set_allow_system_time(&mut self, allowed: bool) {
        self.allow_system_time = allowed;
    }

    /// Whether `Date` and `Time` statements may modify the real system clock.
    pub fn allow_system_time(&self) -> bool {
        self.allow_system_time
    }

    /// Set an initial date for the mock clock at the start of a run.
    ///
    /// Automatically disables real-clock writes (equivalent to calling
    /// [`set_allow_system_time(false)`](Self::set_allow_system_time)).
    /// When set, the mock clock starts at this date (preserving the current
    /// time-of-day) instead of the real system date.
    pub fn set_initial_date(&mut self, date: vb6runtime::civil::Date) {
        self.allow_system_time = false;
        self.initial_date = Some(date);
    }

    /// Clear any initial date override.
    pub fn clear_initial_date(&mut self) {
        self.initial_date = None;
    }

    /// Set an initial time for the mock clock at the start of a run.
    ///
    /// Automatically disables real-clock writes (equivalent to calling
    /// [`set_allow_system_time(false)`](Self::set_allow_system_time)).
    /// When set, the mock clock starts at this time (preserving the current
    /// date) instead of the real system time.
    pub fn set_initial_time(&mut self, time: vb6runtime::civil::Time) {
        self.allow_system_time = false;
        self.initial_time = Some(time);
    }

    /// Set both the initial date and time for the mock clock at the start
    /// of a run.
    ///
    /// Automatically disables real-clock writes.  This is a convenience
    /// shorthand for calling [`set_initial_date`] and [`set_initial_time`]
    /// together.
    pub fn set_initial_date_time(
        &mut self,
        date: vb6runtime::civil::Date,
        time: vb6runtime::civil::Time,
    ) {
        self.allow_system_time = false;
        self.initial_date = Some(date);
        self.initial_time = Some(time);
    }

    /// Clear any initial time override.
    pub fn clear_initial_time(&mut self) {
        self.initial_time = None;
    }
}
