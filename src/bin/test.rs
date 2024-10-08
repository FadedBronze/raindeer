use std::process::ExitCode;

use raindeer::{color::RDColor, path_builder::{RDPath, RDStroke}, Raindeer};

fn main() -> ExitCode {
    let mut renderer = Raindeer::new();

    let path = RDPath::new()
        .to(0.0, 0.0)
        .line(0.1, 0.0)
        .line(0.1, 0.1)
        .line(0.0, 0.1)
        .close()
        .stroke(RDStroke::default())
        .fill(RDColor::RED);

    let mut square = path.make_object();

    square.rotate_deg(45.0);

    renderer.add_object(square);

    loop {
        if let Err(exitcode) = renderer.run() {
            break exitcode;
        }
    }
}
