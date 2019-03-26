#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

use galaxy_sim::galaxy::{
    Galaxy,
    GALAXY_WIDTH
};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub const WINDOW_SIZE: u32 = 800;

pub const STAR_COUNT: u64 = 2000;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "Galaxy Simulator")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag.
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    /// Number of stars
    #[structopt(short = "s", long = "stars", default_value = "2000")]
    stars: u64,

    /// Number of iterations to complete before stopping
    #[structopt(short = "i", long = "iter", default_value = "10000")]
    iterations: u64,

    /// MODE - `single` or `parallel`
    #[structopt(short = "m", long = "mode", default_value = "single")]
    mode: String,

    /// Task - `bench` or `visualize`
    #[structopt(name = "TASK")]
    task: String,

}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Mode {
    Single,
    Parallel
}

fn main() {
    let opt = Opt::from_args();
    if opt.debug {
        println!("{:?}", opt);
    }
    let mode = match opt.mode.to_lowercase().as_ref() {
        "single" => Mode::Single,
        "parallel" => Mode::Parallel,
        _ => panic!("Invalid --mode option")
    };
    match opt.task.to_lowercase().as_ref() {
        "bench" => {
            // TODO bench code here
            let mut _galaxy = Galaxy::new(opt.stars);
            match mode {
                Mode::Single => {},
                Mode::Parallel => {}
            }
        },
        "visualize" => {
            let mut galaxy = Galaxy::new(opt.stars);
            visualize(&mut galaxy, mode);
            
        }
        _ => panic!("requires a task of either `bench` or `visualize`")
    }
    
}


fn visualize(galaxy: &mut Galaxy, mode: Mode) {
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
        let stars = match mode {
            Mode::Single => galaxy.compute_iter(),
            Mode::Parallel => galaxy.par_compute_iter()
        };

        for star in stars {
            let (x, y) = (star.position[0] * scale, star.position[1] * scale);
            canvas.fill_rect(Rect::new(x as i32, y as i32, star.mass as u32, star.mass as u32)).unwrap();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
