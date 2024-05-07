use ash;
use ash::vk;

use std::ffi::c_void;

use crate::utils;

pub fn create_instance(entry: &ash::Entry, is_debug_enabled: bool) -> ash::Instance {
    let app_info = vk::ApplicationInfo {
        api_version: vk::make_api_version(0, 1, 0, 0),
        ..Default::default()
    };

    let extensions = utils::platforms::get_required_extensions();

    let debug_create_info = utils::debug::populate_debug_messenger_create_info();
    let enabled_layer_names = utils::debug::get_required_layers(is_debug_enabled);

    let create_info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        pp_enabled_extension_names: extensions.as_ptr(),
        enabled_extension_count: extensions.len() as u32,
        pp_enabled_layer_names: enabled_layer_names.as_ptr(),
        enabled_layer_count: enabled_layer_names.len() as u32,
        p_next: &debug_create_info as *const vk::DebugUtilsMessengerCreateInfoEXT as *const c_void,
        ..Default::default()
    };

    unsafe {
        entry
            .create_instance(&create_info, None)
            .expect("Failed to create instance!")
    }
}
