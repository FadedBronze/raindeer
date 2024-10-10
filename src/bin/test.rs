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

    let square = path.to_node();

    renderer.scene.add_root(square);

    loop {
        if let Err(exitcode) = renderer.run() {
            break exitcode;
        }
    }
}
