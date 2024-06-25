use std::ffi::{c_void, CStr};

use crate::offsets;

use super::{entity_handle::CBaseHandle};

pub struct CCSPlayerController(pub *mut c_void);

impl CCSPlayerController {
    pub fn is_alive(&self) -> bool {
        unsafe {
            (self
                .0
                .add(offsets::client_dll::CCSPlayerController::m_bPawnIsAlive)
                as *const bool)
                .read()
        }
    }

    pub fn get_pawn_handle(&self) -> CBaseHandle {
        unsafe {
            (self
                .0
                .add(offsets::client_dll::CCSPlayerController::m_hPlayerPawn)
                as *mut CBaseHandle)
                .read()
        }
    }

    pub fn sanitized_player_name(&self) -> anyhow::Result<&str> {
        unsafe {
            let chars = self
                .0
                .add(offsets::client_dll::CCSPlayerController::m_sSanitizedPlayerName)
                as *const i8;

            Ok(CStr::from_ptr(chars).to_str()?)
        }
    }
}
