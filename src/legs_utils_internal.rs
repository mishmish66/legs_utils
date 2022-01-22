use crate::physics_types::transform::*;

pub(crate) struct LegsUtilsLink {
    pos: Vec3,
    mass_mat: Mat3,
    contact_points: Vec<ContactPoint>,
}

struct ContactPoint {
    pos: Vec3,
    foot: bool,
}

struct LegsUtilsJoint {
    pivot_pos: Vec3,
    pivot_axis: Vec3,
    rotor_pos: Vec3,
    rotor_inertia: Vec3,

    parent_link: LegsUtilsLink,
    child_link: LegsUtilsLink,

    pos: f64,
    vel: f64,
}

struct FloatingBaseJoint {
    child_link: LegsUtilsLink,
    pos: Vec6,
    vel: Vec6,
}

impl LegsUtilsLink {
    fn new(pos: Vec3, mass_mat: Mat3) -> Self {
        Self {
            pos: pos,
            mass_mat: mass_mat,
            contact_points: Vec::new(),
        }
    }

    fn add_contact_point(&mut self, contact_points: &[ContactPoint]) {
        self.contact_points.extend_from_slice(contact_points);
    }

    fn get_inertia(&mut self, state: &Transform) {}
}

impl Clone for ContactPoint {
    fn clone(&self) -> Self {
        return ContactPoint {
            pos: self.pos.clone(),
            foot: self.foot.clone(),
        };
    }
}

impl LegsUtilsJoint {
    fn new(
        pivot_pos: Vec3,
        pivot_axis: Vec3,
        rotor_pos: Vec3,
        rotor_inertia: Vec3,
        parent_link: LegsUtilsLink,
        child_link: LegsUtilsLink,
    ) -> Self {
        Self {
            pivot_pos: pivot_axis,
            pivot_axis: pivot_axis,
            rotor_pos: rotor_pos,
            rotor_inertia: rotor_inertia,
            parent_link: parent_link,
            child_link: child_link,
            pos: 0.0,
            vel: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legs_utils_link_constructs() {
        let mass_mat = Mat3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);
        let link = LegsUtilsLink::new(Vec3::new(0.0, 0.0, 0.0), mass_mat);
    }
}
