use std::ffi::c_void;

use windows::{
    core::HRESULT,
    Win32::{
        Foundation::HWND,
        Graphics::{
            Direct3D11::{ID3D11Device, ID3D11DeviceContext, ID3D11RenderTargetView},
            Dxgi::IDXGISwapChain,
        },
        UI::WindowsAndMessaging::WNDCLASSEXA,
    },
};

use crate::{sdk::interfaces::swap_chain_dx11::ISwapChainDx11, utils::memory};

type PresentFn =
    extern "stdcall" fn(this: *mut IDXGISwapChain, sync_interval: u32, flags: u32) -> HRESULT;

pub struct GuiContext {
    pub open: bool,
    pub setup: bool,

    // winapi
    pub window: HWND,
    pub window_class: WNDCLASSEXA,

    // dx
    pub swap_chain: *mut IDXGISwapChain,
    pub device: *mut ID3D11Device,
    pub device_context: *mut ID3D11DeviceContext,
    pub render_target_view: *mut ID3D11RenderTargetView,

    pub present_ptr: *mut c_void,
    pub original_presenet: *mut PresentFn,
}

impl GuiContext {
    pub fn initialize() {
        let sig = skidscan::signature!("66 0F 7F 0D ? ? ? ? 66 0F 7F 05 ? ? ? ? 0F 1F 40");

        unsafe {
            let swap_chain = **(memory::resolve_relative_address(
                sig.scan_module("rendersystemdx11.dll").unwrap() as *mut c_void,
                0x4,
                0x8,
            ) as *mut *mut *mut ISwapChainDx11);
        }
    }
}
