use cgmath::Vector2;
use crate::{color::RDColor, scene::{RDMesh, RDNode, RDTransform, VAO}, triangulate::{triangulate, triangulate_stroke}, RDVertex};

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

struct ContinousPath {
    points: Vec<Vector2<f32>>,
    closed: bool,
}

pub struct RDPath {
    continous_paths: Vec<ContinousPath>,
    pub color: RDColor,
    pub stroke: RDStroke,
}

impl RDPath {
    pub fn new() -> Self {
        Self {
            continous_paths: vec![],
            color: RDColor::WHITE,
            stroke: RDStroke::default(),
        }
    }

    pub fn to(mut self, x: f32, y: f32) -> Self {
        self.continous_paths.push(ContinousPath {
            points: vec![Vector2::new(x, y)],
            closed: false,
        });
        self
    }
    
    pub fn line(mut self, x: f32, y: f32) -> Self {
        let last = self.continous_paths.len()-1;

        debug_assert!(!self.continous_paths[last].closed);

        self.continous_paths[last].points.push(Vector2::new(x, y));
        self 
    }

    pub fn close(mut self) -> Self {
        let last = self.continous_paths.len()-1;

        debug_assert!(!self.continous_paths[last].closed);

        self.continous_paths[last].closed = true;
        self
    }
    
    pub fn fill(mut self, color: RDColor) -> Self {
        self.color = color;
        self
    }
    
    pub fn stroke(mut self, config: RDStroke) -> Self {
        self.stroke = config;
        self
    }

    pub fn to_node(&self) -> RDNode {
        let mut fill_vao = VAO::new();
        let mut stroke_vao = VAO::new();

        for path in self.continous_paths.iter() {
            if path.closed {
                let (points, indicies) = triangulate_stroke(&path.points, &self.stroke);
    
                let mut vertices = vec![];

                for point in points.iter() {
                    vertices.push(RDVertex {
                        id: 0,
                        position: point.clone().into(),
                        texture_position: [0.0, 0.0], 
                        color: self.stroke.color.clone().into(), 
                    })
                }

                let vao = VAO { vertices, indicies };
                stroke_vao.merge(vao);
            }
                
            let indicies = triangulate(&path.points);
    
            let mut vertices = vec![];

            for point in path.points.iter() {
                vertices.push(RDVertex {
                    id: 0,
                    position: point.clone().into(),
                    texture_position: [0.0, 0.0], 
                    color: self.color.clone().into(), 
                })
            }

            let vao = VAO { vertices, indicies };
            fill_vao.merge(vao);
        }

        RDNode {
            transform: RDTransform::default(),
            mesh: None,
            children: vec![
                RDNode {
                    mesh: Some(RDMesh {
                        texture: 0,
                        vao: fill_vao,
                    }),
                    children: vec![],
                    transform: RDTransform::default(),
                },
                RDNode {
                    mesh: Some(RDMesh {
                        texture: 0,
                        vao: stroke_vao,
                    }),
                    children: vec![],
                    transform: RDTransform::default(),
                }
            ]
        }
    }
}
