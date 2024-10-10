use cgmath::{Matrix4, Rad, SquareMatrix, Vector2, Vector3, Zero};
use crate::{RDStorage, RDVertex};

pub struct RDTransform {
    pub position: Vector2<f32>,
    pub rotation: f32,
    pub scale: Vector2<f32>,
}

impl RDTransform {
    pub fn to_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_angle_z(Rad(self.rotation)) * 
            Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, 0.0) *
            Matrix4::from_translation(Vector3::new(self.position.x, self.position.y, 0.0))
    }
}

impl Default for RDTransform {
    fn default() -> Self {
        Self {
            position: Vector2::zero(),
            rotation: 0.0,
            scale: Vector2::new(1.0, 1.0)
        }
    }
}

#[derive(Debug, Clone)]
pub struct VAO {
    pub vertices: Vec<RDVertex>,
    pub indicies: Vec<u32>
}

impl VAO {
    pub(crate) fn new() -> Self {
        Self {
            vertices: vec![],
            indicies: vec![],
        }
    }
    pub(crate) fn merge(&mut self, other: VAO) {
        let VAO { mut vertices, indicies, .. } = other;

        for index in indicies {
            self.indicies.push(index + vertices.len() as u32)
        }

        self.vertices.append(&mut vertices);
    }
}

pub struct RDMesh {
    //placeholder -- change to whatever texture atlas abstraction becomes
    pub texture: u32,
    pub vao: VAO,
}

impl RDMesh {
    fn combine(&mut self, mut other: RDMesh) {
        debug_assert!(self.texture == other.texture);

        let RDMesh { vao: VAO { vertices, indicies }, .. } = &mut other;

        for index in indicies {
            self.vao.indicies.push(*index + self.vao.vertices.len() as u32);
        }

        self.vao.vertices.append(vertices);
    }
}

pub struct RDNode {
    pub children: Vec<RDNode>,
    pub transform: RDTransform,
    pub mesh: Option<RDMesh>
}

pub struct RDScene {
    root: RDNode,
    pub(crate) index_count: u32,
    pub(crate) vertex_cache: bool,
}

impl RDScene {
    pub fn new() -> Self {
        Self {
            index_count: 0,
            vertex_cache: false,
            root: RDNode {
                mesh: None,
                children: vec![],
                transform: RDTransform {
                    position: Vector2::zero(),
                    rotation: 0.0,
                    scale: Vector2::new(1.0, 1.0),
                }
            }
        }
    } 

    pub fn add_root(&mut self, node: RDNode) {
        self.root.children.push(node);
        self.vertex_cache = false;
    }
    pub fn add(&mut self, parent: &mut RDNode, node: RDNode) {
        parent.children.push(node);
        self.vertex_cache = false;
    }

    fn recurse_output_gfx_vertex_index(node: &RDNode, vao: &mut VAO) {
        if let Some(mesh) = &node.mesh {
            vao.merge(mesh.vao.clone());
        }

        for child in node.children.iter() {
            RDScene::recurse_output_gfx_vertex_index(child, vao);
        }
    }
    pub fn output_gfx_vao(&self) -> VAO {
        let mut vao = VAO::new();
        RDScene::recurse_output_gfx_vertex_index(&self.root, &mut vao);
        vao
    }

    fn recurse_output_gfx_storage(parent_matrix: Matrix4<f32>, node: &RDNode, buffer: &mut Vec<RDStorage>) {
        let matrix = parent_matrix * node.transform.to_matrix();

        if let Some(mesh) = &node.mesh {
            let gfx_storage = RDStorage {
                texture: mesh.texture,
                transform: matrix.into(),
            };

            buffer.push(gfx_storage);
        }

        for child in node.children.iter() {
            RDScene::recurse_output_gfx_storage(matrix, child, buffer);
        }
    }
    pub fn output_gfx_storage(&self) -> Vec<RDStorage> {
        let mut output = vec![];
        RDScene::recurse_output_gfx_storage(Matrix4::identity(), &self.root, &mut output);
        output
    }
}
