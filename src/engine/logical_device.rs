use ash;
use ash::vk;

use crate::engine;

pub fn create_logical_device(device: &vk::PhysicalDevice, instance: &ash::Instance) -> ash::Device {
    let create_info = create_device_info(device, instance);
    unsafe { instance.create_device(*device, &create_info, None).unwrap() }
}

fn create_device_info(
    device: &vk::PhysicalDevice,
    instance: &ash::Instance,
) -> vk::DeviceCreateInfo<'static> {
    let indices = engine::queue_families::find_queue_families(device, instance);
    let queue_priority = 1.0f32;
    let queue_create_info = vk::DeviceQueueCreateInfo {
        queue_family_index: indices.graphics_family.unwrap(),
        queue_count: 1,
        p_queue_priorities: &queue_priority,
        ..Default::default()
    };

    let device_features = vk::PhysicalDeviceFeatures {
        ..Default::default()
    };

    // May need to specify extensions and layers later
    vk::DeviceCreateInfo {
        p_queue_create_infos: &queue_create_info,
        queue_create_info_count: 1,
        p_enabled_features: &device_features,
        ..Default::default()
    }
}
