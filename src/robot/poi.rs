use crate::physics_types::*;
use crate::na;
use crate::physics_types::spatial::BigSE3;

pub struct POI<const dofs: usize> {
    location: BigSE3,
}

impl<const ndofs: usize> POI<ndofs> {
    fn new() -> Self {
        Self {
            location: BigSE3::new(),
        }
    }
}