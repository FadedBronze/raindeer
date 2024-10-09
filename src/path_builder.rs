use cgmath::Vector2;
use crate::color::RDColor;

//cap style
//stroke style (in, out, middle)
pub struct RDStroke {
    pub weight: f32,
    pub color: RDColor,
}

impl Default for RDStroke {
    fn default() -> Self {
        Self {
            weight: 10.0,
            color: RDColor::BLACK,
        }
    }
}

pub struct RDPath {
    pub points: Vec<Vec<Vector2<f32>>>,
    pub color: RDColor,
    pub stroke: RDStroke,
    pub closed: bool,
}

impl RDPath {
    pub fn new() -> Self {
        Self {
            closed: false,
            points: vec![],
            color: RDColor::WHITE,
            stroke: RDStroke::default(),
        }
    }

    pub fn to(mut self, x: f32, y: f32) -> Self {
        self.points.push(vec![Vector2::new(x, y)]);
        self
    }
    
    pub fn line(mut self, x: f32, y: f32) -> Self {
        let last = self.points.len()-1;

        self.points[last].push(Vector2::new(x, y));
        self 
    }

    pub fn close(mut self) -> Self {
        self.closed = true;
        self
    }
    
    pub fn fill(mut self, color: RDColor) -> Self {
        if !self.closed {
            panic!("cannot fill open path");
        }
        self.color = color;
        self
    }
    
    pub fn stroke(mut self, config: RDStroke) -> Self {
        self.stroke = config;
        self
    }
}
