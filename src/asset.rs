//#![macro_use]

pub mod math;
pub use self::math::*;

mod root;
pub use self::root::*;
mod scene;
pub use self::scene::*;
mod node;
pub use self::node::*;
mod mesh;
pub use self::mesh::*;
mod primitive;
pub use self::primitive::*;

mod material;
pub use self::material::*;
mod texture;
pub use self::texture::*;

mod camera;
pub use self::camera::*;

//pub mod importer;
//mod error;

use bitflags::bitflags;


/// Helps to simplify the signature of import related functions.
pub struct ImportData {
    pub doc: gltf::Document,
    pub buffers: Vec<gltf::buffer::Data>,
    pub images: Vec<gltf::image::Data>,
}

#[derive(Debug)]
pub struct CameraParams {
    pub position: Vector3,
    pub view_matrix: Matrix4,
    pub projection_matrix: Matrix4,
}

pub const ZOOM: f32 = 45.0;


bitflags! {
    /// Flags matching the defines in the PBR shader
    pub struct ShaderFlags: u16 {
        // vertex shader + fragment shader
        const HAS_NORMALS           = 1;
        const HAS_TANGENTS          = 1 << 1;
        const HAS_UV                = 1 << 2;
        const HAS_COLORS            = 1 << 3;

        // fragment shader only
        const USE_IBL               = 1 << 4;
        const HAS_BASECOLORMAP      = 1 << 5;
        const HAS_NORMALMAP         = 1 << 6;
        const HAS_EMISSIVEMAP       = 1 << 7;
        const HAS_METALROUGHNESSMAP = 1 << 8;
        const HAS_OCCLUSIONMAP      = 1 << 9;
        const USE_TEX_LOD           = 1 << 10;
    }
}





impl ShaderFlags {
    pub fn as_strings(self) -> Vec<String> {
        (0..15)
            .map(|i| 1u16 << i)
            .filter(|i| self.bits & i != 0)
            .map(|i| format!("{:?}", ShaderFlags::from_bits_truncate(i)))
            .collect()
    }
}