use std::sync::Arc;

use pollster::FutureExt;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
};

pub struct Wgpu<'a> {
    surface: wgpu::Surface<'a>,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

pub struct App<'a> {
    window: Option<Arc<winit::window::Window>>,
    wgpu: Option<Wgpu<'a>>,
}

impl<'a> Wgpu<'a> {
    pub fn new(window: Arc<Window>) -> Wgpu<'a> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                compatible_surface: Some(&surface),
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
            })
            .block_on()
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::Performance,
                    ..Default::default()
                },
                None,
            )
            .block_on()
            .unwrap();

        let capabilites = surface.get_capabilities(&adapter);
        let format = capabilites.formats[0];
        let config = wgpu::SurfaceConfiguration {
            width: size.width,
            height: size.height,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            desired_maximum_frame_latency: 2,
            format,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        Wgpu {
            surface,
            surface_config: config,
            device,
            queue,
        }
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window = Arc::new(
                event_loop
                    .create_window(WindowAttributes::default())
                    .unwrap(),
            );
            self.wgpu = Some(Wgpu::new(window.clone()));
            self.window = Some(window);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.wgpu = None;
                self.window = None;
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().pre_present_notify();
            }
            WindowEvent::Resized(size) => {
                if size.width == 0 || size.height == 0 {
                    return;
                }
                let _wgpu = self.wgpu.as_mut().unwrap();
                _wgpu.surface_config.width = size.width;
                _wgpu.surface_config.height = size.height;
                _wgpu
                    .surface
                    .configure(&_wgpu.device, &_wgpu.surface_config);
            }
            _ => (),
        }
    }
}

fn main() {
    let mut app = App {
        window: None,
        wgpu: None,
    };

    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut app).unwrap();
}
