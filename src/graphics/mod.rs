

pub mod material;
pub mod mesh;

use crate::graphics::mesh::Mesh;
use crate::graphics::material::Material;

use std::borrow::Borrow;

pub struct GraphicsInterface {
    material_pool: Vec<Material>,
    mesh_pool: Vec<Mesh>,

}

impl GraphicsInterface {
    pub fn new() -> GraphicsInterface {
        GraphicsInterface {
            material_pool: Vec::new(),
            mesh_pool: Vec::new(),
        }
    }

    pub fn get_mesh_instance(&self) -> MeshInstance {
        let mesh = self.mesh_pool[0].borrow();
        let mesh_instance = MeshInstance::new(mesh);
        mesh_instance
    }
}



pub struct MaterialInstance<'a>  {
    material_data: &'a Material,
}

impl<'a> MaterialInstance<'a> {
}

pub struct MeshInstance<'a> {
    mesh_data: &'a Mesh
}

impl<'a> MeshInstance<'a> {
    pub fn new(mesh: &'a Mesh) -> MeshInstance {
        MeshInstance {
            mesh_data: mesh
        }
    }
}