use windows::Win32::Graphics::Dxgi::IDXGISwapChain;

#[repr(C)]
pub struct ISwapChainDx11 {
    _pad: [u8; 0x170], // Padding to align the data
    pub swap_chain: *mut IDXGISwapChain,
}
