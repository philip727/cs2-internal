
use super::CaptureInterface;


#[derive(Debug)]
pub struct CEngineClient {}

impl CaptureInterface<CEngineClient> for CEngineClient {}

impl CEngineClient {
    pub fn get_is_in_game(this: *mut CEngineClient) -> anyhow::Result<bool> {
        type GetIsInGame = unsafe extern "thiscall" fn(*mut usize) -> bool;
        Ok(unsafe {
            (CaptureInterface::get_virtual_func::<GetIsInGame>(this, 35)?)(this as *mut usize)
        })
    }
}
