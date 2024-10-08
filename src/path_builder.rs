use std::f32::consts::PI;

use cgmath::{Matrix4, Rad, Vector2, Vector3, Zero};

use crate::{color::RDColor, triangulate::triangulate, RDObjectGFXData, Vertex};

pub struct RDNode {
    position: Vector2<f32>,
    texture_position: Vector2<f32>,
}

pub struct RDPrimative {
    texture: u32,
    color: RDColor,
    nodes: Vec<RDNode>,
    indicies: Vec<u32>,
    pub position: Vector2<f32>,
    pub scale: Vector2<f32>,
    pub rotation: f32,
}

pub struct RDObject {
    pub primatives: Vec<RDPrimative>,
}

impl RDObject {
    pub fn rotate_deg(&mut self, deg: f32) {
        for primative in self.primatives.iter_mut() {
            primative.rotation += deg * PI / 180.0;
        }
    }

    pub fn translate(&mut self, translation: Vector2<f32>) {
        for primative in self.primatives.iter_mut() {
            primative.position += translation;
        }
    }

    pub fn scale(&mut self, scale: Vector2<f32>) {
        for primative in self.primatives.iter_mut() {
            primative.scale += scale;
        }
    }

    pub fn gfx_storage_output(&self) -> Vec<RDObjectGFXData> {
        let mut storage = vec![];

        for primative in self.primatives.iter() {
            let storg = primative.gfx_storage_output();
            storage.push(storg);
        }

        storage 
    }

    pub fn gfx_vertex_output(&self, id: u32) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices = vec![];
        let mut indices = vec![];

        for primative in self.primatives.iter() {
            let (mut verts, inds) = primative.gfx_vertex_output(id);

            for index in inds.iter() {
                indices.push(index + vertices.len() as u32);
            }
            
            vertices.append(&mut verts);
        }

        (vertices, indices)
    }
}

impl RDPrimative {
    //implicitly relies on ordering
    pub fn gfx_vertex_output(&self, id: u32) -> (Vec<Vertex>, Vec<u32>) {
        let mut verticies = vec![];

        for node in self.nodes.iter() {
            verticies.push(Vertex {
                position: node.position.into(),
                texture_position: node.texture_position.into(),
                id,
            });
        }

        (verticies, self.indicies.clone())
    }

    pub fn gfx_storage_output(&self) -> RDObjectGFXData {
        let transform = 
            Matrix4::from_translation(Vector3::new(self.position.x, self.position.y, 0.0)) *
            Matrix4::from_angle_z(Rad(self.rotation)) *
            Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, 0.0);

        RDObjectGFXData {
            texture: self.texture,
            transform: transform.into(),
            color: self.color.clone().into(),
        }
    }
}

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
                    texture_position: Vector2::zero(),
                });
            }
        }

        RDObject {
            primatives: vec![RDPrimative {
                texture: 0,
                color: self.color.clone(),
                nodes,
                indicies,
                position: Vector2::zero(),
                scale: Vector2::new(1.0, 1.0),
                rotation: 0.0,
            }]
        }
    }
}
