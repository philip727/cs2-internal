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
        //                             "48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 41 54 41 56 41 57 48 81 EC ? ? ? ? 4C 8B A4 24 ? ? ? ?" 
        let sig = skidscan::signature!("48 89 ? 24 ? 48 89 ? 24 ? ? ? ? ? EC 20 41 8B ?");

        unsafe {
            let present_ptr = memory::relative_rip(
                sig.scan_module("GameOverlayRenderer64.dll")
                    .unwrap()
                    .add(0xAD) as *mut c_void,
                6,
            );

            let original_present_fn = *(present_ptr as *mut PresentFn);
        }
    }
}
