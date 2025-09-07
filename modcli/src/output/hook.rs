use crate::output::print;
#[cfg(feature = "tracing-logs")]
use tracing::{debug as tdebug, error as terror, info as tinfo, warn as twarn};

/// Hook for debug messages
pub fn debug(msg: &str) {
    print::debug(msg);
    #[cfg(feature = "tracing-logs")]
    tdebug!(target: "modcli", message = %msg);
}

/// Hook for info-level messages
pub fn info(msg: &str) {
    print::info(msg);
    #[cfg(feature = "tracing-logs")]
    tinfo!(target: "modcli", message = %msg);
}

/// Hook for warning-level messages
pub fn warn(msg: &str) {
    print::warn(msg);
    #[cfg(feature = "tracing-logs")]
    twarn!(target: "modcli", message = %msg);
}

/// Hook for error-level messages
pub fn error(msg: &str) {
    print::error(msg);
    #[cfg(feature = "tracing-logs")]
    terror!(target: "modcli", message = %msg);
}

/// Hook for success-level messages
pub fn success(msg: &str) {
    print::success(msg);
    #[cfg(feature = "tracing-logs")]
    tinfo!(target: "modcli", success = true, message = %msg);
}

/// Hook for status-level messages
pub fn status(msg: &str) {
    print::status(msg);
    #[cfg(feature = "tracing-logs")]
    tinfo!(target: "modcli", status = true, message = %msg);
}

/// Hook for deprecated messages
pub fn deprecated(msg: &str) {
    print::deprecated(msg);
    #[cfg(feature = "tracing-logs")]
    twarn!(target: "modcli", deprecated = true, message = %msg);
}

/// Hook for unknown command situations
pub fn unknown(msg: &str) {
    print::unknown(msg);
    #[cfg(feature = "tracing-logs")]
    twarn!(target: "modcli", unknown_command = true, message = %msg);
}
