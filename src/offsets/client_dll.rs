#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]
pub const dwLocalPlayerPawn: usize = 0x181A9B8;
pub const dwEntityList: usize = 0x19B49B8;
pub const dwLocalPlayerController: usize = 0x1A04768;

pub mod C_BaseEntity {
    pub const m_iHealth: usize = 0x324; // int32
    pub const m_iMaxHealth: usize = 0x320; // int32
    pub const m_fFlags: usize = 0x3CC; // uint32
}

pub mod CCSPlayerController {
    pub const m_bPawnIsAlive: usize = 0x7E4; // bool
    pub const m_hPlayerPawn: usize = 0x7DC; // CHandle<C_CSPlayerPawn>
    pub const m_sSanitizedPlayerName: usize = 0x740; // CUtlString
}
