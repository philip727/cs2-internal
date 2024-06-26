use std::{
    borrow::Cow,
    ffi::{c_void, CStr},
    slice::from_raw_parts,
};

use crate::{offsets, utils::memory::dereference_addr};

use super::entity_handle::CBaseHandle;

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

    pub unsafe fn sanitized_player_name(&self) -> String {
        let chars = {
            dereference_addr(
                self.0
                    .add(offsets::client_dll::CCSPlayerController::m_sSanitizedPlayerName)
                    as *mut c_void,
            ) as *mut [u8; 128]
        }
        .read();

        String::from_utf8_lossy(&chars).split("\0").next().unwrap_or("").to_string()
    }
}
