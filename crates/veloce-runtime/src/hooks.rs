use std::sync::Once;

use crate::guard::TerminalGuard;

static HOOK: Once = Once::new();

/// Restores the terminal BEFORE the default hook prints the backtrace.
pub fn install_panic_hook() {
    HOOK.call_once(|| {
        let default_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            TerminalGuard::restore_now();
            default_hook(info);
        }));
    });
}
