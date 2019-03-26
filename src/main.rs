use structopt::StructOpt;
use std::time::Instant;

use galaxy_sim::galaxy::{
    Galaxy,
    GALAXY_WIDTH
};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub const WINDOW_SIZE: u32 = 800;

pub const STAR_COUNT: u64 = 2000;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "Galaxy Simulator")]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    /// Number of stars
    #[structopt(short = "s", long = "stars", default_value = "2000")]
    stars: u64,

    /// Number of iterations to complete before stopping
    #[structopt(short = "i", long = "iter", default_value = "250")]
    iterations: u64,

    /// MODE - `single` or `parallel`
    #[structopt(short = "m", long = "mode", default_value = "parallel")]
    mode: String,

    /// Number of threads - defaults to the number of logical cores on the machine if not specified
    #[structopt(short = "t", long = "threads")]
    threads: Option<usize>,

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

    // allow specicification of the number of used threads
    if let Some(threads) = opt.threads {
        rayon::ThreadPoolBuilder::new().num_threads(threads).build_global().unwrap();
    }

    let mode = match opt.mode.to_lowercase().as_ref() {
        "single" => Mode::Single,
        "parallel" => Mode::Parallel,
        _ => panic!("Invalid --mode option")
    };
    match opt.task.to_lowercase().as_ref() {
        "bench" => {
            let mut galaxy = Galaxy::new(opt.stars);
            bench(&mut galaxy, mode, &opt);
        },
        "visualize" => {
            let mut galaxy = Galaxy::new(opt.stars);
            visualize(&mut galaxy, mode);
            
        }
        _ => panic!("requires a task of either `bench` or `visualize`")
    }
    
}

fn bench(galaxy: &mut Galaxy, mode: Mode, opt: &Opt) {
    let start = Instant::now();
    println!("Running benchmark...");
    for _ in 0..opt.iterations {
        match mode {
            Mode::Single => galaxy.compute_iter(),
            Mode::Parallel => galaxy.par_compute_iter()
        };
    }
    let end = Instant::now();

    println!("Time to complete: {}ms", end.duration_since(start).as_millis())
}

fn visualize(galaxy: &mut Galaxy, mode: Mode) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut mode = mode;
    // let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        // .opengl()
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
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    mode = Mode::Parallel
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    mode = Mode::Single
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
    }
}
