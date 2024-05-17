use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::raw_window_handle::HasWindowHandle;
use winit::window::{Theme, Window};

use ash;
use ash::vk;

pub mod engine;
pub mod utils;

struct VulkanApp {
    props: Option<VulkanAppProperties>,
    is_debug_enabled: bool,
}

impl VulkanApp {
    fn new(is_debug_enabled: bool) -> Self {
        VulkanApp {
            props: None,
            is_debug_enabled: is_debug_enabled,
        }
    }

    fn init_vulkan(&mut self, window: Window) {
        let props = VulkanAppProperties::new(window, self.is_debug_enabled);
        self.props = Some(props);
    }

    fn init_window(event_loop: &ActiveEventLoop) -> Window {
        let window_attributes = Window::default_attributes()
            .with_theme(Some(Theme::Dark))
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 800.0))
            .with_title("Vulkan Ray Tracer");
        event_loop.create_window(window_attributes).unwrap()
    }

    fn draw_frame(&mut self) {}
}

struct VulkanAppProperties {
    _window: Window,
    _entry: ash::Entry,
    instance: ash::Instance,
    is_debug_enabled: bool,
    debug_messenger: vk::DebugUtilsMessengerEXT,
    debug_utils_loader: ash::ext::debug_utils::Instance,
    _physical_device: vk::PhysicalDevice,
    logical_device: ash::Device,
    _graphics_queue: vk::Queue,
    _present_queue: vk::Queue,
    _surface_loader: ash::khr::surface::Instance,
    _surface: vk::SurfaceKHR,
    _swap_chain: engine::swap_chain::SwapChain,
}

impl VulkanAppProperties {
    // init_vulkan
    fn new(window: Window, is_debug_enabled: bool) -> Self {
        // Create an instance
        let entry = ash::Entry::linked();
        let instance = engine::instance::create_instance(&entry, is_debug_enabled);

        // Setup the debug manager
        let (debug_utils_loader, debug_messenger) =
            utils::debug::setup_debug_utils(true, &entry, &instance);

        // Create the surface
        let raw_window_handle = window.window_handle().unwrap().as_raw();
        let surface = engine::surface::create_surface(&entry, &instance, &raw_window_handle);
        let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);

        // Create the physical device
        let physical_device =
            engine::physical_device::pick_physical_device(&instance, &surface, &surface_loader);

        // Create the logical device
        let logical_device = engine::logical_device::create_logical_device(
            &physical_device,
            &instance,
            &surface,
            &surface_loader,
        );

        let indices = engine::queue_families::find_queue_families(
            &physical_device,
            &instance,
            &surface,
            &surface_loader,
        );
        let graphics_queue =
            unsafe { logical_device.get_device_queue(indices.graphics_family.unwrap(), 0) };
        let present_queue =
            unsafe { logical_device.get_device_queue(indices.present_family.unwrap(), 0) };

        // Create swap chain and image views
        let swap_chain = engine::swap_chain::SwapChain::new(
            &physical_device,
            &instance,
            &logical_device,
            &surface,
            &surface_loader,
            &window,
        );

        // Create graphics pipeline

        VulkanAppProperties {
            _window: window,
            _entry: entry,
            instance,
            is_debug_enabled: is_debug_enabled,
            debug_messenger,
            debug_utils_loader,
            _physical_device: physical_device,
            logical_device,
            _graphics_queue: graphics_queue,
            _present_queue: present_queue,
            _surface: surface,
            _surface_loader: surface_loader,
            _swap_chain: swap_chain,
        }
    }
}

impl Drop for VulkanAppProperties {
    fn drop(&mut self) {
        unsafe {
            println!("Destroying instance");

            // Logical Device
            // Would be better to call drop but I'm not sure how to do so since
            // self is already &mut
            self._swap_chain.cleanup(&self.logical_device);
            self.logical_device.destroy_device(None);

            if self.is_debug_enabled {
                self.debug_utils_loader
                    .destroy_debug_utils_messenger(self.debug_messenger, None);
            }

            // Physical Device
            self._surface_loader.destroy_surface(self._surface, None);
            self.instance.destroy_instance(None);
        }
    }
}

impl ApplicationHandler for VulkanApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Self::init_window(event_loop);
        self.init_vulkan(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.main_loop(event_loop, event);
    }
}

impl VulkanApp {
    fn main_loop(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                // self.window.as_ref().request_redraw();
                self.draw_frame();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: key,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match key {
                Key::Named(NamedKey::Escape) => event_loop.exit(),
                _ => {}
            },
            _ => {}
        }
    }
}

pub fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut vulkan_app = VulkanApp::new(true);

    let _ = event_loop.run_app(&mut vulkan_app);
}
