use std::sync::{Arc, Mutex, RwLock};

use crate::sdk::{
    entity::{
        base_entity::{CBaseEntity, CBaseEntitySchema},
        cs_player_controller::CCSPlayerController,
        cs_player_pawn::CCSPlayerPawn,
        data_types::{
            vector::{Vector3D, WorldToScreen},
            view_matrix::ViewMatrix4x4,
        },
    },
    interfaces::{
        engine_client::{CEngineClient, WrappedCEngineClient},
        game_entity_system::WrappedCGameEntitySystem,
    },
};

pub struct ESPContext {
    pub entries: [Option<ESPPlayerEntry>; 32],
    pub view_matrix: Option<ViewMatrix4x4>,
}

impl Default for ESPContext {
    fn default() -> Self {
        const ARRAY_REP_VAL: Option<ESPPlayerEntry> = None;
        Self {
            entries: [ARRAY_REP_VAL; 32],
            view_matrix: None,
        }
    }
}

impl ESPContext {
    pub unsafe fn run_update(
        &mut self,
        entity_system: &WrappedCGameEntitySystem,
        engine_client: &WrappedCEngineClient,
    ) {
        if !engine_client.in_game() {
            return;
        }

        for i in 1..32 {
            let entity = entity_system.get_entity_by_index(i);

            if entity.is_null() || !entity.is_aligned() {
                self.entries[i as usize] = None;
                continue;
            };

            let entity_base = CBaseEntity(entity);
            let player_controller: CCSPlayerController = entity_base.into();

            if !player_controller.is_alive() {
                self.entries[i as usize] = None;
                continue;
            }

            //let name = player_controller.sanitized_player_name();
            let pawn_handle = player_controller.get_pawn_handle();
            let player_pawn = CCSPlayerPawn(entity_system.get_entity_by_handle(pawn_handle));
            let mut pos = player_pawn.get_old_origin();
            let health = player_pawn.get_health();
            let max_health = player_pawn.get_max_health();

            let head_pos = Vector3D {
                x: pos.x,
                y: pos.y,
                z: pos.z + 75f32,
            };

            // just offset a lil bit
            pos.z -= 5f32;

            let esp_entry = ESPPlayerEntry {
                origin_pos: pos,
                head_pos,
                name: String::from("dummy"),
                health: (health, max_health)
            };

            self.entries[i as usize] = Some(esp_entry);
        }
    }
}

#[derive(Debug)]
pub struct ESPPlayerEntry {
    pub origin_pos: Vector3D,
    pub head_pos: Vector3D,
    pub name: String,
    pub health: (i32, i32),
}
