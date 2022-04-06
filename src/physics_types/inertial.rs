use super::basic::*;
use super::mass::*;
use super::spatial::*;
use crate::na;

pub struct Inertial {
    pub spatial: Spatial,
    pub mass: PointMass,
}

impl Inertial {
    pub fn new() -> Self {
        Self {
            spatial: Spatial::new(),
            mass: PointMass::new(),
        }
    }

    pub fn momentum(&self) -> SmallSE3 {
        let mom_lin = self.mass.mass * self.spatial.vel.lin;

        let mom_ang = self.mass.moment * self.spatial.vel.ang;

        SmallSE3 {
            lin: mom_lin,
            ang: mom_ang,
        }
    }
}
