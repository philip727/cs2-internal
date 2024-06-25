
use super::entity_identity::GetEntryIndex;

#[repr(C)]
pub struct CBaseHandle {
    pub n_index: u32,
}

impl GetEntryIndex for CBaseHandle {
    fn index(&self) -> u32 {
        self.n_index
    }

    fn get_entry_index(&self) -> i32 {
        (self.n_index & 0x7FFF) as i32
    }
}
