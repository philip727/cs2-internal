use std::ffi::c_void;

use crate::{offsets, utils::memory::dereference_addr};

use super::{
    cs_player_controller::CCSPlayerController,
    data_types::{collision_property::CCollisionProperty, game_scene_node::CGameSceneNode},
};

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

    fn get_collision_property(&self) -> *mut c_void {
        unsafe {
            dereference_addr(
                self.raw()
                    .add(offsets::client_dll::C_BaseEntity::m_pCollision)
                    as *mut usize,
            )
        }
    }

    fn get_game_scene_node(&self) -> *mut CGameSceneNode {
        unsafe {
            dereference_addr(
                self.raw()
                    .add(offsets::client_dll::C_BaseEntity::m_pGameSceneNode)
                    as *mut usize,
            ) as *mut CGameSceneNode
        }
    }
}
