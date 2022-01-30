use crate::physics_types::{basic::*, jacobianable::Jacobianable, transform::PosTransform};
use crate::na;
use crate::robot::link::Link;

pub struct POI<const dofs: usize> {
    location: PosTransform,
    //kinematic_parent: KinematicParentType,
}

impl<const ndofs: usize> POI<ndofs> {
    fn new() -> Self {
        Self {
            location: PosTransform::new(),
            //kinematic_parent: None,
        }
    }
}