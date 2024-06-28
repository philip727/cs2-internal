use std::os::raw::c_void;

use super::CaptureInterface;

type GetIsInGameFn = unsafe extern "thiscall" fn(*mut usize) -> bool;
type GetLocalPlayerFn = unsafe extern "thiscall" fn(*mut usize, &mut i32, u64) -> c_void;

#[derive(Debug)]
pub struct CEngineClient {}

impl CaptureInterface<CEngineClient> for CEngineClient {}

pub struct WrappedCEngineClient {
    pub engine_client: *mut CEngineClient,
    get_is_in_game_fn: GetIsInGameFn,
    get_local_player_fn: GetLocalPlayerFn,
}

unsafe impl Send for WrappedCEngineClient {}
unsafe impl Sync for WrappedCEngineClient {}

impl WrappedCEngineClient {
    pub fn init(engine_client: *mut CEngineClient) -> anyhow::Result<Self> {
        unsafe {
            let get_is_in_game_fn =
                CaptureInterface::get_virtual_func::<GetIsInGameFn>(engine_client, 35)?;

            let get_local_player_fn =
                CaptureInterface::get_virtual_func::<GetLocalPlayerFn>(engine_client, 47)?;

            Ok(Self {
                engine_client,
                get_is_in_game_fn,
                get_local_player_fn,
            })
        }
    }

    pub fn in_game(&self) -> bool {
        unsafe { (self.get_is_in_game_fn)(self.engine_client as *mut usize) }
    }

    pub fn get_local_player(&self) -> i32 {
        let mut index = -1;
        unsafe { (self.get_local_player_fn)(self.engine_client as *mut usize, &mut index, 0) };
        index
    }
}
