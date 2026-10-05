use std::io::IsTerminal;

/// RAII guard for the terminal. On a TTY it enables raw mode, the alternate
/// screen, and mouse capture; `Drop` reverses all of it unconditionally.
/// On a non-TTY or TERM=dumb it is inert and emits no escape sequences.
pub struct TerminalGuard {
    active: bool,
}

impl TerminalGuard {
    pub fn init() -> std::io::Result<Self> {
        if !Self::is_interactive() {
            return Ok(Self { active: false });
        }
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(
            std::io::stdout(),
            crossterm::terminal::EnterAlternateScreen,
            crossterm::event::EnableMouseCapture
        )?;
        Ok(Self { active: true })
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn is_interactive() -> bool {
        std::io::stdout().is_terminal() && std::env::var("TERM").ok().as_deref() != Some("dumb")
    }

    /// Restore the terminal even from a panic handler or signal path.
    pub fn restore_now() {
        let _ = crossterm::execute!(
            std::io::stdout(),
            crossterm::event::DisableMouseCapture,
            crossterm::terminal::LeaveAlternateScreen
        );
        let _ = crossterm::terminal::disable_raw_mode();
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        if self.active {
            Self::restore_now();
        }
    }
}
