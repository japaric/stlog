//! Lightweight logging framework for resource constrained devices
//!
//! ![stlog running on a Cortex-M microcontroller][img]
//!
//! [img]: https://i.imgur.com/9K3SrI2.png
//!
//! # Features
//!
//! - `O(1)` runtime. Logging strings of arbitrary size takes a constant number
//!   of instructions.
//!
//! - `O(0)` memory usage. The logged strings are NOT stored in the device
//!   memory.
//!
//! - Supports different logging levels: trace, debug, info, warn and error.
//!   Each level can be individually disabled at compile time.
//!
//! - Provides a "global" logging mode
//!
//! # Non-features
//!
//! - `printf` style or any other kind of formatting
//!
//! # Limitations
//!
//! - The current implementation only supports 256 different log strings. This
//!   restriction will be lifted in the future.
//!
//! # Requirements
//!
//! The application must be linked using the `stlog.x` linker script provided by
//! this crate. The easiest way to do this is to use the `-C link-arg` rustc
//! flag:
//!
//! ``` text
//! $ cat .cargo/config
//! [target.thumbv7m-none-eabi]
//! rustflags = [
//!     "-C", "link-arg=-Tstlog.x",
//!     # ..
//! ]
//! ```
//!
//! # Example
//!
//! ## Minimal
//!
//! - Device side
//!
//! ``` rust,ignore,no_run
//! #[macro_use]
//! extern crate stlog;
//!
//! struct Logger { .. }
//!
//! impl stlog::Logger for Logger { .. }
//!
//! fn main() {
//!     let logger = Logger { .. };
//!
//!     info!(logger, "Hello, world!");
//!     warn!(logger, "The quick brown fox jumps over the lazy dog.");
//! }
//! ```
//!
//! - Host side
//!
//! ``` text
//! $ # flash and execute the device program
//! $ arm-none-eabi-gdb /path/to/device/binary &
//!
//! $ # stcat is required to decode strings logged via `stlog`
//! $ cargo install stcat --vers 0.1.0
//!
//! $ cat /dev/ttyUSB0 | stcat -e /path/to/device/binary
//! INFO Hello, world!
//! WARN The quick brown fox jumps over the lazy dog.
//! ```
//!
//! Assuming that the device is logging the strings through the `/dev/ttyUSB0`
//! interface. The device binary must *not* be stripped.
//!
//! ## Global logging
//!
//! If the first argument is omitted from the logging macros logging will be
//! performed using the global logger. The global logger must be selected using
//! the [`set_global_logger!`](macro.set_global_logger.html) macro.
//!
//! ``` rust,ignore,no_run
//! // Global logger
//! struct Logger;
//!
//! impl stlog::Logger for Logger {
//!     // required: the error type must be `!`
//!     type Error = !;
//!
//!     fn log(&self, addr: u8) -> Result<(), !> {
//!         interrupt::free(|cs| {
//!             // ..
//!         });
//!     }
//! }
//!
//! set_global_logger!(Logger);
//!
//! fn main() {
//!     info!("Hello");
//! }
//!
//! fn isr() {
//!     info!("World");
//! }
//! ```
//!
//! ## Disabling certain log levels
//!
//! The `Logger` trait includes methods to disable certain log levels. By
//! default all the log levels are enabled.
//!
//! ``` rust,ignore,no_run
//! // Global logger
//! struct Logger;
//!
//! impl stlog::Logger for Logger {
//!     type Error = !;
//!
//!     fn log(&self, addr: u8) -> Result<(), !> { .. }
//!
//!     // disable `trace!` and `debug!`
//!     fn trace_enabled(&self) -> bool { false }
//!     fn debug_enabled(&self) -> bool { false }
//! }
//! ```
//!
//! # Troubleshooting
//!
//! ## Didn't pass `-Tstlog.x` to the linker
//!
//! ### Symptom
//!
//! ``` text
//! $ cat /dev/ttyUSB0 | stcat -e /path/to/device/binary
//! Error: _sstlog_trace symbol not found
//! ```
//!
//! ### Solution
//!
//! Pass `-Tstlog.x` to the linker using the method shown in the
//! [requirements](index.html#requirements) section.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

/// A logger compatible with the `stlog` logging framework
///
/// # Contract
///
/// The `log` implementation must simply send its argument as a single byte
/// through some interface.
pub trait Logger {
    /// Error type of the log operation
    type Error;

    /// Logs a string stored in the symbol table with address `addr`
    fn log(&self, addr: u8) -> Result<(), Self::Error>;

    /// Whether `trace!` is enabled or not
    fn trace_enabled(&self) -> bool {
        true
    }

    /// Whether `debug!` is enabled or not
    fn debug_enabled(&self) -> bool {
        true
    }

    /// Whether `info!` is enabled or not
    fn info_enabled(&self) -> bool {
        true
    }

    /// Whether `warn!` is enabled or not
    fn warn_enabled(&self) -> bool {
        true
    }

    /// Whether `error!` is enabled or not
    fn error_enabled(&self) -> bool {
        true
    }
}

/// Sets the global logger
///
/// `$logger` must be an expression whose type implements the
/// [`Logger`](trait.Logger.html) and `Sync` traits.
#[macro_export]
macro_rules! set_global_logger {
    ($logger:expr) => {
        #[used]
        #[export_name = "STLOGGER"]
        static _STLOGGER: &'static ($crate::Logger<Error = !> + Sync) = &$logger;
    }
}

/// Logs `$string` at the TRACE log level
///
/// `$logger` is the logger through which the string will be logged. If omitted
/// the global logger will be used.
#[macro_export]
macro_rules! trace {
    ($logger:expr, $string:expr) => {
        if $crate::Logger::trace_enabled(&$logger) {
            #[link_section = ".stlog.trace"]
            #[export_name = $string]
            static SYMBOL: bool = false;

            $crate::Logger::log(&$logger, &SYMBOL as *const _ as usize as u8)
        } else {
            Ok(())
        }
    };
    ($string:expr) => {
        unsafe {
            #[allow(improper_ctypes)]
            extern {
                static STLOGGER: &'static $crate::Logger<Error = !>;
            }

            if $crate::Logger::trace_enabled(STLOGGER) {
                #[link_section = ".stlog.trace"]
                #[export_name = $string]
                static SYMBOL: bool = false;

                $crate::Logger::log(STLOGGER,
                                    &SYMBOL as *const _ as usize as u8).unwrap()
            }
        }
    };
}

/// Logs `$string` at the DEBUG log level
///
/// `$logger` is the logger through which the string will be logged. If omitted
/// the global logger will be used.
#[macro_export]
macro_rules! debug {
    ($logger:expr, $string:expr) => {
        if $crate::Logger::debug_enabled(&$logger) {
            #[link_section = ".stlog.debug"]
            #[export_name = $string]
            static SYMBOL: bool = false;

            $crate::Logger::log(&$logger, &SYMBOL as *const _ as usize as u8)
        } else {
            Ok(())
        }
    };
    ($string:expr) => {
        unsafe {
            #[allow(improper_ctypes)]
            extern {
                static STLOGGER: &'static $crate::Logger<Error = !>;
            }

            if $crate::Logger::debug_enabled(STLOGGER) {
                #[link_section = ".stlog.debug"]
                #[export_name = $string]
                static SYMBOL: bool = false;

                $crate::Logger::log(STLOGGER,
                                    &SYMBOL as *const _ as usize as u8).unwrap()
            }
        }
    };
}

/// Logs `$string` at the INFO log level
///
/// `$logger` is the logger through which the string will be logged. If omitted
/// the global logger will be used.
#[macro_export]
macro_rules! info {
    ($logger:expr, $string:expr) => {
        if $crate::Logger::info_enabled(&$logger) {
            #[link_section = ".stlog.info"]
            #[export_name = $string]
            static SYMBOL: bool = false;

            $crate::Logger::log(&$logger, &SYMBOL as *const _ as usize as u8)
        } else {
            Ok(())
        }
    };
    ($string:expr) => {
        unsafe {
            #[allow(improper_ctypes)]
            extern {
                static STLOGGER: &'static $crate::Logger<Error = !>;
            }

            if $crate::Logger::info_enabled(STLOGGER) {
                #[link_section = ".stlog.info"]
                #[export_name = $string]
                static SYMBOL: bool = false;

                $crate::Logger::log(STLOGGER,
                                    &SYMBOL as *const _ as usize as u8).unwrap()
            }
        }
    };
}

/// Logs `$string` at the WARN log level
///
/// `$logger` is the logger through which the string will be logged. If omitted
/// the global logger will be used.
#[macro_export]
macro_rules! warn {
    ($logger:expr, $string:expr) => {
        if $crate::Logger::warn_enabled(&$logger) {
            #[link_section = ".stlog.warn"]
            #[export_name = $string]
            static SYMBOL: bool = false;

            $crate::Logger::log(&$logger, &SYMBOL as *const _ as usize as u8)
        } else {
            Ok(())
        }
    };
    ($string:expr) => {
        unsafe {
            #[allow(improper_ctypes)]
            extern {
                static STLOGGER: &'static $crate::Logger<Error = !>;
            }

            if $crate::Logger::warn_enabled(STLOGGER) {
                #[link_section = ".stlog.warn"]
                #[export_name = $string]
                static SYMBOL: bool = false;

                $crate::Logger::log(STLOGGER,
                                    &SYMBOL as *const _ as usize as u8).unwrap()
            }
        }
    };
}

/// Logs `$string` at the ERROR log level
///
/// `$logger` is the logger through which the string will be logged. If omitted
/// the global logger will be used.
#[macro_export]
macro_rules! error {
    ($logger:expr, $string:expr) => {
        if $crate::Logger::error_enabled(&$logger) {
            #[link_section = ".stlog.error"]
            #[export_name = $string]
            static SYMBOL: bool = false;

            $crate::Logger::log(&$logger, &SYMBOL as *const _ as usize as u8)
        } else {
            Ok(())
        }
    };
    ($string:expr) => {
        unsafe {
            #[allow(improper_ctypes)]
            extern {
                static STLOGGER: &'static $crate::Logger<Error = !>;
            }

            if $crate::Logger::error_enabled(STLOGGER) {
                #[link_section = ".stlog.error"]
                #[export_name = $string]
                static SYMBOL: bool = false;

                $crate::Logger::log(STLOGGER,
                                    &SYMBOL as *const _ as usize as u8).unwrap()
            }
        }
    };
}
