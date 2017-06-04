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
//! The device program must be stored in ELF format.
//!
//! The device linker script must allocate a `.stlog` section in the output ELF
//! file as described below:
//!
//! ``` text
//! SECTIONS
//! {
//!     /* .. */
//!
//!     .stlog 0 (INFO) : {
//!         _sstlog_trace = .;
//!         *(.stlog.trace*);
//!         _estlog_trace = .;
//!
//!         _sstlog_debug = .;
//!         *(.stlog.debug*);
//!         _estlog_debug = .;
//!
//!         _sstlog_info = .;
//!         *(.stlog.info*);
//!         _estlog_info = .;
//!
//!         _sstlog_warn = .;
//!         *(.stlog.warn*);
//!         _estlog_warn = .;
//!
//!         _sstlog_error = .;
//!         *(.stlog.error*);
//!         _estlog_error = .;
//!     }
//!
//!     /* .. */
//! }
//! ```
//!
//! # Example
//!
//! - Device side
//!
//! ``` ignore
//! #[macro_use]
//! extern crate stlog;
//!
//! struct Logger { .. }
//!
//! impl<'a> stlog::Logger for &'a Logger { .. }
//!
//! fn main() {
//!     let logger = Logger { .. };
//!
//!     info!(&logger, "Hello, world!");
//!     warn!(&logger, "The quick brown fox jumps over the lazy dog.");
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

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

/// A logger compatible with the `stlog` logging framework
///
/// # Contract
///
/// The `log` implementation must simply send its argument as a byte through
/// some interface.
pub trait Logger {
    /// Error type of the log operation
    type Error;

    /// Logs a string stored in debuginfo with address `addr`
    fn log(self, addr: u8) -> Result<(), Self::Error>;
}

#[macro_export]
macro_rules! trace {
    ($logger:expr, $string:expr) => {
        {
            #[link_section = ".stlog.trace"]
            #[export_name = $string]
            static SYMBOL: bool = false;

            $crate::Logger::log($logger, &SYMBOL as *const _ as usize as u8)
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($logger:expr, $string:expr) => {
        {
            #[link_section = ".stlog.debug"]
            #[export_name = $string]
            static SYMBOL: bool = false;

            $crate::Logger::log($logger, &SYMBOL as *const _ as usize as u8)
        }
    };
}

#[macro_export]
macro_rules! info {
    ($logger:expr, $string:expr) => {
        {
            #[link_section = ".stlog.info"]
            #[export_name = $string]
            static SYMBOL: bool = false;

            $crate::Logger::log($logger, &SYMBOL as *const _ as usize as u8)
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($logger:expr, $string:expr) => {
        {
            #[link_section = ".stlog.warn"]
            #[export_name = $string]
            static SYMBOL: bool = false;

            $crate::Logger::log($logger, &SYMBOL as *const _ as usize as u8)
        }
    };
}

#[macro_export]
macro_rules! error {
    ($logger:expr, $string:expr) => {
        {
            #[link_section = ".stlog.error"]
            #[export_name = $string]
            static SYMBOL: bool = false;

            $crate::Logger::log($logger, &SYMBOL as *const _ as usize as u8)
        }
    };
}
