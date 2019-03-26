

use nalgebra::base::Vector3;
use rand::prelude::*;
use rayon::prelude::*;

use crate::star::Star;

pub const GALAXY_WIDTH: f64 = 100.0;
pub const SPHERE_RADIUS: f64 = 20.0;
pub const ANGULAR_VELOCITY: f64 = 0.4;
pub const DELTA_TIME: f64 = 0.002; 
pub const DELTA_TIME_HALF: f64 = 0.002 / 2.0; 
pub const DELTA_TIME_SQUARED_HALF: f64 = (DELTA_TIME * DELTA_TIME) / 2.0; 

#[derive(Clone, Debug)]
pub struct Galaxy {
    stars: (Vec<Star>, Vec<Star>),
    iter: u64,
}

impl Galaxy {

    pub fn new(star_count: u64) -> Self {
        let mut stars = Vec::with_capacity(star_count as usize);
        // Randomly choose plane for net angular velocity
        let mut rng = thread_rng();

        // let mut n = Vector3::new(
        //     2.0 * rng.gen_range(0.0, 0.1) - 1.0,
        //     2.0 * rng.gen_range(0.0, 0.1) - 1.0,
        //     2.0 * rng.gen_range(0.0, 0.1) - 1.0,
        // );
        // nalgebra has a norm method
        // let norm = 1.0 / (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        // n[0] *= norm;
        // n[1] *= norm;
        // n[2] *= norm;

        // just rotate in x, y plane
        let n = Vector3::new(0.0, 0.0, 1.0);

        for _ in 0..star_count {

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
                // .with_mass(rng.gen_range(1, 5) as f64);
            stars.push(star);
        }

        Galaxy {
            stars: (stars.clone(), stars),
            iter: 0
        }
    }

    /// Services the galaxy for one complete iteration
    pub fn compute_iter(&mut self) -> &[Star] {
        // Verlet integration:
        // http://en.wikipedia.org/wiki/Verlet_integration#Velocity_Verlet

        // we avoid the use of locks by double buffering the stars
        // we calculate based on the `current_stars` as this is the current state of the galaxy
        // we put all the calculations for the next iteration in another array
        let (currrent_stars, future_stars) = if (self.iter & 1) == 0 {
            (&self.stars.0, &mut self.stars.1)
        } else {
            (&self.stars.1, &mut self.stars.0)
        };

        future_stars.iter_mut() // iterate over the future stars
            .zip(&currrent_stars[..]) // zip in the old stars state (old actually means current stars) for calculations
            .for_each(|(fut_star, curr_star)| {
                fut_star.position = Vector3::new(
                    curr_star.position[0] + (curr_star.velocity[0] * DELTA_TIME) + (curr_star.acceleration[0] * DELTA_TIME_SQUARED_HALF),
                    curr_star.position[1] + (curr_star.velocity[1] * DELTA_TIME) + (curr_star.acceleration[1] * DELTA_TIME_SQUARED_HALF),
                    curr_star.position[2] + (curr_star.velocity[2] * DELTA_TIME) + (curr_star.acceleration[2] * DELTA_TIME_SQUARED_HALF),
                );

                // based on the current stars, compute the new acceleration for the future star state
                fut_star.acceleration = Galaxy::compute_accelerations(curr_star, currrent_stars);

                // calculate the new velocity with the new acceleration
                fut_star.velocity = Vector3::new(
                    curr_star.velocity[0] + (fut_star.acceleration[0] * DELTA_TIME_HALF),
                    curr_star.velocity[1] + (fut_star.acceleration[1] * DELTA_TIME_HALF),
                    curr_star.velocity[2] + (fut_star.acceleration[2] * DELTA_TIME_HALF),
                );
                
            });

        self.iter += 1;

        future_stars
    }

    /// Iterates through the current state of the galaxy, calculating newtons third law on each star
    fn compute_accelerations(curr_star: &Star, current_galaxy: &[Star]) -> Vector3<f64> {
        
        let zero: Vector3<f64> = Vector3::zeros();
        let accel = current_galaxy
            .iter()
            // Starting a zero, we accumulate the affects of other stars on the current star
            .fold(zero, |mut accumalated, star| {
                let dp = curr_star.position - star.position;
                if dp != zero { // make sure we don't check against ourself
                    let dp2 = dp.component_mul(&dp);
                    let r_squared = dp2.sum();
                    let r = r_squared.sqrt();
                    let r_inverse_cubed = 1.0 / (r_squared * r);

                    let delta_acc = dp.component_mul(&Vector3::new(-r_inverse_cubed,-r_inverse_cubed,-r_inverse_cubed));

                    accumalated += delta_acc * curr_star.mass;
                }
                accumalated
            });

        accel
    }

    /// Services the galaxy for one complete iteration
    pub fn par_compute_iter(&mut self) -> &[Star] {
        // Verlet integration:
        // http://en.wikipedia.org/wiki/Verlet_integration#Velocity_Verlet

        // we avoid the use of locks by double buffering the stars
        // we calculate based on the `current_stars` as this is the current state of the galaxy
        // we put all the calculations for the next iteration in another array
        let (currrent_stars, future_stars) = if (self.iter & 1) == 0 {
            (&self.stars.0, &mut self.stars.1)
        } else {
            (&self.stars.1, &mut self.stars.0)
        };

        future_stars.par_iter_mut() // iterate over the future stars
            .zip(&currrent_stars[..]) // zip in the old stars state (old actually means current stars) for calculations
            .for_each(|(fut_star, curr_star)| {
                fut_star.position = Vector3::new(
                    curr_star.position[0] + (curr_star.velocity[0] * DELTA_TIME) + (curr_star.acceleration[0] * DELTA_TIME_SQUARED_HALF),
                    curr_star.position[1] + (curr_star.velocity[1] * DELTA_TIME) + (curr_star.acceleration[1] * DELTA_TIME_SQUARED_HALF),
                    curr_star.position[2] + (curr_star.velocity[2] * DELTA_TIME) + (curr_star.acceleration[2] * DELTA_TIME_SQUARED_HALF),
                );

                // based on the current stars, compute the new acceleration for the future star state
                fut_star.acceleration = Galaxy::par_compute_accelerations(curr_star, currrent_stars);

                // calculate the new velocity with the new acceleration
                fut_star.velocity = Vector3::new(
                    curr_star.velocity[0] + (fut_star.acceleration[0] * DELTA_TIME_HALF),
                    curr_star.velocity[1] + (fut_star.acceleration[1] * DELTA_TIME_HALF),
                    curr_star.velocity[2] + (fut_star.acceleration[2] * DELTA_TIME_HALF),
                );
                
            });

        self.iter += 1;

        future_stars
    }

    /// Iterates through the current state of the galaxy, calculating newtons third law on each star
    fn par_compute_accelerations(curr_star: &Star, current_galaxy: &[Star]) -> Vector3<f64> {
        
        let zero: Vector3<f64> = Vector3::zeros();
        let accel = current_galaxy
            .par_iter()
            // Starting a zero, we accumulate the affects of other stars on the current star
            .fold(
                || zero, 
                |mut accumalated, star| {
                let dp = curr_star.position - star.position;
                if dp != zero { // make sure we don't check against ourself
                    let dp2 = dp.component_mul(&dp);
                    let r_squared = dp2.sum();
                    let r = r_squared.sqrt();
                    let r_inverse_cubed = 1.0 / (r_squared * r);

                    let delta_acc = dp.component_mul(&Vector3::new(-r_inverse_cubed,-r_inverse_cubed,-r_inverse_cubed));

                    accumalated += delta_acc * curr_star.mass;
                }
                accumalated
            })
            .reduce(
                || Vector3::zeros(),
                |a: Vector3<f64>, b: Vector3<f64>| a + b,
            );

        accel
    }
}