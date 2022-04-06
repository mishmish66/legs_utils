use crate::physics_types::{basic::*, spatial::*};

use super::link::UR10LinkEnum;
//use crate::physics_types::{basic::*, jacobianable::Jacobianable};

#[derive(PartialEq, Eq)]
pub enum UR10JointEnum {
    ShoulderPanJoint,
    ShoulderLiftJoint,
    ElbowJoint,
    Wrist1Joint,
    Wrist2Joint,
    Wrist3Joint,
    EEFixedJoint,
}

impl UR10JointEnum {
    const fn get_child<const J: UR10JointEnum>() -> UR10LinkEnum {
        match J {
            ShoulderPanJoint => UR10LinkEnum::ShoulderLink,
            ShoulderLiftJoint => UR10LinkEnum::UpperArmLink,
            ElbowJoint => UR10LinkEnum::ForearmLink,
            Wrist1Joint => UR10LinkEnum::Wrist1Link,
            Wrist2Joint => UR10LinkEnum::Wrist2Link,
            Wrist3Joint => UR10LinkEnum::Wrist3Link,
            EEFixedJoint => UR10LinkEnum::EELink,
        }
    }

    const fn get_parent<const J: UR10JointEnum>() -> UR10LinkEnum {
        match J {
            ShoulderPanJoint => UR10LinkEnum::BaseLink,
            ShoulderLiftJoint => UR10LinkEnum::ShoulderLink,
            ElbowJoint => UR10LinkEnum::UpperArmLink,
            Wrist1Joint => UR10LinkEnum::ForearmLink,
            Wrist2Joint => UR10LinkEnum::Wrist1Link,
            Wrist3Joint => UR10LinkEnum::Wrist2Link,
            EEFixedJoint => UR10LinkEnum::Wrist3Link,
        }
    }
}

pub struct RevoluteJointDesc<const J: UR10JointEnum> {
    pivot_pos: BigSE3,
    pivot_axis: Vec3,
    rotor_pos: BigSE3,
    rotor_inertia: f64,
}

pub struct FixedJointDesc<const J: UR10JointEnum> {
    root_pos: BigSE3,
}

pub struct RevoluteJointState<const J: UR10JointEnum> {
    pos: f64,
    vel: f64,
}
