extern crate nalgebra as na;
use likely_stable::unlikely;

pub type Vec3 = na::Vector3<f64>;
pub type Mat3 = na::Matrix3<f64>;
pub type Mat4 = na::Matrix4<f64>;
pub type Vec6 = na::Vector6<f64>;
pub type RotMat = na::Rotation3<f64>;

pub(crate) struct Transform {
    pos: PosTransform,
    vel: VelTransform,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            pos: PosTransform::new(),
            vel: VelTransform::new(),
        }
    }

    pub fn set(&mut self, pos: PosTransform, vel: VelTransform) {
        self.pos = pos;
        self.vel = vel;
    }

    pub fn get_pos(&self) -> &PosTransform {
        &self.pos
    }

    pub fn get_vel(&self) -> &VelTransform {
        &self.vel
    }
}

pub(crate) struct PosTransform {
    pos: Vec3,
    rot: Vec3,

    rotmat: Option<RotMat>,
}

pub(crate) struct VelTransform {
    pos: Vec3,
    rot: Vec3,
}

impl PosTransform {
    pub fn new() -> Self {
        Self {
            pos: na::zero(),
            rot: na::zero(),
            rotmat: None,
        }
    }

    pub fn assign(&mut self, pos: Vec3, rot: Vec3) {
        self.set_lin_vec(pos);
        self.set_rot_vec(rot);
    }

    pub fn get_rotmat(&mut self) -> &RotMat {
        if unlikely(self.rotmat.is_none()) {
            self.force_rotmat_update();
        }
        self.rotmat.as_ref().unwrap()
    }

    fn force_rotmat_update(&mut self) {
        self.rotmat = Some(RotMat::new(self.rot));
    }

    pub fn get_lin_vec(&self) -> &Vec3 {
        &self.pos
    }

    pub fn get_rot_vec(&self) -> &Vec3 {
        &self.rot
    }

    pub fn set_lin_vec(&mut self, pos: Vec3) {
        self.pos = pos;
    }

    pub fn set_rot_vec(&mut self, rot: Vec3) {
        self.rot = rot;
        self.rotmat = None;
    }
}

impl VelTransform {
    pub fn new() -> Self {
        Self {
            pos: na::zero(),
            rot: na::zero(),
        }
    }

    pub fn assign(&mut self, pos: Vec3, rot: Vec3) {
        self.pos = pos;
        self.rot = rot;
    }

    pub fn get_lin_vec(&self) -> &Vec3 {
        &self.pos
    }

    pub fn get_rot_vec(&self) -> &Vec3 {
        &self.rot
    }

    pub fn set_lin_vec(&mut self, pos: Vec3) {
        self.pos = pos;
    }

    pub fn set_rot_vec(&mut self, rot: Vec3) {
        self.rot = rot;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_constructs() {
        let t = Transform::new();
        assert!(*t.get_pos().get_lin_vec() == Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_pos_transform_rotmat() {
        let mut pt = PosTransform::new();

        let mut rotvec = Vec3::new(1.0, 0.0, 0.0);
        pt.set_rot_vec(rotvec);
        let mut rm = *pt.get_rotmat();
        let mut rm_test_against = RotMat::new(rotvec);
        assert!(rm == rm_test_against);

        rotvec = Vec3::new(0.0, 0.0, 1.0);
        pt.set_rot_vec(rotvec);
        rm = *pt.get_rotmat();
        rm_test_against = RotMat::new(rotvec);
        assert!(rm == rm_test_against);
    }

    #[test]
    fn test_vel_transform_constructs() {
        let mut vt = VelTransform::new();

        let pos = Vec3::new(1.0, 0.0, 0.0);
        vt.set_lin_vec(pos);

        assert!(*vt.get_lin_vec() == pos);
    }
}
