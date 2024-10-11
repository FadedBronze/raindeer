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
            weight: 0.04,
            color: RDColor::BLACK,
        })
        .fill(RDColor::RED);
    
    let path2 = RDPath::new()
        .to(0.0, 0.0)
        .line(0.1, 0.0)
        .line(0.1, 0.1)
        .line(0.0, 0.1)
        .close()
        .stroke(RDStroke {
            weight: 0.07,
            color: RDColor::GREEN,
        })
        .fill(RDColor::BLUE);

    let mut square = path.to_node();
    let square2 = path2.to_node();

    square.transform.position.x += 0.2;
    square.transform.scale.x = 2.0;
    square.transform.scale.y = 2.0;

    renderer.scene.add_root(square);
    renderer.scene.add_root(square2);

    loop {
        if let Err(exitcode) = renderer.run() {
            break exitcode;
        }
    }
}
