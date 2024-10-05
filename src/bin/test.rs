use std::process::ExitCode;

use raindeer::Raindeer;

fn main() -> ExitCode {
    let mut gui = Raindeer::new();

    loop {
        if let Err(exitcode) = gui.run() {
            break exitcode;
        }
    }
}
