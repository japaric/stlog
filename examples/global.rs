extern crate stlog;

use stlog::{error, global_logger, info, GlobalLog};

struct Logger;

impl GlobalLog for Logger {
    fn log(&self, _: u8) {}
}

#[global_logger]
static LOGGER: Logger = Logger;

fn main() {
    info!("Hello!");
    error!("Bye!");
}
