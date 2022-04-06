#[cfg(test)]
mod tests {
    use std::iter::Rev;

    use crate::robot::joint::*;
    use crate::robot::link::*;
    use crate::robot::*;

    // test robot 1

    /*enum TR1JE {
        Base,
        LeftHipAbad,
        LeftHipPitch,
        LeftKneePitch,
        RightHipAbad,
        RightHipPitch,
        RightKneePitch,
    }

    enum TR1LE {
        Base,
        LeftHip,
        LeftThigh,
        LeftShank,
        RightHip,
        RightThigh,
        RightShank,
    }

    enum TR1POIE {}

    impl const RobotJointEnum for TR1JE {}

    impl const RobotLinkEnum for TR1LE {}

    impl const RobotPOIEnum for TR1POIE {}

    struct TR1DM {
        base: FloatingBase,
        left_hip_abad: RevoluteJoint,
        left_hip_pitch: RevoluteJoint,
        left_knee_pitch: RevoluteJoint,
        right_hip_abad: RevoluteJoint,
        right_hip_pitch: RevoluteJoint,
        right_knee_pitch: RevoluteJoint,
    }

    impl RobotDataManager<TR1JE, TR1LE, TR1POIE> for TR1DM {
        fn get_joint_list(&self) -> [TR1JE] {
            [
                TR1JE::Base,
                TR1JE::LeftHipAbad,
                TR1JE::LeftHipPitch,
                TR1JE::LeftKneePitch,
                TR1JE::RightHipAbad,
                TR1JE::RightHipPitch,
                TR1JE::RightKneePitch,
            ]
        }
    }*/

    #[test]
    fn test_do_tests_even_work_here() {}
}
