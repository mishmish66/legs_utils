#[cfg(test)]
mod tests_robot {
    mod tests_link {
        use crate::{na, physics_types::{spatial, mass::PointMass}};
    
        use super::*;
    
        #[test]
        fn test_link_constructs() {
            let link: BasicLink<3>  = BasicLink {
                inertial: Inertial {
                    mass: PointMass {
                        location: Vec3::new(1.0, 0.0, 0.0),
                        mass: 1.0,
                        moment: na::one(),
                    },
                    spatial: Spatial {
                        pos: Transform {
                            lin: Vec3::new(-1.0, 0.0, 0.0),
                            ang: na::zero(),
                        },
                        vel: Transform::new(),
                    },
                },
            };
        }
    }
}