

use nalgebra::base::Vector3;
use rand::prelude::*;
use rayon::prelude::*;

pub const STAR_COUNT: u64 = 2000;
pub const GALAXY_WIDTH: f64 = 100.0;
pub const WORK: usize = 250;

pub const SPHERE_RADIUS: f64 = 20.0;
pub const ANGULAR_VELOCITY: f64 = 0.4;
pub const DELTA_TIME: f64 = 0.002; 
pub const DELTA_TIME_HALF: f64 = 0.002 / 2.0; 
pub const DELTA_TIME_SQUARED_HALF: f64 = (DELTA_TIME * DELTA_TIME) / 2.0; 

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Star {
    pub position : Vector3<f64>,
    pub velocity : Vector3<f64>,
    pub acceleration : Vector3<f64>,
}

impl Star {

    pub fn new() -> Self {
        Star {
            position: Vector3::new(0.0,0.0,0.0),
            velocity: Vector3::new(0.0,0.0,0.0),
            acceleration: Vector3::new(0.0,0.0,0.0),
        }
    }

    pub fn with_position(self, vector: Vector3<f64>) -> Self {
        Self {
            position: vector,
            .. self
        }
    }
    pub fn with_velocities(self, vector: Vector3<f64>) -> Self {
        Self {
            velocity: vector,
            .. self
        }
    }
    pub fn with_accelerations(self, vector: Vector3<f64>) -> Self {
        Self {
            acceleration: vector,
            .. self
        }
    }
}

pub struct Galaxy {
    stars: Vec<Star>,
}

impl Galaxy {

    pub fn new() -> Self {
        let mut stars = Vec::with_capacity(STAR_COUNT as usize);
        // Randomly choose plane for net angular velocity
        let mut rng = thread_rng();

        // let mut n = Vector3::new(
        //     2.0 * rng.gen_range(0.0, 0.1) - 1.0,
        //     2.0 * rng.gen_range(0.0, 0.1) - 1.0,
        //     2.0 * rng.gen_range(0.0, 0.1) - 1.0,
        // );
        // let norm = 1.0 / (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        // n[0] *= norm;
        // n[1] *= norm;
        // n[2] *= norm;

        // just rotate in x, y plane
        let n = Vector3::new(0.0, 0.0, 1.0);

        for _ in 0..STAR_COUNT {

            let rv = loop {
                let rv = Vector3::<f64>::new(
                    rng.gen_range(-SPHERE_RADIUS, SPHERE_RADIUS),
                    rng.gen_range(-SPHERE_RADIUS, SPHERE_RADIUS),
                    rng.gen_range(-SPHERE_RADIUS, SPHERE_RADIUS),
                );
                let r = (rv[0].powf(2.0) + rv[1].powf(2.0) + rv[2].powf(2.0)).sqrt();
                if r < SPHERE_RADIUS {
                    break rv;
                }
            };

            let new_vel = Vector3::new(
                ANGULAR_VELOCITY * (n[1] * rv[2] - n[2] * rv[1]),
                ANGULAR_VELOCITY * (n[2] * rv[0] - n[0] * rv[2]),
                ANGULAR_VELOCITY * (n[0] * rv[1] - n[1] * rv[0]),
            );

            let new_pos = Vector3::new(
                0.5 * GALAXY_WIDTH + rv[0],
                0.5 * GALAXY_WIDTH + rv[1],
                0.5 * GALAXY_WIDTH + rv[2],
            );
            let star = Star::new()
                .with_position(new_pos)
                .with_velocities(new_vel);
            stars.push(star);
        }

        Galaxy {
            stars: stars
        }
    }

    /// Services the galaxy for one complete iteration
    pub fn compute_delta(&mut self) {
        // Verlet integration:
        // http://en.wikipedia.org/wiki/Verlet_integration#Velocity_Verlet

        for star in self.stars.iter_mut() {
            star.position = Vector3::new(
                star.position[0] + (star.velocity[0] * DELTA_TIME) + (star.acceleration[0] * DELTA_TIME_SQUARED_HALF),
                star.position[1] + (star.velocity[1] * DELTA_TIME) + (star.acceleration[1] * DELTA_TIME_SQUARED_HALF),
                star.position[2] + (star.velocity[2] * DELTA_TIME) + (star.acceleration[2] * DELTA_TIME_SQUARED_HALF),
            );
        }

        self.compute_accelerations();

        // we have calculated new accelerations - update the velocities
        for star in self.stars.iter_mut() {
            star.velocity = Vector3::new(
                star.velocity[0] + (star.acceleration[0] * DELTA_TIME_HALF),
                star.velocity[1] + (star.acceleration[1] * DELTA_TIME_HALF),
                star.velocity[2] + (star.acceleration[2] * DELTA_TIME_HALF),
            );
        }

    }

    fn compute_accelerations(&mut self) {
        for star in self.stars.iter_mut() {
            star.acceleration = Vector3::zeros();
        }

        // Interaction forces (gravity)
        // This is where the program spends most of its time.

        // (NOTE: use of Newton's 3rd law below to essentially half number
        // of calculations needs some care in a parallel version.
        // A naive decomposition on the i loop can lead to a race condition
        // because you are assigning to ax[j], etc.
        // You can remove these assignments and extend the j loop to a fixed
        // upper bound of N, or, for extra credit, find a cleverer solution!)

        let works: Vec<_> = self.stars.par_chunks(WORK).collect();
        for stars in works {
            for i in 0..stars.len() {
                for j in 0..i {
                    // Vector version of inverse square law
                    let dp = self.stars[i].position - self.stars[j].position;
                    let dp2 = dp.component_mul(&dp);
                    let r_squared = dp2.sum();
                    let r = r_squared.sqrt();
                    let r_inverse_cubed = 1.0 / (r_squared * r);

                    let delta_acc = dp.component_mul(& Vector3::new(
                        -r_inverse_cubed,
                        -r_inverse_cubed,
                        -r_inverse_cubed
                    ));

                    // // add this force on to i's acceleration (mass = 1)
                    self.stars[i].acceleration += delta_acc;
                    // newtons third law
                    self.stars[j].acceleration -= delta_acc;
                }
            }
            // for j in 0..i {
            //     // Vector version of inverse square law
            //     let dp = self.stars[i].position - self.stars[j].position;
            //     let dp2 = dp.component_mul(&dp);
            //     let r_squared = dp2.sum();
            //     let r = r_squared.sqrt();
            //     let r_inverse_cubed = 1.0 / (r_squared * r);

            //     let delta_acc = dp.component_mul(& Vector3::new(
            //         -r_inverse_cubed,
            //         -r_inverse_cubed,
            //         -r_inverse_cubed
            //     ));

            //     // // add this force on to i's acceleration (mass = 1)
            //     self.stars[i].acceleration += delta_acc;
            //     // newtons third law
            //     self.stars[j].acceleration -= delta_acc;
            // }
        }
    }

    pub fn get_stars(&self) -> &Vec<Star> {
        &self.stars
    }
}

