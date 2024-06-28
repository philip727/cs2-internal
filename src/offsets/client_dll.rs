#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]
pub const dwLocalPlayerController: usize = 0x1A0DA88;
pub const dwViewMatrix: usize = 0x1A1FDB0;

pub mod C_BaseEntity {
    pub const m_iHealth: usize = 0x324; // int32
    pub const m_iMaxHealth: usize = 0x320; // int32
    pub const m_fFlags: usize = 0x3CC; // uint32
    pub const m_pCollision: usize = 0x318; // CCollisionProperty*
    pub const m_pGameSceneNode: usize = 0x308; // CGameSceneNode*
    pub const m_iTeamNum: usize = 0x3C3; // uint8
}

pub mod C_BasePlayerPawn {
    pub const m_vOldOrigin: usize = 0x1274; // Vector
}

pub mod CCSPlayerController {
    pub const m_bPawnIsAlive: usize = 0x7E4; // bool
    pub const m_hPlayerPawn: usize = 0x7DC; // CHandle<C_CSPlayerPawn>
    pub const m_sSanitizedPlayerName: usize = 0x740; // CUtlString
}

pub mod CCollisionProperty {
    pub const m_vecMins: usize = 0x40; // Vector
    pub const m_vecMaxs: usize = 0x4C; // Vector
}

pub mod CGameSceneNode {
    pub const m_nodeToWorld: usize = 0x10; // CTransform
    pub const m_vecAbsOrigin: usize = 0xD0; // Vector
}
