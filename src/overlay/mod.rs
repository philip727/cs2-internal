use std::sync::{Arc, Mutex};

use hudhook::{
    hooks::dx11::ImguiDx11Hooks,
    imgui::{ImColor32, ImStr, StyleVar, Ui},
    ImguiRenderLoop,
};

use crate::{config::ConfigContext, sdk::data_types::vector::WorldToScreen};

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

        ui.get_foreground_draw_list()
            .add_rect(
                [0.0, 0.0],
                [20.0, 20.0],
                ImColor32::from_rgba(255, 255, 255, 255),
            )
            .filled(true)
            .rounding(2.0)
            .build();

        unsafe {
            ui.window("Valeria")
                .size([600f32, 400f32], hudhook::imgui::Condition::Once)
                .title_bar(true)
                .resizable(false)
                .collapsible(false)
                .build(|| {
                    custom_checkbox(
                        ui,
                        ImStr::from_utf8_with_nul_unchecked(b"ESP Enabled"),
                        &mut config_context.esp_enabled,
                    )
                });
        }

        if !config_context.esp_enabled {
            return;
        }

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

                let drawlist = ui.get_background_draw_list();

                // Bounding Box
                ESPContext::create_bounding_box(&drawlist, &head_screen_pos, &origin_screen_pos);

                // Health Bar
                ESPContext::create_health_bar(
                    &drawlist,
                    &head_screen_pos,
                    &origin_screen_pos,
                    &entry.health,
                );

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
    }
}

fn custom_checkbox(ui: &Ui, label: &ImStr, value: &mut bool) -> bool {
    let style = ui.push_style_var(StyleVar::FrameBorderSize(1.0));
    let clicked = ui.checkbox(label, value);
    style.pop();

    clicked
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
