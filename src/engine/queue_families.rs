use ash;
use ash::vk;

#[derive(Default)]
pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
    pub present_family: Option<u32>,
}

pub fn find_queue_families(
    device: &vk::PhysicalDevice,
    instance: &ash::Instance,
    surface: &vk::SurfaceKHR,
    surface_loader: &ash::khr::surface::Instance,
) -> QueueFamilyIndices {
    let mut indices = QueueFamilyIndices::default();

    let queue_families = unsafe { instance.get_physical_device_queue_family_properties(*device) };

    for (i, family) in queue_families.iter().enumerate() {
        if family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
            indices.graphics_family = Some(i as u32);
        }

        let present_support = unsafe {
            surface_loader
                .get_physical_device_surface_support(*device, i as u32, *surface)
                .unwrap()
        };
        if present_support {
            indices.present_family = Some(i as u32);
        }
    }

    indices
}
