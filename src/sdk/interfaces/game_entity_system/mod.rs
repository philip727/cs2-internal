use std::ffi::c_void;

use crate::sdk::entity::entity_identity::GetEntryIndex;
pub type GetBaseEntityFn = extern "thiscall" fn(*mut c_void, i32) -> *mut c_void;

pub struct WrappedCGameEntitySystem {
    pub game_entity_system: *mut CGameEntitySystem,
    get_base_entity_fn: GetBaseEntityFn,
}

unsafe impl Send for WrappedCGameEntitySystem {}
unsafe impl Sync for WrappedCGameEntitySystem {}


impl WrappedCGameEntitySystem {
    pub fn init(game_entity_system: *mut CGameEntitySystem) -> Self {
        unsafe {
            let sig = skidscan::signature!("81 FA ? ? ? ? 77 ? 8B C2 C1 F8 ? 83 F8 ? 77 ? 48 98 48 8B 4C C1 ? 48 85 C9 74 ? 8B C2 25 ? ? ? ? 48 6B C0 ? 48 03 C8 74 ? 8B 41 ? 25 ? ? ? ? 3B C2 75 ? 48 8B 01");

            let Ok(rel_address) = sig.scan_module("client.dll") else {
                panic!("Failed to create wrapped entity system");
            };

            let get_base_fn = std::mem::transmute::<_, GetBaseEntityFn>(rel_address);

            Self {
                game_entity_system,
                get_base_entity_fn: get_base_fn,
            }
        }
    }

    pub fn get_entity_by_handle(&self, handle: impl GetEntryIndex) -> *mut c_void {
        self.get_entity_by_index(handle.get_entry_index())
    }

    pub fn get_entity_by_index(&self, index: i32) -> *mut c_void {
        (self.get_base_entity_fn)(self.game_entity_system as *mut c_void, index)
    }
}

#[repr(C)]
pub struct CGameEntitySystem;
