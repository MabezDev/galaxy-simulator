use nalgebra::base::Vector3;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Star {
    pub position : Vector3<f64>,
    pub velocity : Vector3<f64>,
    pub acceleration : Vector3<f64>,
    pub mass : f64,
}

impl Star {

    pub fn new() -> Self {
        Star {
            position: Vector3::new(0.0,0.0,0.0),
            velocity: Vector3::new(0.0,0.0,0.0),
            acceleration: Vector3::new(0.0,0.0,0.0),
            mass: 1.0,
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
    pub fn with_mass(self, vector:f64) -> Self {
        Self {
            mass: vector,
            .. self
        }
    }
}