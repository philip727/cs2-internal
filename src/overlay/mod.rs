use std::sync::{Arc, Mutex};

use hudhook::{
    hooks::dx11::ImguiDx11Hooks,
    imgui::{ImColor32, Ui},
    ImguiRenderLoop,
};

use crate::{config::ConfigContext, sdk::entity::data_types::vector::WorldToScreen};

use self::esp::ESPContext;
pub mod esp;

pub struct OverlayRenderLoop {
    pub config_ctx: Arc<Mutex<ConfigContext>>,
    pub esp_ctx: Arc<Mutex<ESPContext>>,
}

impl ImguiRenderLoop for OverlayRenderLoop {
    fn render(&mut self, ui: &mut Ui) {
        let mut config_context = { self.config_ctx.lock().unwrap() };
        let esp_context = { self.esp_ctx.lock().unwrap() };

        if let Some(view_matrix) = &esp_context.view_matrix {
            for entry in esp_context.entries.iter() {
                let Some(entry) = entry else {
                    continue;
                };

                let Some(origin_screen_pos) = entry.origin_pos.world_to_screen(&view_matrix, ui)
                else {
                    continue;
                };

                let Some(head_screen_pos) = entry.head_pos.world_to_screen(&view_matrix, ui) else {
                    continue;
                };

                let height = origin_screen_pos.y - head_screen_pos.y;
                let width = height * 0.3f32;

                //println!("{origin_screen_pos:?}");
                //
                let drawlist = ui.get_background_draw_list();

                // Bounding Box
                drawlist
                    .add_rect(
                        [head_screen_pos.x - width, head_screen_pos.y],
                        [head_screen_pos.x + width, origin_screen_pos.y],
                        ImColor32::WHITE,
                    )
                    .build();

                // Health bar
                drawlist
                    .add_rect(
                        [head_screen_pos.x - width - 4f32, head_screen_pos.y],
                        [head_screen_pos.x - width - 2f32, origin_screen_pos.y],
                        ImColor32::BLACK,
                    )
                    .filled(true)
                    .build();

                let [text_width, _] = ui.calc_text_size(&entry.name);

                drawlist.add_text(
                    [
                        head_screen_pos.x - (text_width / 2f32),
                        head_screen_pos.y - 20f32,
                    ],
                    ImColor32::WHITE,
                    &entry.name,
                );
            }
        }

        ui.get_foreground_draw_list()
            .add_rect(
                [0.0, 0.0],
                [20.0, 20.0],
                ImColor32::from_rgba(255, 255, 255, 255),
            )
            .filled(true)
            .rounding(2.0)
            .build();

        ui.window("GUI")
            .build(|| ui.checkbox("print values", &mut config_context.print_values));
    }
}

pub fn create_overlay(config_ctx: Arc<Mutex<ConfigContext>>, esp_ctx: Arc<Mutex<ESPContext>>) {
    hudhook::Hudhook::builder()
        .with::<ImguiDx11Hooks>(OverlayRenderLoop {
            config_ctx,
            esp_ctx,
        })
        .build()
        .apply()
        .unwrap();
}
