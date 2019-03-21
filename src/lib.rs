

use nalgebra::base::Vector3;
use rand::prelude::*;

pub const STAR_COUNT: u64 = 5/* 2000 */;
pub const GALAXY_WIDTH: f64 = 100.0;

pub const SPHERE_RADIUS: f64 = 20.0;
pub const ANGULAR_VELOCITY: f64 = 0.4;
pub const DELTA_TIME: f64 = 0.002; 

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Star {
    position : Vector3<f64>,
    velocities : Vector3<f64>,
    accelerations : Vector3<f64>,
}

impl Star {

    pub fn new() -> Self {
        Star {
            position: Vector3::new(0.0,0.0,0.0),
            velocities: Vector3::new(0.0,0.0,0.0),
            accelerations: Vector3::new(0.0,0.0,0.0),
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
            velocities: vector,
            .. self
        }
    }
    pub fn with_accelerations(self, vector: Vector3<f64>) -> Self {
        Self {
            accelerations: vector,
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
        // let norm = 1.0 / (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]);
        // n[0] *= norm;
        // n[1] *= norm;
        // n[2] *= norm;

        // just rotate in x, y plane
        let n = Vector3::new(0.0, 0.0, 1.0);

        for _ in 0..STAR_COUNT {
            let r = Vector3::new(
                rng.gen_range(-SPHERE_RADIUS, SPHERE_RADIUS),
                rng.gen_range(-SPHERE_RADIUS, SPHERE_RADIUS),
                rng.gen_range(-SPHERE_RADIUS, SPHERE_RADIUS),
            );

            println!("x vel: {:?}", n[1] * r[2] - n[2] * r[1]);

            let new_vel = Vector3::new(
                ANGULAR_VELOCITY * (n[1] * r[2] - n[2] * r[1]),
                ANGULAR_VELOCITY * (n[2] * r[0] - n[0] * r[2]),
                ANGULAR_VELOCITY * (n[0] * r[1] - n[1] * r[0]),
            );

            let new_pos = Vector3::new(
                0.5 * GALAXY_WIDTH + r[0],
                0.5 * GALAXY_WIDTH + r[1],
                0.5 * GALAXY_WIDTH + r[2],
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
}

