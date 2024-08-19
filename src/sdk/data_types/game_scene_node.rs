use std::ffi::c_void;

use crate::offsets;

use super::{transform::CTransform, vector::Vector3D};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct CGameSceneNode(pub *mut Self);

impl CGameSceneNode {
    pub fn node_to_world(&self) -> CTransform {
        unsafe {
            (self
                .0
                .add(offsets::client_dll::CGameSceneNode::m_nodeToWorld)
                as *mut CTransform)
                .read_unaligned()
        }
    }

    pub fn get_abs_origin(&self) -> Vector3D {
        unsafe {
            *(self
                .0
                .add(offsets::client_dll::CGameSceneNode::m_vecAbsOrigin)
                as *mut Vector3D)
        }
    }
}
