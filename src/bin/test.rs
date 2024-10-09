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
        .stroke(RDStroke {
            weight: 0.09,
            color: RDColor::BLACK,
        })
        .fill(RDColor::RED);

    //square.rotate_deg(45.0);
    //square.scale(Vector2::new(1, 1)); 

    loop {
        if let Err(exitcode) = renderer.run() {
            break exitcode;
        }
    }
}
