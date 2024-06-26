use super::CaptureInterface;

type GetIsInGame = unsafe extern "thiscall" fn(*mut usize) -> bool;

#[derive(Debug)]
pub struct CEngineClient {}

impl CaptureInterface<CEngineClient> for CEngineClient {}

impl CEngineClient {
    pub fn get_is_in_game(this: *mut CEngineClient) -> anyhow::Result<bool> {
        Ok(unsafe {
            (CaptureInterface::get_virtual_func::<GetIsInGame>(this, 35)?)(this as *mut usize)
        })
    }
}

pub struct WrappedCEngineClient {
    pub engine_client: *mut CEngineClient,
    get_is_in_game_fn: GetIsInGame,
}

unsafe impl Send for WrappedCEngineClient {}
unsafe impl Sync for WrappedCEngineClient {}

impl WrappedCEngineClient {
    pub fn init(engine_client: *mut CEngineClient) -> anyhow::Result<Self> {
        unsafe {
            let get_is_in_game =
                CaptureInterface::get_virtual_func::<GetIsInGame>(engine_client, 35)?;

            Ok(Self {
                engine_client,
                get_is_in_game_fn: get_is_in_game,
            })
        }
    }

    pub fn in_game(&self) -> bool {
        unsafe { (self.get_is_in_game_fn)(self.engine_client as *mut usize) }
    }
}
