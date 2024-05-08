use winit::raw_window_handle::RawWindowHandle;

use ash;
use ash::vk;

pub fn create_surface(
    entry: &ash::Entry,
    instance: &ash::Instance,
    raw_window_handle: &RawWindowHandle,
) -> vk::SurfaceKHR {
    let mut create_info = vk::Win32SurfaceCreateInfoKHR {
        ..Default::default()
    };

    match raw_window_handle {
        RawWindowHandle::Win32(handle) => {
            create_info = vk::Win32SurfaceCreateInfoKHR {
                hwnd: isize::from(handle.hwnd),
                hinstance: isize::from(handle.hinstance.unwrap()),
                ..Default::default()
            };
        }
        RawWindowHandle::Xlib(handle) => println!("{}", handle.window),
        RawWindowHandle::AppKit(_) => println!("macos window handle"),
        _ => {}
    }

    let surface_instance = ash::khr::win32_surface::Instance::new(entry, instance);
    unsafe {
        surface_instance
            .create_win32_surface(&create_info, None)
            .unwrap()
    }
}
