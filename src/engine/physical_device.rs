use std::ffi::CStr;

use ash;
use ash::vk;

use crate::engine;
use crate::utils::required;

pub fn pick_physical_device(
    instance: &ash::Instance,
    surface: &vk::SurfaceKHR,
    surface_loader: &ash::khr::surface::Instance,
) -> vk::PhysicalDevice {
    let devices = unsafe { instance.enumerate_physical_devices().unwrap() };
    println!("Devices [{}]: ", devices.len());
    devices.iter().for_each(|device| {
        let device_properties = unsafe { instance.get_physical_device_properties(*device) };
        println!("{:?}", device_properties.device_name_as_c_str().unwrap());
    });

    for device in devices.iter() {
        if is_device_suitable(&device, instance, surface, surface_loader) {
            return *device;
        }
    }

    panic!("Could not find any suitable devices!");
}

fn is_device_suitable(
    device: &vk::PhysicalDevice,
    instance: &ash::Instance,
    surface: &vk::SurfaceKHR,
    surface_loader: &ash::khr::surface::Instance,
) -> bool {
    let device_properties = unsafe { instance.get_physical_device_properties(*device) };
    let indices =
        engine::queue_families::find_queue_families(device, instance, surface, surface_loader);

    let extensions_supported = check_device_extension_support(device, instance);

    let swap_chain_support_details =
        engine::swap_chain::query_swap_chain_support(device, surface, surface_loader);
    let swap_chain_adequate = !swap_chain_support_details.formats.is_empty()
        && !swap_chain_support_details.present_modes.is_empty();

    device_properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU
        && indices.is_complete()
        && extensions_supported
        && swap_chain_adequate
}

fn check_device_extension_support(device: &vk::PhysicalDevice, instance: &ash::Instance) -> bool {
    let available_extensions = unsafe {
        instance
            .enumerate_device_extension_properties(*device)
            .unwrap()
    };
    let available_extensions: Vec<&CStr> = available_extensions
        .iter()
        .map(|extension| extension.extension_name_as_c_str().unwrap())
        .collect();
    let required_extensions = required::get_required_extensions_cstr();

    for required in required_extensions.iter() {
        if !available_extensions.contains(&required) {
            return false;
        }
    }

    true
}
