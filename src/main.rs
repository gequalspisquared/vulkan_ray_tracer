use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::raw_window_handle::HasDisplayHandle;
use winit::window::{Theme, Window};

use ash::{vk, Entry};
use ash_window;

struct App {
    window: Option<Window>,
    entry: Entry,
    instance: ash::Instance,
}

impl App {
    unsafe fn new() -> App {
        let (entry, instance) = unsafe { Self::init_vulkan() };

        let extensions = unsafe { entry.enumerate_instance_extension_properties(None).unwrap() };
        println!("Extensions count: {}", extensions.len()); // 19 on my machine

        App {
            window: None,
            entry,
            instance,
        }
    }

    unsafe fn init_vulkan() -> (Entry, ash::Instance) {
        let entry = Entry::linked();
        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 0, 0),
            ..Default::default()
        };

        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            ..Default::default()
        };

        let instance = unsafe { entry.create_instance(&create_info, None).unwrap() };

        (entry, instance)
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_theme(Some(Theme::Dark))
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 800.0))
            .with_title("Vulkan Ray Tracer");
        self.window = Some(event_loop.create_window(window_attributes).unwrap());

        let extensions = ash_window::enumerate_required_extensions(
            self.window
                .as_ref()
                .unwrap()
                .display_handle()
                .unwrap()
                .as_raw(),
        )
        .unwrap();

        println!("Required extensions count: {}", extensions.len());

        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 0, 0),
            ..Default::default()
        };
        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            pp_enabled_extension_names: extensions.as_ptr(),
            ..Default::default()
        };
        self.instance = unsafe { self.entry.create_instance(&create_info, None).unwrap() };
    }

    // main loop
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Closing window...");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
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
                Key::Named(NamedKey::Escape) => {
                    event_loop.exit();
                }
                _ => (),
            },
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = unsafe { App::new() };
    let _ = event_loop.run_app(&mut app);
}
