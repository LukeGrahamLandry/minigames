use glam::Vec2;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use wgpu::PresentMode;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{CursorGrabMode, Window, WindowBuilder};

pub struct WindowContext {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: RefCell<wgpu::SurfaceConfiguration>,
    pub size: RefCell<PhysicalSize<u32>>,
    pub window: Window,
    pub timer: RefCell<FrameTimer>,
}

pub struct FrameTimer {
    pub frame_count: i32,
    pub micro_seconds: u128,
    pub last: Instant,
}

impl FrameTimer {
    pub fn new() -> Self {
        FrameTimer {
            frame_count: 0,
            micro_seconds: 0,
            last: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.micro_seconds += self.last.elapsed().as_micros();
        self.last = now;
        self.frame_count += 1;

        if self.micro_seconds > 5000000 {
            self.reset();
        }
    }

    pub fn reset(&mut self) {
        let seconds = self.micro_seconds as f64 / 1000000.0;
        let frame_time_ms = (self.micro_seconds as f64 / self.frame_count as f64).round() / 1000.0;
        let fps = self.frame_count as f64 / seconds;
        println!(
            "{} seconds; {} frames; {} fps; {} ms per frame;",
            seconds,
            self.frame_count,
            fps.round(),
            frame_time_ms
        );
        self.micro_seconds = 0;
        self.frame_count = 0;
    }
}

pub trait App {
    fn new(ctx: Rc<WindowContext>) -> Self;
    /// Return false to catch the event and prevent further handler (ie. make ESC not quit the window)
    fn handle_window_event(&mut self, event: &WindowEvent) -> bool;
    // winit::event::WindowEvent::CursorMoved warns against using it for 3D camera control so now we have two event callbacks
    fn handle_device_event(&mut self, event: &DeviceEvent);
    /// Just called before gpu. But there's a semantic separation and I think it's nice to put it in the same impl block.
    fn update(&mut self);
    fn render(&mut self) -> Result<(), wgpu::SurfaceError>;
    fn resize(&mut self, new_size: PhysicalSize<u32>);
}

impl WindowContext {
    async fn new() -> (Rc<WindowContext>, EventLoop<()>) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        // window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
        // window.set_cursor_visible(false);

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        (
            Rc::new(WindowContext {
                window,
                surface,
                device,
                queue,
                config: RefCell::new(config),
                size: RefCell::new(size),
                timer: RefCell::new(FrameTimer::new()),
            }),
            event_loop,
        )
    }

    pub async fn run<A, F>(constructor: F)
    where
        A: App + 'static,
        F: FnOnce(Rc<WindowContext>) -> A,
    {
        let (ctx, event_loop) = WindowContext::new().await;
        println!("Initializing...");
        let start = Instant::now();
        let mut app = constructor(ctx.clone());
        let mut vsync_on = true;
        let end = Instant::now();
        println!(
            "Initialized in {} ms; present_mode={:?}",
            (end - start).as_millis(),
            ctx.config.borrow().present_mode
        );
        event_loop.run(move |event, _, control_flow| match event {
            Event::DeviceEvent { ref event, .. } => {
                app.handle_device_event(event);
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == ctx.window.id() => {
                if !app.handle_window_event(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => app.resize(*physical_size),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            app.resize(**new_inner_size)
                        }
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::V),
                                    ..
                                },
                            ..
                        } => {
                            vsync_on = !vsync_on;
                            ctx.config.borrow_mut().present_mode = if vsync_on {
                                PresentMode::AutoVsync
                            } else {
                                PresentMode::AutoNoVsync
                            };
                            let size = { *ctx.size.borrow() };
                            ctx.resize(&size);
                            ctx.timer.borrow_mut().reset();
                            println!("set present_mode={:?}", ctx.config.borrow().present_mode);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == ctx.window.id() => {
                app.update();
                match app.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => {
                        let size = *ctx.size.borrow();
                        app.resize(size);
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
                ctx.timer.borrow_mut().update();
            }
            Event::MainEventsCleared => {
                ctx.window.request_redraw();
            }
            _ => {}
        });
    }

    pub fn resize(&self, new_size: &PhysicalSize<u32>) -> bool {
        if new_size.width > 0 && new_size.height > 0 {
            *self.size.borrow_mut() = *new_size;
            let mut config = self.config.borrow_mut();
            config.width = new_size.width;
            config.height = new_size.height;
            self.surface.configure(&self.device, &config);
            true
        } else {
            false
        }
    }

    pub fn window_size(&self) -> Vec2 {
        let size = self.window.inner_size();
        // .to_logical::<f32>(self.window.scale_factor());
        Vec2::new(size.width as f32, size.height as f32)
    }
}
