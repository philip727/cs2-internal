use std::sync::{Arc, Mutex, RwLock};

use crate::sdk::{
    entity::{
        base_entity::CBaseEntity,
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
    pub fn create_esp_entry(&mut self, origin: &Vector3D, player_name: String) -> ESPPlayerEntry {
        let head_pos = Vector3D {
            x: origin.x,
            y: origin.y,
            z: origin.z + 75f32,
        };

        ESPPlayerEntry {
            origin_pos: *origin,
            head_pos,
            name: player_name,
        }
    }

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
            let pos = player_pawn.get_old_origin();

            let esp_entry = self.create_esp_entry(&pos, String::from("dummy"));
            self.entries[i as usize] = Some(esp_entry);
        }
    }
}

#[derive(Debug)]
pub struct ESPPlayerEntry {
    pub origin_pos: Vector3D,
    pub head_pos: Vector3D,
    pub name: String,
}
