use crate::physics_types::{transform::*, inertial::*, basic::*};

pub(crate) struct Link {
    inertial: Inertial,
    contact_points: Vec<ContactPoint>,
}

#[derive(Clone)]
enum ContactPoint {
    Generic(Vec3),
    Foot(Vec3),
}

impl Link {
    fn new(inertial: Inertial, mass_mat: Mat3) -> Self {
        Self {
            inertial: inertial,
            contact_points: Vec::new(),
        }
    }

    fn add_contact_point(&mut self, contact_points: &[ContactPoint]) {
        self.contact_points.extend_from_slice(contact_points);
    }

    fn get_inertia(&self) -> MomTransform {
        self.inertial.momentum()
    }
}

#[cfg(test)]
mod tests {
    use crate::physics_types::{na, inertial};

    use super::*;

    #[test]
    fn test_legs_utils_link_constructs() {
        let mass_mat = na::one();
        let link = Link::new(Inertial::new(), mass_mat);
    }
}