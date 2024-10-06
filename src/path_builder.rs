use nalgebra::{Matrix3, Vector2};
use crate::{color::RDColor, triangulate::triangulate, Vertex};

pub struct RDNode {
    color: RDColor,
    position: Vector2<f32>,
    texture_position: Vector2<f32>,
    texture: u32,
}

pub struct RDObject {
    nodes: Vec<RDNode>,
    indicies: Vec<u32>,
    transform: Matrix3<f32>,
}

impl RDObject {
    pub fn gfx_output(&self) -> (Vec<Vertex>, Vec<u32>) {
        let mut verticies = vec![];

        for node in self.nodes.iter() {
            let new_position = self.transform.transform_vector(&node.position);

            verticies.push(Vertex {
                position: new_position.into(),
                texture_position: node.texture_position.into(),
                texture: node.texture,
                color: node.color.clone().into(),
            });
        }

        (verticies, self.indicies.clone())
    }
}

//cap style
//stroke style (in, out, middle)
pub struct RDPath {
    pub points: Vec<Vec<Vector2<f32>>>,
    pub color: RDColor,
    pub stroke: RDColor,
    pub stroke_weight: f32,
    pub closed: bool,
}

impl RDPath {
    pub fn new() -> Self {
        Self {
            closed: false,
            points: vec![],
            color: RDColor::WHITE,
            stroke: RDColor::BLACK,
            stroke_weight: 0.0,
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
    
    pub fn stroke(mut self, color: RDColor) -> Self {
        self.stroke = color;
        self
    }

    pub fn make_object(&self) -> RDObject {
        let mut nodes = vec![];
        let mut indicies = vec![];

        for path in self.points.clone().iter_mut() {
            let path_indicies = triangulate(path);

            for i in 0..path_indicies.len() {
                indicies.push(path_indicies[i] + nodes.len() as u32);
            }

            for point in path {
                nodes.push(RDNode {
                    color: self.color.clone(),
                    position: *point,
                    texture_position: Vector2::zeros(),
                    texture: 0,
                });
            }
        }

        RDObject {
            transform: Matrix3::identity(),
            nodes,
            indicies,
        }
    }
}
