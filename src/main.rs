extern crate piston;

extern "C" {
    #[allow(dead_code)]
    fn puts(ptr: *const u8, len: usize);
}

fn putstr(s: &str) {
    unsafe {
        puts(s.as_ptr(), s.len());
    }
}

use piston::window;
use piston::event_loop::*;

fn main() {}

#[no_mangle]
pub fn game_loop() {
    let event_settings = EventSettings {
        max_fps: 60,
        ups: 2,
        ups_reset: 0,
        swap_buffers: true,
        bench_mode: false,
        lazy: false,
    };

    let window_settings = window::WindowSettings::new("Test", window::Size{ width: 0, height: 0});
    let mut window = window::NoWindow::new(&window_settings);

    let mut events = Events::new(event_settings);

    while let Some(_) = events.next(&mut window) {
        putstr("Hello");
    }
}
