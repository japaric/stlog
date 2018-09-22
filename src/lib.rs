//! Ultra lightweight logging framework for resource constrained devices
//!
//! ![stlog running on a Cortex-M microcontroller](https://i.imgur.com/rPxmAlZ.jpg)
//!
//! **[See stlog in action!](https://streamable.com/nmlx7)**
//!
//! # Features
//!
//! - `O(1)` execution time. Logging a message of arbitrary size is done in a constant number of
//! instructions.
//!
//! - `O(0)` memory usage. The messages are NOT stored in the target device memory.
//!
//! - Supports different logging levels: error, warning, info, debug and trace, in decreasing level
//! of severity. By default, the `dev` profile logs debug, and more severe, messages and the
//! `release` profile logs info, and more severe, messages, but this can changed using the Cargo
//! features of this crate.
//!
//! - Provides a global logging mode
//!
//! # Non-features
//!
//! - `printf` style or any other kind of formatting
//!
//! # Known limitations
//!
//! - The current implementation only supports 256 different log strings. This restriction may be
//! lifted in the future.
//!
//! - The exact same string can't be used in two or more macro invocations. This restriction will be
//! lifted when procedural macros that expand into expressions are allowed on stable.
//!
//! ``` ignore
//! use stlog::{error, info};
//!
//! fn foo() {
//!     info!("Hello!");
//! }
//!
//! fn good() {
//!     foo();
//!     foo();
//! }
//!
//! fn bad() {
//!     info!("Hey!");
//!     info!("Hey!"); //~ ERROR symbol `Hey!` is already defined
//! }
//!
//! fn also_bad() {
//!     info!("Bye!");
//!     error!("Bye!"); //~ ERROR symbol `Bye!` is already defined
//! }
//! ```
//!
//! # Requirements
//!
//! The target application must be linked using the `stlog.x` linker script provided by this crate.
//! The easiest way to do this is to append the `-C link-arg` to the other rustc flags using a Cargo
//! configuration file (`.cargo/config`).
//!
//! ``` toml
//! [target.thumbv7m-none-eabi]
//! rustflags = [
//!     "-C", "link-arg=-Tstlog.x",
//!     # ..
//! ]
//! ```
//!
//! To decode the logs on the host you'll need version v0.2.x of the [`stcat`] tool.
//!
//! [`stcat`]: https://crates.io/crates/stcat
//!
//! # Example
//!
//! ## Local logger
//!
//! - Device side
//!
//! ```
//! extern crate stlog;
//!
//! use stlog::{info, warn, Log};
//!
//! struct Logger {
//!     // ..
//! #   _0: (),
//! }
//!
//! impl Log for Logger {
//!     // ..
//! #   type Error = ();
//! #
//! #   fn log(&mut self, _: u8) -> Result<(), ()> {
//! #       Ok(())
//! #   }
//! }
//!
//! fn main() {
//!     let mut logger = Logger {
//!         // ..
//! #       _0: (),
//!     };
//!
//!     info!(logger, "Hello, world!");
//!     warn!(logger, "The quick brown fox jumps over the lazy dog");
//! }
//! ```
//!
//! - Host side
//!
//! Assuming that the device is `log`ging through the `/dev/ttyUSB0` interface.
//!
//! ``` text
//! $ flash-and-run /path/to/device/binary
//!
//! $ cat /dev/ttyUSB0 | stcat -e /path/to/device/binary
//! Sept 22 13:00:00.000 INFO Hello, world!
//! Sept 22 13:00:00.001 WARN The quick brown fox jumps over the lazy dog
//! ```
//!
//! ## Global logger
//!
//! If the first argument is omitted from the logging macros then logging will be done through the
//! global logger. The global logger must be selected using the `global_logger` attribute *in the
//! top crate*.
//!
//! ``` ignore
//! use stlog::{info, GlobalLog};
//!
//! struct Logger;
//!
//! impl GlobalLog for Logger { .. }
//!
//! #[global_logger]
//! static LOGGER: Logger = Logger;
//!
//! fn main() {
//!     info!("Hello");
//! }
//!
//! #[interrupt]
//! fn SomeInterrupt() {
//!     info!("World");
//! }
//! ```
//!
//! # Troubleshooting
//!
//! ## Didn't pass `-Tstlog.x` to the linker
//!
//! Symptom: you'll get an error when linking the target application or when calling `stcat`.
//!
//! ``` text
//! $ cargo build
//! error: linking with `rust-lld` failed: exit code: 1
//!   |
//!   = note: "rust-lld" (..)
//!   = note: rust-lld: error: no memory region specified for section '.stlog.info'
//!           rust-lld: error: no memory region specified for section '.stlog.error'
//!
//! $ stcat -e /path/to/binary logfile
//! error: symbol `__stlog_error_start__` not found
//! ```
//!
//! Pass `-Tstlog.x` to the linker as explained in the requirements section.
//!
//! ## Didn't set a `global_logger`
//!
//! Symptom: you'll get an error when linking the program
//!
//! ``` text
//! $ cargo build
//! error: linking with `rust-lld` failed: exit code: 1
//!   |
//!   = note: "rust-lld" (..)
//!   = note: rust-lld: error: undefined symbol: stlog::GLOBAL_LOGGER
//! ```

#![no_std]
#![deny(warnings)]

extern crate stlog_macros;

pub use stlog_macros::global_logger;

/// A global version of the [`Log`](trait.Log) trait
///
/// This is very similar to [`Log`](trait.Log) except that the implementor must ensure that this
/// method is synchronized with other invocations of itself that could occur concurrently. Also,
/// note that there the return type is `()` and not `Result` so errors must be handled by the `log`
/// method.
pub trait GlobalLog: Sync {
    fn log(&self, address: u8);
}

/// A logger that encodes messages using a symbol table
///
/// # Contract
///
/// The implementation of the `log` method MUST send its argument as a single byte through some
/// interface.
pub trait Log {
    /// Error type of the log operation
    type Error;

    /// Sends the `address` of the symbol through some interface
    fn log(&mut self, address: u8) -> Result<(), Self::Error>;
}

/// Logs the given string literal at the ERROR log level
///
/// `$logger` must be an expression whose type implements the [`Log`](trait.Log.html) trait.
///
/// If `$logger` is omitted the global logger will be used.
#[macro_export]
macro_rules! error {
    ($logger:expr, $string:expr) => {{
        if $crate::max_level() as u8 >= $crate::Level::Error as u8 {
            #[export_name = $string]
            #[link_section = ".stlog.error"]
            static SYMBOL: u8 = 0;

            $crate::Log::log(&mut $logger, &SYMBOL as *const u8 as usize as u8)
        } else {
            Ok(())
        }
    }};

    ($string:expr) => {
        unsafe {
            if $crate::max_level() as u8 >= $crate::Level::Error as u8 {
                extern "Rust" {
                    #[link_name = "stlog::GLOBAL_LOGGER"]
                    static LOGGER: &'static $crate::GlobalLog;
                }

                #[export_name = $string]
                #[link_section = ".stlog.error"]
                static SYMBOL: u8 = 0;

                $crate::GlobalLog::log(LOGGER, &SYMBOL as *const u8 as usize as u8)
            }
        }
    };
}

/// Logs the given string literal at the WARNING log level
///
/// For more details see the [`error!`](macro.error.html) macro.
#[macro_export]
macro_rules! warn {
    ($logger:expr, $string:expr) => {{
        if $crate::max_level() as u8 >= $crate::Level::Warn as u8 {
            #[export_name = $string]
            #[link_section = ".stlog.warn"]
            static SYMBOL: u8 = 0;

            $crate::Log::log(&mut $logger, &SYMBOL as *const u8 as usize as u8)
        } else {
            Ok(())
        }
    }};

    ($string:expr) => {
        unsafe {
            if $crate::max_level() as u8 >= $crate::Level::Warn as u8 {
                extern "Rust" {
                    #[link_name = "stlog::GLOBAL_LOGGER"]
                    static LOGGER: &'static $crate::GlobalLog;
                }

                #[export_name = $string]
                #[link_section = ".stlog.warn"]
                static SYMBOL: u8 = 0;

                $crate::GlobalLog::log(LOGGER &SYMBOL as *const u8 as usize as u8)
            }
        }
    };
}

/// Logs the given string literal at the INFO log level
///
/// For more details see the [`error!`](macro.error.html) macro.
#[macro_export]
macro_rules! info {
    ($logger:expr, $string:expr) => {{
        if $crate::max_level() as u8 >= $crate::Level::Info as u8 {
            #[export_name = $string]
            #[link_section = ".stlog.info"]
            static SYMBOL: u8 = 0;

            $crate::Log::log(&mut $logger, &SYMBOL as *const u8 as usize as u8)
        } else {
            Ok(())
        }
    }};

    ($string:expr) => {
        unsafe {
            if $crate::max_level() as u8 >= $crate::Level::Info as u8 {
                extern "Rust" {
                    #[link_name = "stlog::GLOBAL_LOGGER"]
                    static LOGGER: &'static $crate::GlobalLog;
                }

                #[export_name = $string]
                #[link_section = ".stlog.info"]
                static SYMBOL: u8 = 0;

                $crate::GlobalLog::log(LOGGER, &SYMBOL as *const u8 as usize as u8)
            }
        }
    };
}

/// Logs the given string literal at the DEBUG log level
///
/// For more details see the [`error!`](macro.error.html) macro.
#[macro_export]
macro_rules! debug {
    ($log:expr, $string:expr) => {{
        if $crate::max_level() as u8 >= $crate::Level::Debug as u8 {
            #[export_name = $string]
            #[link_section = ".stlog.debug"]
            static SYMBOL: u8 = 0;

            $crate::Log::log(&mut $log, &SYMBOL as *const u8 as usize as u8)
        } else {
            Ok(())
        }
    }};

    ($string:expr) => {
        unsafe {
            if $crate::max_level() as u8 >= $crate::Level::Debug as u8 {
                extern "Rust" {
                    #[link_name = "stlog::GLOBAL_LOGGER"]
                    static LOGGER: &'static $crate::GlobalLog;
                }

                #[export_name = $string]
                #[link_section = ".stlog.debug"]
                static SYMBOL: u8 = 0;

                $crate::GlobalLog::log(LOGGER, &SYMBOL as *const u8 as usize as u8)
            }
        }
    };
}

/// Logs the given string literal at the TRACE log level
///
/// For more details see the [`error!`](macro.error.html) macro.
#[macro_export]
macro_rules! trace {
    ($logger:expr, $string:expr) => {{
        if $crate::max_level() as u8 >= $crate::Level::Trace as u8 {
            #[export_name = $string]
            #[link_section = ".stlog.trace"]
            static SYMBOL: u8 = 0;

            $crate::Log::log(&mut $logger, &SYMBOL as *const u8 as usize as u8)
        } else {
            Ok(())
        }
    }};

    ($string:expr) => {
        unsafe {
            if $crate::max_level() as u8 >= $crate::Level::Trace as u8 {
                extern "Rust" {
                    #[link_name = "stlog::GLOBAL_LOGGER"]
                    static LOGGER: &'static $crate::GlobalLog;
                }

                #[export_name = $string]
                #[link_section = ".stlog.trace"]
                static SYMBOL: u8 = 0;

                $crate::GlobalLog::log(LOGGER, &SYMBOL as *const u8 as usize as u8)
            }
        }
    };
}

#[doc(hidden)]
pub enum Level {
    Off = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

#[doc(hidden)]
#[inline(always)]
pub fn max_level() -> Level {
    match () {
        #[cfg(debug_assertions)]
        () => {
            #[cfg(feature = "max-level-off")]
            return Level::Off;

            #[cfg(feature = "max-level-error")]
            return Level::Error;

            #[cfg(feature = "max-level-warn")]
            return Level::Warn;

            #[cfg(feature = "max-level-info")]
            return Level::Info;

            #[cfg(feature = "max-level-debug")]
            return Level::Debug;

            #[cfg(feature = "max-level-trace")]
            return Level::Trace;

            #[allow(unreachable_code)]
            Level::Debug
        }
        #[cfg(not(debug_assertions))]
        () => {
            #[cfg(feature = "release-max-level-off")]
            return Level::Off;

            #[cfg(feature = "release-max-level-error")]
            return Level::Error;

            #[cfg(feature = "release-max-level-warn")]
            return Level::Warn;

            #[cfg(feature = "release-max-level-info")]
            return Level::Info;

            #[cfg(feature = "release-max-level-debug")]
            return Level::Debug;

            #[cfg(feature = "release-max-level-trace")]
            return Level::Trace;

            #[allow(unreachable_code)]
            Level::Info
        }
    }
}
