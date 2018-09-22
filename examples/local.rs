extern crate stlog;

use stlog::{error, info, Log};

struct Logger;

impl Log for Logger {
    type Error = ();

    fn log(&mut self, _: u8) -> Result<(), ()> {
        Ok(())
    }
}

fn main() {
    let mut logger = Logger;

    info!(logger, "Hello!");
    error!(logger, "Bye!");
}
