use std::sync::{Arc, Mutex};

use hudhook::{hooks::dx11::ImguiDx11Hooks, imgui::{ImColor32, Ui}, ImguiRenderLoop};

use crate::config::ConfigContext;

pub struct OverlayRenderLoop {
    pub config_ctx: Arc<Mutex<ConfigContext>>,
}

impl ImguiRenderLoop for OverlayRenderLoop {
    fn render(&mut self, ui: &mut Ui) {
        let mut config_context = { self.config_ctx.lock().unwrap() };

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

pub fn create_overlay(config_ctx: &Arc<Mutex<ConfigContext>>) {
    hudhook::Hudhook::builder()
        .with::<ImguiDx11Hooks>(OverlayRenderLoop {
            config_ctx: Arc::clone(config_ctx),
        })
        .build()
        .apply()
        .unwrap();
}

