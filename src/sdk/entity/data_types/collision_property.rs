use std::ffi::c_void;

use crate::offsets;

use super::vector::Vector3D;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct CCollisionProperty(pub *mut Self);

impl CCollisionProperty {
    pub fn get_vec_mins(&self) -> Vector3D {
        unsafe {
            (self
                .0
                .add(offsets::client_dll::CCollisionProperty::m_vecMins) as *const Vector3D)
                .read()
        }
    }

    pub fn get_vec_maxs(&self) -> Vector3D {
        unsafe {
            (self
                .0
                .add(offsets::client_dll::CCollisionProperty::m_vecMaxs) as *const Vector3D)
                .read()
        }
    }
}
