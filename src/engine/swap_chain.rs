use ash;
use ash::vk;

pub struct SwapChain {
    pub swap_chain: vk::SwapchainKHR,
    pub swap_chain_device: ash::khr::swapchain::Device,
    pub swap_chain_images: Vec<vk::Image>,
    pub swap_chain_image_views: Vec<vk::ImageView>,
    pub image_format: vk::Format,
    pub extent: vk::Extent2D,
}

impl SwapChain {
    pub fn new(
        device: &vk::PhysicalDevice,
        instance: &ash::Instance,
        logical_device: &ash::Device,
        surface: &vk::SurfaceKHR,
        surface_loader: &ash::khr::surface::Instance,
        window: &winit::window::Window,
    ) -> Self {
        let swap_chain_support = query_swap_chain_support(device, surface, surface_loader);

        let surface_format = choose_swap_surface_format(&swap_chain_support.formats);
        let present_mode = choose_swap_present_mode(&swap_chain_support.present_modes);
        let extent = choose_swap_extent(&swap_chain_support.capabilities, window);

        let mut image_count = swap_chain_support.capabilities.min_image_count + 1;
        if swap_chain_support.capabilities.max_image_count > 0
            && image_count > swap_chain_support.capabilities.max_image_count
        {
            image_count = swap_chain_support.capabilities.max_image_count;
        }

        let mut create_info = vk::SwapchainCreateInfoKHR {
            surface: *surface,
            min_image_count: image_count,
            image_format: surface_format.format,
            image_color_space: surface_format.color_space,
            image_extent: extent,
            image_array_layers: 1,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            ..Default::default()
        };

        let indices = crate::engine::queue_families::find_queue_families(
            device,
            instance,
            surface,
            surface_loader,
        );
        let queue_family_indices = [
            indices.graphics_family.unwrap(),
            indices.present_family.unwrap(),
        ];

        if indices.graphics_family != indices.present_family {
            create_info.image_sharing_mode = vk::SharingMode::CONCURRENT;
            create_info.queue_family_index_count = 2;
            create_info.p_queue_family_indices = queue_family_indices.as_ptr();
        } else {
            create_info.image_sharing_mode = vk::SharingMode::EXCLUSIVE;
        }

        create_info.pre_transform = swap_chain_support.capabilities.current_transform;
        create_info.composite_alpha = vk::CompositeAlphaFlagsKHR::OPAQUE;
        create_info.present_mode = present_mode;
        create_info.clipped = vk::TRUE;
        create_info.old_swapchain = vk::SwapchainKHR::null();

        let swap_chain_device = ash::khr::swapchain::Device::new(instance, logical_device);
        let swap_chain = unsafe {
            swap_chain_device
                .create_swapchain(&create_info, None)
                .expect("Failed to create swapchain!")
        };

        let swap_chain_images =
            unsafe { swap_chain_device.get_swapchain_images(swap_chain).unwrap() };
        let swap_chain_image_views = swap_chain_images
            .iter()
            .map(|image| {
                let components = vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                };
                let image_subresource_range = vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                };
                let create_info = vk::ImageViewCreateInfo {
                    image: *image,
                    view_type: vk::ImageViewType::TYPE_2D,
                    format: surface_format.format,
                    components: components,
                    subresource_range: image_subresource_range,
                    ..Default::default()
                };

                unsafe {
                    logical_device
                        .create_image_view(&create_info, None)
                        .unwrap()
                }
            })
            .collect();

        let image_format = surface_format.format;
        let extent = choose_swap_extent(&swap_chain_support.capabilities, window);

        Self {
            swap_chain,
            swap_chain_device,
            swap_chain_images,
            swap_chain_image_views,
            image_format,
            extent,
        }
    }

    pub fn cleanup(&mut self, logical_device: &ash::Device) {
        unsafe {
            for image_view in self.swap_chain_image_views.iter() {
                logical_device.destroy_image_view(*image_view, None);
            }
            self.swap_chain_device
                .destroy_swapchain(self.swap_chain, None);
        }
    }
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        unsafe {
            self.swap_chain_device
                .destroy_swapchain(self.swap_chain, None);
        }
    }
}

pub struct SwapChainSupportDetails {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

pub fn query_swap_chain_support(
    device: &vk::PhysicalDevice,
    surface: &vk::SurfaceKHR,
    surface_loader: &ash::khr::surface::Instance,
) -> SwapChainSupportDetails {
    let capabilities = unsafe {
        surface_loader
            .get_physical_device_surface_capabilities(*device, *surface)
            .unwrap()
    };

    let formats = unsafe {
        surface_loader
            .get_physical_device_surface_formats(*device, *surface)
            .unwrap()
    };

    let present_modes = unsafe {
        surface_loader
            .get_physical_device_surface_present_modes(*device, *surface)
            .unwrap()
    };

    SwapChainSupportDetails {
        capabilities,
        formats,
        present_modes,
    }
}

fn choose_swap_surface_format(
    available_formats: &Vec<vk::SurfaceFormatKHR>,
) -> vk::SurfaceFormatKHR {
    for format in available_formats.iter() {
        if format.format == vk::Format::B8G8R8A8_SRGB
            && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        {
            return *format;
        }
    }

    available_formats[0]
}

fn choose_swap_present_mode(
    available_present_modes: &Vec<vk::PresentModeKHR>,
) -> vk::PresentModeKHR {
    for present_mode in available_present_modes.iter() {
        if *present_mode == vk::PresentModeKHR::MAILBOX {
            return *present_mode;
        }
    }

    vk::PresentModeKHR::FIFO
}

fn choose_swap_extent(
    capabilities: &vk::SurfaceCapabilitiesKHR,
    window: &winit::window::Window,
) -> vk::Extent2D {
    if capabilities.current_extent.width != u32::MAX {
        return capabilities.current_extent;
    } else {
        let size = window.inner_size();
        let (width, height) = (size.width, size.height);

        let mut actual_extent = vk::Extent2D {
            width: width,
            height: height,
        };

        actual_extent.width = actual_extent.width.clamp(
            capabilities.min_image_extent.width,
            capabilities.max_image_extent.width,
        );
        actual_extent.height = actual_extent.height.clamp(
            capabilities.min_image_extent.height,
            capabilities.max_image_extent.height,
        );

        actual_extent
    }
}
