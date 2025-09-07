use crate::output::print;

/// Hook for debug messages
pub fn debug(msg: &str) {
    print::debug(msg);
}

/// Hook for info-level messages
pub fn info(msg: &str) {
    print::info(msg);
}

/// Hook for warning-level messages
pub fn warn(msg: &str) {
    print::warn(msg);
}

/// Hook for error-level messages
pub fn error(msg: &str) {
    print::error(msg);
}

/// Hook for success-level messages
pub fn success(msg: &str) {
    print::success(msg);
}

/// Hook for status-level messages
pub fn status(msg: &str) {
    print::status(msg);
}

/// Hook for deprecated messages
pub fn deprecated(msg: &str) {
    print::deprecated(msg);
}

/// Hook for unknown command situations
pub fn unknown(msg: &str) {
    print::unknown(msg);
}
