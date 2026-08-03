//! # `SendKeys` Statement
//!
//! Sends one or more keystrokes to the active window as if typed at the keyboard.
//!
//! ## Syntax
//!
//! ```vb
//! SendKeys string [, wait]
//! ```
//!
//! ## Parts
//!
//! - **string**: Required. String expression specifying the keystrokes to send.
//! - **wait**: Optional. Boolean value specifying the wait mode. If True, Visual Basic waits for the keystrokes to be processed before returning control to the calling procedure. If False (default), control returns immediately after the keys are sent.
//!
//! ## Remarks
//!
//! - **Active Window**: `SendKeys` sends keystrokes to the currently active window. Your application must activate the target window before using `SendKeys`.
//! - **Keystroke Representation**: Each key is represented by one or more characters. To specify a single keyboard character, use the character itself (e.g., "A" sends the letter A).
//! - **Multiple Characters**: To send a string of characters, concatenate them (e.g., "Hello" sends H, e, l, l, o in sequence).
//! - **Special Keys**: Some keys have special representations enclosed in braces (e.g., {ENTER}, {TAB}, {ESC}).
//! - **Wait Parameter**: Setting wait to True ensures that keystrokes are processed before your code continues. This is useful when you need to wait for an application to respond.
//! - **Focus Issues**: If the target application doesn't have focus when `SendKeys` executes, the keystrokes may be sent to the wrong application.
//! - **`AppActivate`**: Use `AppActivate` to activate the target window before calling `SendKeys`.
//!
//! ## Special Key Codes
//!
//! | Key | Code |
//! |-----|------|
//! | BACKSPACE | {BACKSPACE} or {BS} or {BKSP} |
//! | BREAK | {BREAK} |
//! | CAPS LOCK | {CAPSLOCK} |
//! | DELETE | {DELETE} or {DEL} |
//! | DOWN ARROW | {DOWN} |
//! | END | {END} |
//! | ENTER | {ENTER} or ~ |
//! | ESC | {ESC} or {ESCAPE} |
//! | HELP | {HELP} |
//! | HOME | {HOME} |
//! | INSERT | {INSERT} or {INS} |
//! | LEFT ARROW | {LEFT} |
//! | NUM LOCK | {NUMLOCK} |
//! | PAGE DOWN | {PGDN} |
//! | PAGE UP | {PGUP} |
//! | PRINT SCREEN | {PRTSC} |
//! | RIGHT ARROW | {RIGHT} |
//! | SCROLL LOCK | {SCROLLLOCK} |
//! | TAB | {TAB} |
//! | UP ARROW | {UP} |
//! | F1-F16 | {F1} through {F16} |
//!
//! ## Modifier Keys
//!
//! | Key | Code |
//! |-----|------|
//! | SHIFT | + (plus sign) |
//! | CTRL | ^ (caret) |
//! | ALT | % (percent sign) |
//!
//! To specify modifier keys with regular keys, enclose the regular keys in parentheses:
//! - `"+{F1}"` sends SHIFT+F1
//! - `"^(ec)"` sends CTRL+E followed by CTRL+C
//! - `"%(FA)"` sends ALT+F followed by ALT+A
//!
//! ## Repeating Keys
//!
//! To repeat a key, use the format `{key number}`:
//! - `"{RIGHT 10}"` sends RIGHT arrow 10 times
//! - `"{TAB 5}"` sends TAB 5 times
//!
//! ## Examples
//!
//! ### Send Simple Text
//!
//! ```vb
//! SendKeys "Hello World"
//! ```
//!
//! ### Send Text with Enter Key
//!
//! ```vb
//! SendKeys "Username{TAB}Password{ENTER}"
//! ```
//!
//! ### Activate Window and Send Keys
//!
//! ```vb
//! AppActivate "Notepad"
//! SendKeys "Hello from VB6{ENTER}", True
//! ```
//!
//! ### Send Alt+F4 to Close Window
//!
//! ```vb
//! SendKeys "%{F4}"  ' ALT+F4
//! ```
//!
//! ### Send Ctrl+C to Copy
//!
//! ```vb
//! SendKeys "^c"  ' CTRL+C
//! ```
//!
//! ### Send Multiple Keys with Wait
//!
//! ```vb
//! SendKeys "{DOWN}{DOWN}{ENTER}", True
//! ```
//!
//! ### Fill Form Fields
//!
//! ```vb
//! AppActivate "Data Entry Form"
//! SendKeys "John Doe{TAB}123 Main St{TAB}555-1234{ENTER}", True
//! ```
//!
//! ### Send Function Keys
//!
//! ```vb
//! SendKeys "{F1}"    ' Help key
//! SendKeys "{F5}"    ' Refresh
//! SendKeys "+{F10}"  ' SHIFT+F10 (context menu)
//! ```
//!
//! ### Repeat Keys
//!
//! ```vb
//! SendKeys "{RIGHT 5}"    ' Move right 5 times
//! SendKeys "{DOWN 10}"    ' Move down 10 times
//! SendKeys "{BACKSPACE 3}" ' Delete 3 characters
//! ```
//!
//! ### Send Key Combinations
//!
//! ```vb
//! SendKeys "^a"       ' CTRL+A (Select All)
//! SendKeys "^c"       ' CTRL+C (Copy)
//! SendKeys "^v"       ' CTRL+V (Paste)
//! SendKeys "^s"       ' CTRL+S (Save)
//! ```
//!
//! ### Navigate Menus
//!
//! ```vb
//! AppActivate "Microsoft Word"
//! SendKeys "%f", True  ' ALT+F (File menu)
//! SendKeys "s", True   ' S (Save)
//! ```
//!
//! ### Send Special Characters
//!
//! ```vb
//! SendKeys "Test {+} Addition"  ' Sends: Test + Addition
//! SendKeys "Test {^} Power"     ' Sends: Test ^ Power
//! SendKeys "Test {% } Percent"  ' Sends: Test % Percent
//! ```
//!
//! ## Important Notes
//!
//! - **Timing**: `SendKeys` is not always reliable for complex automation. Consider using API calls or UI automation libraries for critical tasks.
//! - **Focus Management**: Always ensure the target window has focus before sending keys.
//! - **Wait Parameter**: Use True for the wait parameter when you need synchronous operation.
//! - **Case Sensitivity**: To send uppercase letters, use the SHIFT modifier: `"+abc"` sends uppercase ABC.
//! - **Reserved Characters**: To send +, ^, %, ~, or {}, enclose them in braces: `{+}`, `{^}`, `{%}`, `{~}`, `{{}`, `{}}`.
//! - **Limitations**: `SendKeys` doesn't work with applications that directly process keyboard input at a low level.
//! - **Error Handling**: If the target application is busy or unresponsive, `SendKeys` may fail silently or send keys to the wrong window.
//!
//! ## Common Errors
//!
//! - **Error 5**: Invalid procedure call - occurs if string contains invalid key codes
//! - Keys sent to wrong application if focus isn't properly managed
//! - Timing issues when wait is False and subsequent code depends on keystrokes being processed
//!
//! ## Best Practices
//!
//! - Always use `AppActivate` before `SendKeys` to ensure the correct window receives the keystrokes
//! - Use the wait parameter (True) when the next operation depends on the keystrokes being processed
//! - Add delays (`DoEvents` or Sleep) between `SendKeys` calls for complex sequences
//! - Test thoroughly as `SendKeys` behavior can vary across different applications and Windows versions
//! - Consider alternatives like Windows API or UI Automation for production applications
//!
//! ## See Also
//!
//! - `AppActivate` statement (activate an application window)
//! - `DoEvents` function (yield execution to allow events to be processed)
//! - `Shell` function (run executable programs)
//!
//! ## References
//!
//! - [SendKeys Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/sendkeys-statement)
