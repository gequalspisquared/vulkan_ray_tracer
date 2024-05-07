use std::ffi::c_void;

use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Theme, Window};

use ash;
use ash::vk;

pub mod utils;

struct VulkanApp {
    window: Option<Window>,
    _entry: ash::Entry,
    instance: ash::Instance,
    is_debug_enabled: bool,
    debug_messenger: vk::DebugUtilsMessengerEXT,
    debug_utils_loader: ash::ext::debug_utils::Instance,
}

impl VulkanApp {
    pub fn new(is_debug_enabled: bool) -> VulkanApp {
        let entry = ash::Entry::linked();
        let instance = Self::create_instance(&entry, is_debug_enabled);

        let (debug_utils_loader, debug_messenger) =
            utils::debug::setup_debug_utils(true, &entry, &instance);

        VulkanApp {
            window: None,
            _entry: entry,
            instance,
            is_debug_enabled,
            debug_messenger,
            debug_utils_loader,
        }
    }

    fn create_instance(entry: &ash::Entry, is_debug_enabled: bool) -> ash::Instance {
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
            p_next: &debug_create_info as *const vk::DebugUtilsMessengerCreateInfoEXT
                as *const c_void,
            ..Default::default()
        };

        unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create instance!")
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
            if self.is_debug_enabled {
                self.debug_utils_loader
                    .destroy_debug_utils_messenger(self.debug_messenger, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut vulkan_app = VulkanApp::new(true);

    let _ = event_loop.run_app(&mut vulkan_app);
}
