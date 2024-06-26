use std::ffi::c_void;

use crate::offsets;

use super::{base_entity::CBaseEntitySchema, data_types::vector::Vector3D};

pub struct CCSPlayerPawn(pub *mut c_void);

impl CBaseEntitySchema for CCSPlayerPawn {
    fn raw(&self) -> *mut c_void {
        self.0
    }
}

impl CCSPlayerPawn {
    pub unsafe fn get_old_origin(&self) -> Vector3D {
        (self
            .0
            .add(offsets::client_dll::C_BasePlayerPawn::m_vOldOrigin) as *mut Vector3D)
            .read()
        //vector3d.z += 67f32;
    }
}
