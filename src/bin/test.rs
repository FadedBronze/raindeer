use std::process::ExitCode;

use raindeer::Raindeer;

fn main() -> ExitCode {
    let mut renderer = Raindeer::new();

    loop {
        if let Err(exitcode) = renderer.run() {
            break exitcode;
        }
    }
}
