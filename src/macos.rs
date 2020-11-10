//! This module defines the unix thread control.
//!
//! The crate's prelude doesn't have much control over
//! the unix threads, and this module provides
//! better control over those.

use crate::{Error, ThreadPriority};

/// Return a placeholder type for the ThreadId. This is used in association with other types which are always errors on this unsupported platform (macOS) so this allows complilation and calling code has the option to handle the error gracefully at runtime
pub type ThreadId = i32;

/// Set current thread's priority.
pub fn set_current_thread_priority(_priority: ThreadPriority) -> Result<(), Error> {
    // Silently do nothing- not a supported platform- priority will remain unchanged
    Err(Error::UnsupportedPlatform())
}

/// Get current thread's priority value.
pub fn thread_priority() -> Result<ThreadPriority, Error> {
    // Indicate that this is not a supported platform
    Err(Error::UnsupportedPlatform())
}

/// Returns a placeholder for the current thread id.  This is used in association with other types which are always errors on this unsupported platform (macOS) so this allows complilation and calling code has the option to handle the error gracefully at runtime
pub fn thread_native_id() -> ThreadId {
    0
}
