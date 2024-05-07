use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Theme, Window};

use ash;
use ash::vk;

pub mod engine;
pub mod utils;

struct VulkanApp {
    window: Option<Window>,
    _entry: ash::Entry,
    instance: ash::Instance,
    is_debug_enabled: bool,
    debug_messenger: vk::DebugUtilsMessengerEXT,
    debug_utils_loader: ash::ext::debug_utils::Instance,
    _physical_device: vk::PhysicalDevice,
    logical_device: ash::Device,
    _graphics_queue: vk::Queue,
}

impl VulkanApp {
    pub fn new(is_debug_enabled: bool) -> VulkanApp {
        let entry = ash::Entry::linked();
        let instance = engine::instance::create_instance(&entry, is_debug_enabled);

        let (debug_utils_loader, debug_messenger) =
            utils::debug::setup_debug_utils(true, &entry, &instance);

        let physical_device = engine::physical_device::pick_physical_device(&instance);
        let logical_device =
            engine::logical_device::create_logical_device(&physical_device, &instance);

        let indices = engine::queue_families::find_queue_families(&physical_device, &instance);
        let graphics_queue =
            unsafe { logical_device.get_device_queue(0, indices.graphics_family.unwrap()) };

        VulkanApp {
            window: None,
            _entry: entry,
            instance,
            is_debug_enabled,
            debug_messenger,
            debug_utils_loader,
            _physical_device: physical_device,
            logical_device,
            _graphics_queue: graphics_queue,
        }
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

impl ApplicationHandler for VulkanApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(Self::init_window(event_loop));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.main_loop(event_loop, event)
    }
}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        unsafe {
            println!("Destroying instance");
            self.logical_device.destroy_device(None);
            if self.is_debug_enabled {
                self.debug_utils_loader
                    .destroy_debug_utils_messenger(self.debug_messenger, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}

impl VulkanApp {
    fn main_loop(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                // self.window.as_ref().unwrap().request_redraw();
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

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut vulkan_app = VulkanApp::new(true);

    let _ = event_loop.run_app(&mut vulkan_app);
}
