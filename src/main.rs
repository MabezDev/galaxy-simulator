
use galaxy_sim::Galaxy;

fn main() {
    println!("Hello, world!");
    let mut galaxy = Galaxy::new();

    loop {
        galaxy.compute_delta();
    }
}
