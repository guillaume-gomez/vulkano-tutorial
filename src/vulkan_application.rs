extern crate winit;
extern crate vulkano_win;
extern crate vulkano;

use std::sync::Arc;

use winit::{EventsLoop, WindowBuilder, dpi::LogicalSize};

use vulkano::instance::{
    Instance,
    InstanceExtensions,
    ApplicationInfo,
    Version,
    PhysicalDevice,
    layers_list,
};
use vulkano::instance::debug::{DebugCallback, MessageTypes};


const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const VALIDATION_LAYERS: &[&str] =  &[
    "VK_LAYER_LUNARG_standard_validation"
];

#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;

struct QueueFamilyIndices {
    graphics_family: i32,
}
impl QueueFamilyIndices {
    fn new() -> Self {
        Self { graphics_family: -1 }
    }

    fn is_complete(&self) -> bool {
        self.graphics_family >= 0
    }
}


pub struct VulkanApplication {
    instance: Arc<Instance>,
    debug_callback: Option<DebugCallback>,
    events_loop: EventsLoop,
    physical_device_index: usize,
}

impl VulkanApplication {
    pub fn initialize() -> Self {
        let events_loop = Self::init_window();
        let instance = Self::create_instance();
        let physical_device_index = Self::pick_physical_device(&instance);
        let debug_callback = Self::setup_debug_callback(&instance);

        Self {
            instance,
            events_loop,
            debug_callback,
            physical_device_index
        }
    }

    fn create_instance() -> Arc<Instance> {
        if ENABLE_VALIDATION_LAYERS && !Self::check_validation_layer_support() {
            println!("Validation layers requested, but not available!")
        }

        let supported_extensions = InstanceExtensions::supported_by_core()
            .expect("failed to retrieve supported extensions");
        println!("Supported extensions: {:?}", supported_extensions);

        let app_info = ApplicationInfo {
            application_name: Some("Hello Triangle".into()),
            application_version: Some(Version { major: 1, minor: 0, patch: 0 }),
            engine_name: Some("No Engine".into()),
            engine_version: Some(Version { major: 1, minor: 0, patch: 0 }),
        };

        let required_extensions = Self::get_required_extensions();

        if ENABLE_VALIDATION_LAYERS && Self::check_validation_layer_support() {
            Instance::new(Some(&app_info), &required_extensions, VALIDATION_LAYERS.iter().cloned())
                .expect("failed to create Vulkan instance")
        } else {
            Instance::new(Some(&app_info), &required_extensions, None)
                .expect("failed to create Vulkan instance")
        }

    }

    fn init_window() -> EventsLoop {
        let events_loop = EventsLoop::new();
        let _window = WindowBuilder::new()
            .with_title("Vulkan")
            .with_dimensions(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))
            .build(&events_loop);
        events_loop
    }

    fn check_validation_layer_support() -> bool {
        let layers: Vec<_> = layers_list().unwrap().map(|l| l.name().to_owned()).collect();
        VALIDATION_LAYERS.iter()
            .all(|layer_name| layers.contains(&layer_name.to_string()))
    }

    fn get_required_extensions() -> InstanceExtensions {
        let mut extensions = vulkano_win::required_extensions();
        if ENABLE_VALIDATION_LAYERS {
            // TODO!: this should be ext_debug_utils (_report is deprecated), but that doesn't exist yet in vulkano
            extensions.ext_debug_report = true;
        }

        extensions
    }

    fn setup_debug_callback(instance: &Arc<Instance>) -> Option<DebugCallback> {
        if !ENABLE_VALIDATION_LAYERS  {
            return None;
        }

        let msg_types = MessageTypes {
            error: true,
            warning: true,
            performance_warning: true,
            information: false,
            debug: true,
        };
        DebugCallback::new(&instance, msg_types, |msg| {
            println!("validation layer: {:?}", msg.description);
        }).ok()
    }

    fn pick_physical_device(instance: &Arc<Instance>) -> usize {
        PhysicalDevice::enumerate(&instance)
            .position(|device| Self::is_device_suitable(&device))
            .expect("failed to find a suitable GPU!")
    }

    fn is_device_suitable(device: &PhysicalDevice) -> bool {
        let indices = Self::find_queue_families(device);
        indices.is_complete()
    }

    fn find_queue_families(device: &PhysicalDevice) -> QueueFamilyIndices {
        let mut indices = QueueFamilyIndices::new();
        // TODO: replace index with id to simplify?
        for (i, queue_family) in device.queue_families().enumerate() {
            if queue_family.supports_graphics() {
                indices.graphics_family = i as i32;
            }

            if indices.is_complete() {
                break;
            }
        }

        indices
    }

    pub fn main_loop(&mut self) {
        self.events_loop.run_forever(|event| {
            println!("{:?}", event);

            match event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::CloseRequested,
                    ..
                } => winit::ControlFlow::Break,
                _ => winit::ControlFlow::Continue,
            }
        });
    }
}

