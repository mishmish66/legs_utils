use super::joint::UR10JointEnum;
use super::poi::{POI, self};
use crate::{na, physics_types::{mass::PointMass, basic::*, inertial::*, spatial::*}};

#[derive(PartialEq, Eq)]
pub enum UR10LinkEnum {
    BaseLink,
    ShoulderLink,
    UpperArmLink,
    ElbowJoint,
    ForearmLink,
    Wrist1Link,
    Wrist2Link,
    Wrist3Link,
    EELink,
}

impl UR10LinkEnum {
    const fn get_child<const L: UR10LinkEnum>() -> Option<UR10JointEnum> {
        match L {
            BaseLink => Some(UR10JointEnum::ShoulderPanJoint),
            ShoulderLink => Some(UR10JointEnum::ShoulderLiftJoint),
            UpperArmLink => Some(UR10JointEnum::ElbowJoint),
            ForearmLink => Some(UR10JointEnum::Wrist1Joint),
            Wrist1Link => Some(UR10JointEnum::Wrist2Joint),
            Wrist2Link => Some(UR10JointEnum::Wrist3Joint),
            Wrist3Link => Some(UR10JointEnum::EEFixedJoint),
            EELink => None,
        }
    }

    const fn get_parent<const L: UR10LinkEnum>() -> Option<UR10JointEnum> {
        match L {
            BaseLink => None,
            ShoulderLink => Some(UR10JointEnum::ShoulderPanJoint),
            UpperArmLink => Some(UR10JointEnum::ShoulderLiftJoint),
            ForearmLink => Some(UR10JointEnum::ElbowJoint),
            Wrist1Link => Some(UR10JointEnum::Wrist1Joint),
            Wrist2Link => Some(UR10JointEnum::Wrist2Joint),
            Wrist3Link => Some(UR10JointEnum::Wrist3Joint),
            EELink => Some(UR10JointEnum::EEFixedJoint),
        }
    }
}
pub struct LinkDesc<const L: UR10LinkEnum> {
    mass: PointMass,
}

pub struct LinkState {

}