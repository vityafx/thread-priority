//! Thread priority. A library for changing thread's priority.
//!
//! # Usage
//!
//! Setting thread priority to minimum:
//!
//! ```rust
//! use thread_priority::*;
//!
//! let result = set_current_thread_priority(ThreadPriority::Min);
//! // Or like this:
//! let result = ThreadPriority::Min.set_for_current();
//! ```
#![warn(missing_docs)]
#![deny(warnings)]

#[cfg(all(unix, not(target_os = "macos")))]
pub mod unix;
#[cfg(all(unix, not(target_os = "macos")))]
pub use unix::*;
#[cfg(windows)]
pub mod windows;
#[cfg(windows)]
pub use windows::*;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

/// A error type
#[derive(Debug, Copy, Clone)]
pub enum Error {
    /// A value which describes why it is impossible to use such a priority.
    Priority(&'static str),
    /// Target OS' error type. In most systems it is an integer which
    /// later should be used with target OS' API for understanding the value.
    OS(i32),
    /// FFI failure
    Ffi(&'static str),
    /// Attempt to probe values on an platform which is not supported (macOS..)
    UnsupportedPlatform(),
}

/// Thread priority enumeration.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ThreadPriority {
    /// Holds a value representing the minimum possible priority.
    Min,
    /// Holds a specific priority value. Should be in [0; 100] range,
    /// a percentage value. The `u32` value is reserved for different
    /// OS'es support.
    Specific(u32),
    /// Holds a value representing the maximum possible priority.
    /// Should be used with caution, it solely depends on the target
    /// os where the program is going to be running on, how it will
    /// behave. On some systems, the whole system may become frozen
    /// if not used properly.
    Max,
}

impl ThreadPriority {
    /// Sets current thread's priority to this value.
    ///
    ///
    /// # Usage
    ///
    /// ```rust
    /// use thread_priority::*;
    ///
    /// // Ignore the response- we just keep going even if priority changes on a given platform are not supported (macOS)
    /// let _ = set_current_thread_priority(ThreadPriority::Max);
    /// ```
    pub fn set_for_current(self) -> Result<(), Error> {
        set_current_thread_priority(self)
    }
}

/// Represents an OS thread.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Thread {
    /// Thread's priority.
    pub priority: ThreadPriority,
    /// Thread's ID (or handle).
    pub id: ThreadId,
}

impl Thread {
    /// Get current thread as a platform-independent structure
    ///
    /// # Usage
    ///
    /// ```rust
    /// use thread_priority::*;
    ///
    /// let thread = Thread::current();
    /// ```
    pub fn current() -> Result<Thread, Error> {
        Ok(Thread {
            priority: thread_priority()?,
            id: thread_native_id(),
        })
    }
}
