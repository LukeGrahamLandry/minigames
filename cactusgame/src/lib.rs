use std::alloc;
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
use std::alloc::Layout;
use std::time::{SystemTime, UNIX_EPOCH};
use save::Template;
use crate::level::{default_level, Entity, EntityInfo, Level, Texture};
use crate::math::{AABB, Vec2};
use crate::save::{read_level, write_level};
use crate::templates::TEMPLATES;

pub mod level;
pub mod physics;
pub mod math;
pub mod save;
pub mod templates;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        #[cfg(target_arch = "wasm32")]
        {
            let msg = format!($($arg)*);
            unsafe { $crate::console_log(msg.as_ptr(), msg.len()); }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            println!($($arg)*);
        }
    }};
}

#[no_mangle]
extern "C" fn start_level() -> *mut Level {
    let mut level = Box::new(read_level(LEVELS[0]).unwrap());
    // let mut level = Box::new(default_level());
    level.last_time = get_time();
    Box::into_raw(level)
}

pub const LEVELS: &[&[u8]] = &[
    include_bytes!("../web/assets/levels/first_test.cactus")
];

// TODO: don't rerender walls every frame. and have adjacent squares share a bigger rectangular aabb.
#[no_mangle]
extern "C" fn render(level: &mut Level) {
    level.frame_update();
    for e in &level.entities {
        if matches!(e.info, EntityInfo::Deleted) {
            continue;
        }
        let pos = level.physics.things[e.id].shape.position();
        unsafe { draw(e.texture, pos.x, pos.y, 1.0) };
    }

    unsafe {
        if GHOST.enable {
            draw(GHOST.template.texture, GHOST.pos.x, GHOST.pos.y, 0.3);
        }
    }
}

#[no_mangle]
extern "C" fn update_key(level: &mut Level, key: u16, pressed: bool) {
    match char::from_u32(key as u32).unwrap() {
        'a' => level.input.left = pressed,
        'd' => level.input.right = pressed,
        'w' => level.input.up = pressed,
        _ => {}
    }
}

#[no_mangle]
extern "C" fn load_level(level: &mut Level, ptr: *const u8, len: usize) -> bool {
    let data = unsafe { &*slice_from_raw_parts(ptr, len) };
    match read_level(data) {
        Ok(new_level) => {
            *level = new_level;
            true
        }
        Err(err) => {
            log!("{:?}", err);
            false
        }
    }
}

#[no_mangle]
extern "C" fn save_level(level: &Level, ptr: *mut u8, max_len: usize) -> usize {
    let data = write_level(level);
    if data.len() > max_len {
        return 0;
    }

    let output = slice_from_raw_parts_mut(ptr, data.len());
    unsafe { output.as_mut().unwrap().copy_from_slice(&data) };
    data.len()
}

#[no_mangle]
extern "C" fn alloc(len: usize) -> *const u8 {
    let layout = Layout::array::<u8>(len).unwrap();
    unsafe { alloc::alloc(layout) }
}

// Can't call this free if included when running tests or the test runner has a stack overflow... that's interesting.
// Am I really overriding the libc one? Why doesn't the compiler catch that?
#[no_mangle]
extern "C" fn drop(ptr: *mut u8, len: usize) {
    let layout = Layout::array::<u8>(len).unwrap();
    unsafe { alloc::dealloc(ptr, layout) }
}

#[no_mangle]
extern "C" fn reset_time(level: &mut Level) {
    level.last_time = get_time();
}

#[cfg(target_arch = "wasm32")]
pub fn get_time() -> f32 {
    let ms = unsafe { performance_now() };
    (ms / 1000.0) as f32
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_time() -> f32 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f32()
}

extern "C" {
    fn random() -> f64;
    fn console_log(ptr: *const u8, len: usize);
    fn draw(texture: Texture, x: f32, y: f32, alpha: f32);
    fn performance_now() -> f64;
}

#[no_mangle]
extern "C" fn place_thing(level: &mut Level, x: usize, y: usize, type_index: usize) {
    if type_index == 0 {
        let aabb = AABB::square(Vec2::new(x as f32, y as f32), 50.0);
        for i in 0..level.entities.len() {
            let hit = level.physics.things[i].shape.get_aabb().collides_aabb(&aabb);
            if hit {
                level.kill(i);
            }
        }
    } else {
        let template = &TEMPLATES[type_index];
        level.add(template, Vec2::new(x as f32, y as f32));
    }
}

static mut GHOST: EditorGhost = EditorGhost { pos: Vec2::ZERO, template: &TEMPLATES[0], enable: false };

#[no_mangle]
extern "C" fn editor_mouse_move(x: usize, y: usize, type_index: usize) {
    unsafe {
        GHOST.enable = true;
        GHOST.template = &TEMPLATES[type_index];
        GHOST.pos = Vec2::new(x as f32, y as f32);
    }
}

struct EditorGhost {
    pos: Vec2,
    template: &'static Template,
    enable: bool,
}
