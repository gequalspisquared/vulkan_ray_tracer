use ash;
use ash::vk;

use crate::engine;

pub fn create_logical_device(
    device: &vk::PhysicalDevice,
    instance: &ash::Instance,
    surface: &vk::SurfaceKHR,
    surface_loader: &ash::khr::surface::Instance,
) -> ash::Device {
    let indices =
        engine::queue_families::find_queue_families(device, instance, surface, surface_loader);
    let queue_priority = 1.0_f32;
    let queues = [
        indices.graphics_family.unwrap(),
        indices.present_family.unwrap(),
    ];

    let mut queue_create_infos = vec![];
    for queue in queues.iter() {
        queue_create_infos.push(vk::DeviceQueueCreateInfo {
            queue_family_index: *queue,
            queue_count: 1,
            p_queue_priorities: &queue_priority,
            ..Default::default()
        });
    }

    let device_features = vk::PhysicalDeviceFeatures {
        ..Default::default()
    };

    // todo! update with is_debug_enabled
    // let layers = debug::get_required_layers(true);

    // May need to specify extensions and layers later
    let create_info = vk::DeviceCreateInfo {
        p_queue_create_infos: queue_create_infos.as_ptr(),
        queue_create_info_count: queue_create_infos.len() as u32,
        p_enabled_features: &device_features,
        ..Default::default()
    };

    unsafe { instance.create_device(*device, &create_info, None).unwrap() }
}
