use std::ffi::c_void;

use crate::offsets;

use super::cs_player_controller::CCSPlayerController;

#[derive(Clone, Copy)]
pub struct CBaseEntity(pub *mut c_void);

impl CBaseEntitySchema for CBaseEntity {
    fn raw(&self) -> *mut c_void {
        self.0
    }
}

impl Into<CCSPlayerController> for CBaseEntity {
    fn into(self) -> CCSPlayerController {
        CCSPlayerController(self.0)
    }
}

pub trait CBaseEntitySchema {
    fn raw(&self) -> *mut c_void;

    fn get_health(&self) -> i32 {
        unsafe {
            (self.raw().add(offsets::client_dll::C_BaseEntity::m_iHealth) as *const i32).read()
        }
    }

    fn get_max_health(&self) -> i32 {
        unsafe {
            (self
                .raw()
                .add(offsets::client_dll::C_BaseEntity::m_iMaxHealth) as *const i32)
                .read()
        }
    }
}
