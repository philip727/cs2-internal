use crate::sdk::entity::data_types::{
    vector::{Vector3D, WorldToScreen}, view_matrix::ViewMatrix4x4,
};

pub struct ESPContext {
    pub entries: Vec<ESPPlayerEntry>,
    pub view_matrix: Option<ViewMatrix4x4>,
}

impl Default for ESPContext {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
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
}

pub struct ESPPlayerEntry {
    pub origin_pos: Vector3D,
    pub head_pos: Vector3D,
    pub name: String,
}
