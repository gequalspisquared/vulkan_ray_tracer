#[cfg(target_os = "windows")]
use ash::khr::win32_surface;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::khr::xlib_surface;
#[cfg(target_os = "macos")]
use ash::mvk::macos_surface;

use ash::ext::debug_utils;
use ash::khr::surface;
// use ash::vk;

#[cfg(target_os = "windows")]
pub fn get_required_extensions() -> Vec<*const i8> {
    vec![
        debug_utils::NAME.as_ptr(),
        surface::NAME.as_ptr(),
        win32_surface::NAME.as_ptr(),
    ]
}

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
pub fn get_required_extensions() -> Vec<*const i8> {
    vec![
        debug_utils::NAME.as_ptr(),
        surface::NAME.as_ptr(),
        xlib_surface::NAME.as_ptr(),
    ]
}

#[cfg(target_os = "macos")]
pub fn get_required_extensions() -> Vec<*const i8> {
    vec![
        debug_utils::NAME.as_ptr(),
        surface::NAME.as_ptr(),
        macos_surface::NAME.as_ptr(),
    ]
}

// #[cfg(target_os = "windows")]
// pub fn create_surface(window: &winit::window::Window) -> vk::SurfaceKHR {
//     use std::os::raw::c_void;
//     use std::ptr;
//     use winapi::shared::windef::HWND;
//     use winapi::um::libloaderapi::GetModuleHandleW;
//     use winit::platform::windows::WindowExtWindows;

//     let hwnd = window.hwnd() as HWND;
// }
