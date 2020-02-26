use winapi::{
    shared::{
        dxgi::{IDXGIFactory, IDXGISwapChain, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_EFFECT_DISCARD},
        dxgiformat,
        dxgitype,
        minwindef::TRUE,
        windef::HWND,
        winerror,
    },
    um::{d3d11, d3d11sdklayers, d3dcommon},
    Interface as _,
};

pub struct Device {
    raw: ComPtr<d3d11::ID3D11Device>,
    pub(crate) context: ComPtr<d3d11::ID3D11DeviceContext>,
    memory_properties: MemoryProperties,
    pub(crate) internal: internal::Internal,
}