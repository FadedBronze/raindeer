#[derive(Clone)]
pub struct RDColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Into<[u8; 4]> for RDColor {
    fn into(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl RDColor {
    pub const RED: RDColor = RDColor::new(255, 0, 0, 255);
    pub const GREEN: RDColor = RDColor::new(0, 255, 0, 255);
    pub const BLUE: RDColor = RDColor::new(0, 0, 255, 255);

    pub const YELLOW: RDColor = RDColor::new(255, 255, 0, 255);
    pub const MAGENTA: RDColor = RDColor::new(255, 0, 255, 255);
    pub const CYAN: RDColor = RDColor::new(0, 255, 255, 255);

    pub const BLACK: RDColor = RDColor::new(0, 0, 0, 255);
    pub const WHITE: RDColor = RDColor::new(255, 255, 255, 255);
    pub const TRANSPARENT: RDColor = RDColor::new(0, 0, 0, 0);

    pub const fn new(
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) -> Self {
        Self { r, g, b, a }
    }
}
