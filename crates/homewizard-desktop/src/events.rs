//! Event emission helpers.

use std::fmt::Display;

/// Log a warning if `result` is `Err`, otherwise do nothing.
///
/// Use to wrap `AppHandle::emit` calls so silent drops become observable
/// in the log stream instead of vanishing into `let _ = ...`.
pub(crate) fn warn_on_err<E: Display>(context: &str, result: Result<(), E>) {
    if let Err(e) = result {
        tracing::warn!("{context}: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn logs_warning_on_err() {
        warn_on_err("emit_failure", Err::<(), &str>("channel closed"));
        assert!(logs_contain("emit_failure"));
        assert!(logs_contain("channel closed"));
    }

    #[traced_test]
    #[test]
    fn does_not_log_on_ok() {
        warn_on_err::<&str>("emit_ok", Ok(()));
        assert!(!logs_contain("emit_ok"));
    }
}
