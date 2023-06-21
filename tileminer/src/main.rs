use crate::gen::starting_level;
use crate::gpu::draw::{Drawing, Vertex};
use crate::gpu::window::{App, WindowContext};
use crate::level::{EntityID, Level, TileType};
use crate::render::{render_level, ScreenTransform};
use glam::Vec2;
use std::rc::Rc;
use wgpu::{BufferBindingType, RenderPipeline, ShaderStages, SurfaceError};
use winit::dpi::PhysicalSize;
use winit::event::{
    DeviceEvent, ElementState, ModifiersState, MouseButton, MouseScrollDelta, VirtualKeyCode,
    WindowEvent,
};

mod gen;
mod gpu;
mod level;
mod player;
pub mod render;

fn main() {
    env_logger::init();
    pollster::block_on(WindowContext::run(Game::new));
}

pub struct Game {
    level: Level,
    player_id: EntityID,
    ctx: Rc<WindowContext>,
    left: bool,
    right: bool,
    draw: Drawing,
    pipeline: RenderPipeline,
}

impl App for Game {
    fn new(ctx: Rc<WindowContext>) -> Self {
        let (level, player_id) = starting_level();
        let pipeline_layout = ctx.pipeline_layout(&[]);
        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4],
        };
        let pipeline = ctx.render_pipeline(
            "drawing",
            &pipeline_layout,
            &[vertex_layout],
            include_str!("shader.wgsl"),
        );

        Game {
            level,
            player_id,
            draw: Drawing::new(ctx.clone()),
            ctx,
            left: false,
            right: false,
            pipeline,
        }
    }

    fn handle_window_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::MouseWheel { delta, .. } => {
                if let MouseScrollDelta::PixelDelta(pos) = delta {
                    self.level.view_scale =
                        (self.level.view_scale + (pos.y * 0.01) as f32).clamp(0.25, 100.0);
                } else {
                    todo!();
                }
            }
            WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                Some(VirtualKeyCode::W) => {
                    let player = self.level.entities.get_mut(&self.player_id).unwrap();
                    const JUMP: f32 = 10.0;
                    if player.is_on_ground {
                        player.velocity.y -= JUMP;
                    }
                }
                Some(VirtualKeyCode::A) => {
                    self.left = input.state == ElementState::Pressed;
                }
                Some(VirtualKeyCode::D) => {
                    self.right = input.state == ElementState::Pressed;
                }
                _ => {}
            },
            WindowEvent::CursorMoved { position, .. } => {
                let mouse_pos_screen = Vec2::new(position.x as f32, position.y as f32);
                let cam = ScreenTransform::new(&self.level, self.player_id, self.ctx.window_size());
                let mouse_world_pos = cam.pixel_to_tile(mouse_pos_screen);
                self.level.mouse_pos = (
                    mouse_world_pos.0.clamp(0, self.level.tiles.size - 1),
                    mouse_world_pos.1.clamp(0, self.level.tiles.size - 1),
                );
            }
            WindowEvent::MouseInput {
                state,
                button,
                modifiers,
                ..
            } => {
                if *state == ElementState::Pressed {
                    match button {
                        MouseButton::Left => {
                            self.level.set(
                                self.level.mouse_pos.0,
                                self.level.mouse_pos.1,
                                TileType::Empty,
                            );
                        }
                        MouseButton::Right => {
                            let tile = if modifiers.contains(ModifiersState::SHIFT) {
                                TileType::Sand
                            } else {
                                TileType::Dirt
                            };
                            self.level
                                .set(self.level.mouse_pos.0, self.level.mouse_pos.1, tile);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        };
        false
    }

    fn handle_device_event(&mut self, event: &DeviceEvent) {}

    fn update(&mut self) {
        const SPEED: f32 = 10.0;
        let player = self.level.entities.get_mut(&self.player_id).unwrap();
        if self.left {
            player.velocity.x = -SPEED;
        }
        if self.right {
            player.velocity.x = SPEED;
        }

        self.level.update(self.ctx.timer.borrow().delta());
    }

    fn render(&mut self) -> Result<(), SurfaceError> {
        let mut encoder = self.ctx.command_encoder("gpu");

        let output = self.ctx.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut render_pass = self.ctx.render_pass(&mut encoder, &view);
            render_pass.set_pipeline(&self.pipeline);
            self.draw.start_frame();
            render_level(
                &self.level,
                self.player_id,
                &mut self.draw,
                self.ctx.window_size(),
            );
            self.draw.write_buffer();
            // This slice may include invalid vertices from previous frames but the only the correct range is drawn below.
            render_pass.set_vertex_buffer(0, self.draw.buffer.slice(..));
            render_pass.draw(0..self.draw.count, 0..1);
        }

        self.ctx.queue.submit([encoder.finish()].into_iter());

        Ok(())
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {}
}
