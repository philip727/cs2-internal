use std::os::raw::c_void;

use hudhook::imgui::{DrawListMut, ImColor32};

use crate::{
    config::ConfigContext,
    offsets,
    sdk::{
        data_types::{vector::{Vector2D, Vector3D}, view_matrix::ViewMatrix4x4}, entity::{
            base_entity::{CBaseEntity, CBaseEntitySchema},
            cs_player_controller::CCSPlayerController,
            cs_player_pawn::CCSPlayerPawn,
        }, interfaces::{
            engine_client::WrappedCEngineClient, game_entity_system::WrappedCGameEntitySystem,
        }
    },
    utils::module::Module,
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
    pub fn empty_entries(&mut self) {
        for entry in self.entries.iter_mut() {
            *entry = None;
        }
    }

    pub unsafe fn run_update(
        &mut self,
        entity_system: &WrappedCGameEntitySystem,
        client_dll: &Module,
    ) {
        let local_player_addr = *((client_dll.base_addr()
            + offsets::client_dll::dwLocalPlayerController)
            as *mut *mut c_void);

        let mut local_player_controller: Option<CCSPlayerController> = None;
        if local_player_addr.is_aligned() && !local_player_addr.is_null() {
            local_player_controller = Some(CCSPlayerController(local_player_addr));
        }

        for i in 1..32 {
            let entity = entity_system.get_entity_by_index(i);

            if entity.is_null() || !entity.is_aligned() {
                self.entries[i as usize] = None;
                continue;
            };

            let entity_base = CBaseEntity(entity);

            if let Some(local_player_controller) = &local_player_controller {
                if entity_base.get_team() == local_player_controller.get_team() {
                    self.entries[i as usize] = None;
                    continue;
                }
            }

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
                health: (health, max_health),
            };

            self.entries[i as usize] = Some(esp_entry);
        }
    }

    pub fn create_bounding_box(
        drawlist: &DrawListMut,
        head_screen_pos: &Vector2D,
        origin_screen_pos: &Vector2D,
    ) {
        let height = origin_screen_pos.y - head_screen_pos.y;
        let width = height * 0.3f32;

        // Box Border
        drawlist
            .add_rect(
                [head_screen_pos.x - width, head_screen_pos.y],
                [head_screen_pos.x + width, origin_screen_pos.y],
                ImColor32::BLACK,
            )
            .thickness(3f32)
            .build();

        // Box
        drawlist
            .add_rect(
                [head_screen_pos.x - width, head_screen_pos.y],
                [head_screen_pos.x + width, origin_screen_pos.y],
                ImColor32::WHITE,
            )
            .build();
    }

    pub fn create_health_bar(
        drawlist: &DrawListMut,
        head_screen_pos: &Vector2D,
        origin_screen_pos: &Vector2D,
        health: &(i32, i32),
    ) {
        let height = origin_screen_pos.y - head_screen_pos.y;
        let width = height * 0.3f32;

        let bar_top = head_screen_pos.y - 1f32;
        let bar_bottom = origin_screen_pos.y + 1f32;
        // Health bar outline
        drawlist
            .add_rect(
                [head_screen_pos.x - width - 5f32, bar_top],
                [head_screen_pos.x - width - 2f32, bar_bottom],
                ImColor32::BLACK,
            )
            .filled(true)
            .build();

        // Full bar height
        let bar_height = (bar_bottom - bar_top) - 2f32;

        // health / max = perc
        let health_percentage = health.0 as f32 / health.1 as f32;
        let bar_height = bar_height * health_percentage;

        // Health bar fill
        drawlist
            .add_rect(
                [
                    head_screen_pos.x - width - 4f32,
                    origin_screen_pos.y - bar_height,
                ],
                [head_screen_pos.x - width - 3f32, origin_screen_pos.y],
                ImColor32::from_rgb(0, 255, 0),
            )
            .filled(true)
            .build();
    }
}

#[derive(Debug)]
pub struct ESPPlayerEntry {
    pub origin_pos: Vector3D,
    pub head_pos: Vector3D,
    pub name: String,
    pub health: (i32, i32),
}
