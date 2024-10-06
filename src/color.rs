#[derive(Clone)]
pub struct RDColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Into<[f32; 4]> for RDColor {
    fn into(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl RDColor {
    pub const RED: RDColor = RDColor::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: RDColor = RDColor::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: RDColor = RDColor::new(0.0, 0.0, 1.0, 1.0);

    pub const YELLOW: RDColor = RDColor::new(1.0, 1.0, 0.0, 1.0);
    pub const MAGENTA: RDColor = RDColor::new(1.0, 0.0, 1.0, 1.0);
    pub const CYAN: RDColor = RDColor::new(0.0, 1.0, 1.0, 1.0);
    
    pub const BLACK: RDColor = RDColor::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: RDColor = RDColor::new(1.0, 1.0, 1.0, 1.0);
    pub const TRANSPARENT: RDColor = RDColor::new(0.0, 0.0, 0.0, 0.0);

    pub const fn new(
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) -> Self {
        Self { r, g, b, a }
    }
}
