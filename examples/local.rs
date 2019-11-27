#![cfg_attr(feature = "spanned", feature(proc_macro_hygiene))]

#[cfg(feature = "spanned")]
use stlog::spanned::{error, info, trace};
use stlog::Log;
#[cfg(not(feature = "spanned"))]
use stlog::{error, info};

struct Logger;

impl Log for Logger {
    type Error = ();

    fn log(&mut self, byte: u8) -> Result<(), ()> {
        println!("{}", byte);
        Ok(())
    }
}

fn main() {
    let mut logger = Logger;

    info!(logger, "Hello!").unwrap();
    #[cfg(feature = "spanned")]
    trace!(logger, "Hello!").unwrap();
    error!(logger, "Bye!").unwrap();
}
