use super::basic::*;
use crate::na;

pub struct Transform {
    pub pos: PosTransform,
    pub vel: VelTransform,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            pos: PosTransform::new(),
            vel: VelTransform::new(),
        }
    }
}

pub struct PosTransform {
    pub pos: Vec3,
    pub rot: Vec3,
}

pub struct VelTransform {
    pub lin: Vec3,
    pub ang: Vec3,
}

pub struct MomTransform {
    pub lin: Vec3,
    pub ang: Vec3,
}

impl PosTransform {
    pub fn new() -> Self {
        Self {
            pos: na::zero(),
            rot: na::zero(),
        }
    }

    pub fn get_rotmat(&self) -> RotMat {
        RotMat::new(self.rot)
    }
}

impl VelTransform {
    pub fn new() -> Self {
        Self {
            lin: na::zero(),
            ang: na::zero(),
        }
    }
}

impl MomTransform {
    pub fn new() -> Self {
        Self {
            lin: na::zero(),
            ang: na::zero(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_constructs() {
        let t = Transform::new();
        assert!(t.pos.pos == Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_pos_transform_rotmat() {
        let mut pt = PosTransform::new();

        let mut rotvec = Vec3::new(1.0, 0.0, 0.0);
        pt.rot = rotvec;
        let mut rm = pt.get_rotmat();
        let mut rm_test_against = RotMat::new(rotvec);
        assert!(rm == rm_test_against);

        rotvec = Vec3::new(0.0, 0.0, 1.0);
        pt.rot = rotvec;
        rm = pt.get_rotmat();
        rm_test_against = RotMat::new(rotvec);
        assert!(rm == rm_test_against);
    }

    #[test]
    fn test_vel_transform_constructs() {
        let mut vt = VelTransform::new();

        let pos = Vec3::new(1.0, 0.0, 0.0);
        vt.lin = pos;

        assert!(vt.lin == pos);
    }
}
