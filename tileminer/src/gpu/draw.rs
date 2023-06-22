use crate::gpu::window::WindowContext;
use glam::{Vec2, Vec4};
use std::mem::size_of;
use std::rc::Rc;
use wgpu::{Buffer, BufferUsages, RenderPass};

pub struct Drawing {
    ctx: Rc<WindowContext>,
    vertices: Vec<Vertex>,
    pub buffer: Buffer,
    pub count: u32,
}

#[repr(C)]
pub struct Vertex {
    colour: [f32; 4],
    pos: Vec2,
}

impl Drawing {
    pub fn new(ctx: Rc<WindowContext>) -> Drawing {
        Drawing {
            buffer: ctx.buffer_init(
                "drawing_vertices",
                &[0; 10000],
                BufferUsages::VERTEX | BufferUsages::COPY_DST,
            ),
            ctx,
            vertices: vec![],
            count: 0,
        }
    }

    pub fn start_frame(&mut self) {
        self.count = 0;
        assert!(self.vertices.is_empty());
    }

    pub fn rect(&mut self, mut top_left: Vec2, width: f32, height: f32, colour: Colour) {
        assert!(width > 0.0 && height > 0.0);
        let bottom_right = top_left + Vec2::new(width, height);
        let top_right = top_left + Vec2::new(width, 0.0);
        let bottom_left = top_left + Vec2::new(0.0, height);
        let points = [
            top_right,
            top_left,
            bottom_left,
            top_right,
            bottom_right,
            bottom_left,
        ];
        self.vertices.extend(points.into_iter().map(|pos| Vertex {
            pos,
            colour: colour.to_array(),
        }));
    }

    // TODO: keep the tile map vertices in a separate buffer that only gets updated when the world changes.
    //       and do the camera offset in the shader so they dont change based on player movement.
    pub fn write_buffer(&mut self) {
        let needed = (self.vertices.len() * size_of::<Vertex>()) as u64;
        let data = slice_to_bytes(&self.vertices);
        if needed > self.buffer.size() {
            self.buffer = self.ctx.buffer_init(
                "drawing_vertices",
                data,
                BufferUsages::VERTEX | BufferUsages::COPY_DST,
            );
        } else {
            self.ctx.write_buffer(&self.buffer, data);
        }
        self.count = self.vertices.len() as u32;
        self.vertices.clear(); // Keeps capacity so generally won't reallocate next frame.
    }
}

fn slice_to_bytes<T: Sized>(p: &[T]) -> &[u8] {
    unsafe {
        core::slice::from_raw_parts(
            (p as *const [T]) as *const u8,
            core::mem::size_of::<T>() * p.len(),
        )
    }
}

pub type Colour = Vec4;

// TODO: these are probably wrong << 16 | (0xFF) for alpha
pub const BLACK: Colour = Vec4::new(0.0, 0.0, 0.0, 1.0);
pub const RED: Colour = Vec4::new(1.0, 0.0, 0.0, 1.0);
pub const BLUE: Colour = Vec4::new(0.0, 0.0, 1.0, 1.0);
pub const LIGHT_GRAY: Colour = Vec4::new(0.3, 0.3, 0.3, 1.0);
pub const ORANGE: Colour = Vec4::new(1.0, 0.2, 0.0, 1.0);
pub const BROWN: Colour = Vec4::new(0.6, 0.35, 0.2, 1.0);
pub const GREEN: Colour = Vec4::new(0.0, 1.0, 0.0, 1.0);
pub const WHITE: Colour = Vec4::new(1.0, 1.0, 1.0, 1.0);
