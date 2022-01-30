use std::usize;
use super::poi::{POI, self};
use crate::{na, physics_types::jacobianable::Jacobianable};

use crate::physics_types::{basic::*, inertial::*, transform::*};

pub trait Link<const ndofs: usize>: Jacobianable<ndofs, ndofs> { }

pub struct BasicLink<const dofs: usize> {
    inertial: Inertial,
}

impl<const ndofs: usize> Jacobianable<ndofs, ndofs> for BasicLink<ndofs> {
    fn get_jacobian(&self, parent_jacobian: Matrix<6, ndofs>) -> Matrix<6, ndofs> {
        na::zero()
    }
}

impl<const ndofs: usize> Link<ndofs> for BasicLink<ndofs> { }

impl<const ndofs: usize> BasicLink<ndofs> {
    fn new() -> Self {
        Self {
            inertial: Inertial::new(),
        }
    }

    fn get_inertia(&self) -> MomTransform {
        self.inertial.momentum()
    }
}

#[cfg(test)]
mod tests {
    use crate::{na, physics_types::{inertial, mass::PointMass}};

    use super::*;

    #[test]
    fn test_legs_utils_link_constructs() {
        let link: BasicLink<3>  = BasicLink {
            inertial: Inertial {
                mass: PointMass {
                    location: Vec3::new(1.0, 0.0, 0.0),
                    mass: 1.0,
                    moment: na::one(),
                },
                transform: Transform {
                    pos: PosTransform {
                        pos: Vec3::new(-1.0, 0.0, 0.0),
                        rot: na::zero(),
                    },
                    vel: VelTransform::new(),
                },
            },
        };
    }
}
