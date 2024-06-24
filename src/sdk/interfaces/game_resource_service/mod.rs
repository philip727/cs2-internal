

use super::{game_entity_system::CGameEntitySystem, CaptureInterface};

#[repr(C)]
pub struct IGameResourceService {
    _pad: [u8; 0x58], // Padding to align the data
    pub game_entity_system: *mut CGameEntitySystem,
}

impl CaptureInterface<IGameResourceService> for IGameResourceService {}
