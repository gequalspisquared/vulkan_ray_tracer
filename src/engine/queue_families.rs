use ash;
use ash::vk;

#[derive(Default)]
pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
}

pub fn find_queue_families(
    device: &vk::PhysicalDevice,
    instance: &ash::Instance,
) -> QueueFamilyIndices {
    let mut indices = QueueFamilyIndices::default();

    let queue_families = unsafe { instance.get_physical_device_queue_family_properties(*device) };

    for (i, family) in queue_families.iter().enumerate() {
        if family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
            indices.graphics_family = Some(i as u32);
        }
    }

    indices
}
