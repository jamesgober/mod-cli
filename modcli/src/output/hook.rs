use crate::output::messages;
use crate::output::print;
#[cfg(feature = "tracing-logs")]
use tracing::{debug as tdebug, error as terror, info as tinfo, warn as twarn};

/// Hook for debug messages
pub fn debug(msg: &str) {
    let m = messages::intercept("debug", msg);
    print::debug(&m);
    #[cfg(feature = "tracing-logs")]
    tdebug!(target: "modcli", message = %m);
}

/// Hook for info-level messages
pub fn info(msg: &str) {
    let m = messages::intercept("info", msg);
    print::info(&m);
    #[cfg(feature = "tracing-logs")]
    tinfo!(target: "modcli", message = %m);
}

/// Hook for warning-level messages
pub fn warn(msg: &str) {
    let m = messages::intercept("warn", msg);
    print::warn(&m);
    #[cfg(feature = "tracing-logs")]
    twarn!(target: "modcli", message = %m);
}

/// Hook for error-level messages
pub fn error(msg: &str) {
    let m = messages::intercept("error", msg);
    print::error(&m);
    #[cfg(feature = "tracing-logs")]
    terror!(target: "modcli", message = %m);
}

/// Hook for success-level messages
pub fn success(msg: &str) {
    let m = messages::intercept("success", msg);
    print::success(&m);
    #[cfg(feature = "tracing-logs")]
    tinfo!(target: "modcli", success = true, message = %m);
}

/// Hook for status-level messages
pub fn status(msg: &str) {
    let m = messages::intercept("status", msg);
    print::status(&m);
    #[cfg(feature = "tracing-logs")]
    tinfo!(target: "modcli", status = true, message = %m);
}

/// Hook for deprecated messages
pub fn deprecated(msg: &str) {
    let m = messages::intercept("deprecated", msg);
    print::deprecated(&m);
    #[cfg(feature = "tracing-logs")]
    twarn!(target: "modcli", deprecated = true, message = %m);
}

/// Hook for unknown command situations
pub fn unknown(msg: &str) {
    let m = messages::intercept("unknown", msg);
    print::unknown(&m);
    #[cfg(feature = "tracing-logs")]
    twarn!(target: "modcli", unknown_command = true, message = %m);
}
