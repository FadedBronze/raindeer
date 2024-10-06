use nalgebra::{Matrix3, Vector2};
use crate::{color::RDColor, triangulate::triangulate, RDObjectGFXData, Vertex};

pub struct RDNode {
    position: Vector2<f32>,
    texture_position: Vector2<f32>,
}

pub struct RDObject {
    texture: u32,
    color: RDColor,
    nodes: Vec<RDNode>,
    indicies: Vec<u32>,
    transform: Matrix3<f32>,
}

impl RDObject {
    //implicitly relies on ordering
    pub fn gfx_output(&self, id: u32) -> (Vec<Vertex>, Vec<u32>, RDObjectGFXData) {
        let mut verticies = vec![];

        for node in self.nodes.iter() {
            let new_position = self.transform.transform_vector(&node.position);

            verticies.push(Vertex {
                position: new_position.into(),
                texture_position: node.texture_position.into(),
                id,
            });
        }

        (verticies, self.indicies.clone(), RDObjectGFXData {
            texture: self.texture,
            color: self.color.clone().into(),
        })
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
                    position: *point,
                    texture_position: Vector2::zeros(),
                });
            }
        }

        RDObject {
            texture: 0,
            color: self.color.clone(),
            transform: Matrix3::identity(),
            nodes,
            indicies,
        }
    }
}
