#![cfg_attr(feature = "spanned", feature(proc_macro_hygiene))]

#[cfg(feature = "spanned")]
use stlog::spanned::{error, info, trace};
#[cfg(not(feature = "spanned"))]
use stlog::{error, info};
use stlog::{global_logger, GlobalLog};

struct Logger;

impl GlobalLog for Logger {
    fn log(&self, _: u8) {}
}

#[global_logger]
static LOGGER: Logger = Logger;

fn main() {
    info!("Hello!");
    #[cfg(feature = "spanned")]
    trace!("Hello!");
    error!("Bye!");
}
