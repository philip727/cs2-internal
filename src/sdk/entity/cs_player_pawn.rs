use std::ffi::c_void;


use super::base_entity::CBaseEntitySchema;

pub struct CCSPlayerPawn(pub *mut c_void);

impl CBaseEntitySchema for CCSPlayerPawn {
    fn raw(&self) -> *mut c_void {
        self.0
    }
}
