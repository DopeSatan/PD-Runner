#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Graphics_CompositionSwapchain")]
pub mod CompositionSwapchain;
#[cfg(feature = "Win32_Graphics_DXCore")]
pub mod DXCore;
#[cfg(feature = "Win32_Graphics_Direct2D")]
pub mod Direct2D;
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub mod Direct3D;
#[cfg(feature = "Win32_Graphics_Direct3D10")]
pub mod Direct3D10;
#[cfg(feature = "Win32_Graphics_Direct3D11")]
pub mod Direct3D11;
#[cfg(feature = "Win32_Graphics_Direct3D11on12")]
pub mod Direct3D11on12;
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub mod Direct3D12;
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub mod Direct3D9;
#[cfg(feature = "Win32_Graphics_Direct3D9on12")]
pub mod Direct3D9on12;
#[cfg(feature = "Win32_Graphics_DirectComposition")]
pub mod DirectComposition;
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub mod DirectDraw;
#[cfg(feature = "Win32_Graphics_DirectManipulation")]
pub mod DirectManipulation;
#[cfg(feature = "Win32_Graphics_DirectWrite")]
pub mod DirectWrite;
#[cfg(feature = "Win32_Graphics_Dwm")]
pub mod Dwm;
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub mod Dxgi;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub mod Gdi;
#[cfg(feature = "Win32_Graphics_Hlsl")]
pub mod Hlsl;
#[cfg(feature = "Win32_Graphics_Imaging")]
pub mod Imaging;
#[cfg(feature = "Win32_Graphics_OpenGL")]
pub mod OpenGL;
#[cfg(feature = "Win32_Graphics_Printing")]
pub mod Printing;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
