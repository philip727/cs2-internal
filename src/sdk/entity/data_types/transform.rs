use super::{quaternion::QuaternionAligned, vector::VectorAligned3D};

#[repr(align(16), C)]
pub struct CTransform {
    pub vec_position: VectorAligned3D,
    pub quaternion: QuaternionAligned,
}
