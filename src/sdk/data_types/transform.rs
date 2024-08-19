use super::{quaternion::QuaternionAligned, vector::VectorAligned3D};

#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct CTransform {
    pub position: VectorAligned3D,
    pub orientation: QuaternionAligned,
}
