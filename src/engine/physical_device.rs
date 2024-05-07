use ash;
use ash::vk;

use crate::engine;

pub fn pick_physical_device(instance: &ash::Instance) -> vk::PhysicalDevice {
    let devices = unsafe { instance.enumerate_physical_devices().unwrap() };
    println!("Devices [{}]: ", devices.len());
    devices.iter().for_each(|device| {
        let device_properties = unsafe { instance.get_physical_device_properties(*device) };
        println!("{:?}", device_properties.device_name_as_c_str().unwrap());
    });

    for device in devices.iter() {
        if is_device_suitable(&device, instance) {
            let indices = engine::queue_families::find_queue_families(&device, instance);
            if indices.graphics_family.is_some() {
                return *device;
            }
        }
    }

    panic!("Could not find any suitable devices!");
}

pub fn is_device_suitable(device: &vk::PhysicalDevice, instance: &ash::Instance) -> bool {
    let device_properties = unsafe { instance.get_physical_device_properties(*device) };

    device_properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU
}
