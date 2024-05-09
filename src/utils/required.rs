use std::ffi::CStr;

use ash::vk;

pub fn get_required_extensions() -> Vec<*const i8> {
    unsafe { vec![CStr::from_bytes_with_nul_unchecked(b"VK_KHR_swapchain\0").as_ptr()] }
}

pub fn get_required_extensions_cstr() -> Vec<&'static CStr> {
    vec![vk::KHR_SWAPCHAIN_NAME]
}

pub fn get_required_layers() -> Vec<*const i8> {
    unsafe { vec![CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0").as_ptr()] }
}
