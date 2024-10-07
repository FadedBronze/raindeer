use std::{f32::consts::PI, process::ExitCode};

use raindeer::{color::RDColor, path_builder::RDPath, Raindeer};

fn main() -> ExitCode {
    let mut renderer = Raindeer::new();

    let path = RDPath::new()
        .to(0.0, 0.0)
        .line(0.1, 0.0)
        .line(0.1, 0.1)
        .line(0.0, 0.1)
        .close()
        .fill(RDColor::RED);

    let mut square = path.make_object();

    square.rotation = PI / 4.0;

    renderer.add_object(square);

    loop {
        if let Err(exitcode) = renderer.run() {
            break exitcode;
        }
    }
}
