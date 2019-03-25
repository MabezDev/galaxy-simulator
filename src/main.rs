
use galaxy_sim::{
    Galaxy,
    GALAXY_WIDTH
};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub const WINDOW_SIZE: u32 = 800;

fn main() {
    println!("Hello, world!");
    let mut galaxy = Galaxy::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let scale = WINDOW_SIZE as f64 / GALAXY_WIDTH;
    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i = 0;
    'running: loop {
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        // now our simulations

        galaxy.compute_delta();

        for star in galaxy.get_stars() {
            let (x, y) = (star.position[0] * scale, star.position[1] * scale);
            canvas.fill_rect(Rect::new(x as i32, y as i32, star.mass as u32, star.mass as u32)).unwrap();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
