#[cfg(test)]
mod tests_physics_types {
    use super::*;

    mod tests_inertial {
        use super::*;

        #[test]
        fn physics_types_inertial_inertia_constructs() {
            let mass = PointMass {
                location: Vec3::new(1.0, 0.0, 0.0),
                mass: 1.0,
                moment: na::one(),
            };

            let mut inertial = Inertial {
                spatial: Spatial::new(),
                mass: mass,
            };

            inertial.spatial.vel.ang = Vec3::new(0.0, 0.0, 1.0);
        }
    }

    mod spatial_tests {
        use super::*;

        #[test]
        fn physics_types_spatial_transform_constructs() {
            let s = Spatial::new();
            assert!(s.pos.lin == Vec3::new(0.0, 0.0, 0.0));
        }
    }
}
