use super::basic::*;
use crate::na;

pub struct Spatial {
    pub pos: BigSE3,
    pub vel: SmallSE3,
}

pub struct BigSE3 {
    pub lin: Vec3,
    pub ang: Rot3,
}

pub struct SmallSE3 {
    pub lin: Vec3,
    pub ang: Vec3,
}

impl Spatial {
    pub fn new() -> Self {
        Self {
            pos: BigSE3::new(),
            vel: SmallSE3::new(),
        }
    }
}

impl BigSE3 {
    pub fn new() -> Self {
        Self {
            lin: na::zero(),
            ang: na::one(),
        }
    }
}

impl SmallSE3 {
    pub fn new() -> Self {
        Self {
            lin: na::zero(),
            ang: na::zero(),
        }
    }
}