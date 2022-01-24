use super::basic::*;
use super::na;
use likely_stable::unlikely;

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
    rot: Vec3,
}

pub struct VelTransform {
    pub lin: Vec3,
    pub ang: Vec3,
}

pub struct MomTransform {
    pub lin: Vec3,
    pub ang: Vec3,
}

/*impl PosTransform {
    pub fn new() -> Self {
        Self {
            pos: na::zero(),
            rot: na::zero(),
            rotmat: None,
        }
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

    fn set_rot(&mut self, rot: Vec3) {
        self.rot = rot;
        self.rotmat = None;
    }
}*/
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
