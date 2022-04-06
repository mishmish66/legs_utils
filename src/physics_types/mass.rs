use crate::na;
use super::basic::*;

pub struct PointMass {
    pub location: Vec3,
    pub mass: f64,
    pub moment: Mat3,
}

impl PointMass {
    pub fn new() -> Self {
        Self {
            location: na::zero(),
            mass: 0.0,
            moment: na::zero(),
        }
    }
}