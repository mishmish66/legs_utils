/*
 * These are some super tedious defines pulled from the UR10 URDF file
 * They are banished to this corner please do not speak of them
 */

use super::super::joint::*;
use crate::physics_types::{spatial::BigSE3, basic::*};

impl UR10JointEnum {
    const fn get_shoulder_pan_joint_desc() -> RevoluteJointDesc::<{UR10JointEnum::ShoulderPanJoint}> {
        RevoluteJointDesc {
            pivot_pos: BigSE3 {
                lin: Vec3::new(0.0, 0.0, 0.1273),
                ang: Rot3::from_euler_angles(0.0, 0.0, 0.0),
            },
            pivot_axis: Vec3::new(0.0, 0.0, 1.0),
            rotor_pos: BigSE3 {
                lin: Vec3::new(0.0, 0.0, 0.1273),
                ang: Rot3::from_euler_angles(0.0, 0.0, 0.0),
            },
            rotor_inertia: 0.0,
        }
    }

    const fn get_shoulder_lift_joint_desc() -> RevoluteJointDesc::<{UR10JointEnum::ShoulderLiftJoint}> {
        RevoluteJointDesc {
            pivot_pos: BigSE3 {
                lin: Vec3::new(0.0, 0.220941, 0.0),
                ang: Rot3::from_euler_angles(0.0, 1.570796325, 0.0),
            },
            pivot_axis: Vec3::new(0.0, 0.0, 1.0),
            rotor_pos: BigSE3 {
                lin: Vec3::new(0.0, 0.220941, 0.0),
                ang: Rot3::from_euler_angles(0.0, 1.570796325, 0.0),
            },
            rotor_inertia: 0.0,
        }
    }

    const fn get_elbow_joint_desc() -> RevoluteJointDesc::<{UR10JointEnum::ElbowJoint}> {
        RevoluteJointDesc {
            pivot_pos: BigSE3 {
                lin: Vec3::new(0.0, -0.1719, 0.612),
                ang: Rot3::from_euler_angles(0.0, 0.0, 0.0),
            },
            pivot_axis: Vec3::new(0.0, 0.0, 1.0),
            rotor_pos: BigSE3 {
                lin: Vec3::new(0.0, -0.1719, 0.612),
                ang: Rot3::from_euler_angles(0.0, 0.0, 0.0),
            },
            rotor_inertia: 0.0,
        }
    }

    const fn get_wrist_1_joint_desc() -> RevoluteJointDesc::<{UR10JointEnum::Wrist1Joint}> {
        RevoluteJointDesc {
            pivot_pos: BigSE3 {
                lin: Vec3::new(0.0, 0.0, 0.5723),
                ang: Rot3::from_euler_angles(0.0, 1.570796325, 0.0),
            },
            pivot_axis: Vec3::new(0.0, 0.0, 1.0),
            rotor_pos: BigSE3 {
                lin: Vec3::new(0.0, 0.0, 0.5723),
                ang: Rot3::from_euler_angles(0.0, 1.570796325, 0.0),
            },
            rotor_inertia: 0.0,
        }
    }

    const fn get_wrist_2_joint_desc() -> RevoluteJointDesc::<{UR10JointEnum::Wrist2Joint}> {
        RevoluteJointDesc {
            pivot_pos: BigSE3 {
                lin: Vec3::new(0.0, 0.1149, 0.0),
                ang: Rot3::from_euler_angles(0.0, 0.0, 0.0),
            },
            pivot_axis: Vec3::new(0.0, 0.0, 1.0),
            rotor_pos: BigSE3 {
                lin: Vec3::new(0.0, 0.1149, 0.0),
                ang: Rot3::from_euler_angles(0.0, 0.0, 0.0),
            },
            rotor_inertia: 0.0,
        }
    }

    const fn get_wrist_3_joint_desc() -> RevoluteJointDesc::<{UR10JointEnum::Wrist3Joint}> {
        RevoluteJointDesc {
            pivot_pos: BigSE3 {
                lin: Vec3::new(0.0, 0.0, 0.1157),
                ang: Rot3::from_euler_angles(0.0, 0.0, 0.0),
            },
            pivot_axis: Vec3::new(0.0, 0.0, 1.0),
            rotor_pos: BigSE3 {
                lin: Vec3::new(0.0, 0.0, 0.1157),
                ang: Rot3::from_euler_angles(0.0, 0.0, 0.0),
            },
            rotor_inertia: 0.0,
        }
    }

    const fn get_ee_fixed_joint_desc() -> FixedJointDesc::<{UR10JointEnum::EEFixedJoint}> {
        FixedJointDesc {
            root_pos: BigSE3 {
                lin: Vec3::new(0.0, 0.0, 0.1157),
                ang: Rot3::from_euler_angles(0.0, 0.0, 0.0),
            },
        }
    }
}