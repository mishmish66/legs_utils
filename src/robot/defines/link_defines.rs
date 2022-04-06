use super::super::link::*;
use crate::physics_types::{mass::PointMass, basic::*};

impl UR10LinkEnum {
    const fn get_base_link_desc() -> LinkDesc::<{UR10LinkEnum::BaseLink}> {
        LinkDesc {
            mass: PointMass {
                mass: 4.0,
                location: na::zero(),
                moment: Mat3::new(0.0061063308908, 0.0, 0.0, 
                                  0.0, 0.0061063308908, 0.0,
                                  0.0, 0.0, 0.01125),
            }
        }
    }

    const fn get_shoulder_link_desc() -> LinkDesc::<{UR10LinkEnum::ShoulderLink}> {
        LinkDesc {
            mass: PointMass {
                mass: 7.778,
                location: na::zero(),
                moment: Mat3::new(0.0314743125769, 0.0, 0.0, 
                                  0.0, 0.0314743125769, 0.0,
                                  0.0, 0.0, 0.021875625),
            }
        }
    }

    const fn get_upper_arm_link_desc() -> LinkDesc::<{UR10LinkEnum::UpperArmLink}> {
        LinkDesc {
            mass: PointMass {
                mass: 12.93,
                location: na::zero(),
                moment: Mat3::new(0.421753803798, 0.0, 0.0, 
                                  0.0, 0.421753803798, 0.0,
                                  0.0, 0.0, 0.036365625),
            }
        }
    }

    const fn get_forearm_link_desc() -> LinkDesc::<{UR10LinkEnum::ForearmLink}> {
        LinkDesc {
            mass: PointMass {
                mass: 3.87,
                location: na::zero(),
                moment: Mat3::new(0.111069694097, 0.0, 0.0, 
                                  0.0, 0.111069694097, 0.0,
                                  0.0, 0.0, 0.010884375),
            }
        }
    }

    const fn get_wrist_1_link_desc() -> LinkDesc::<{UR10LinkEnum::Wrist1Link}> {
        LinkDesc {
            mass: PointMass {
                mass: 1.96,
                location: na::zero(),
                moment: Mat3::new(0.0051082479567, 0.0, 0.0, 
                                  0.0, 0.0051082479567, 0.0,
                                  0.0, 0.0, 0.0055125),
            }
        }
    }

    const fn get_wrist_2_link_desc() -> LinkDesc::<{UR10LinkEnum::Wrist2Link}> {
        LinkDesc {
            mass: PointMass {
                mass: 1.96,
                location: na::zero(),
                moment: Mat3::new(0.0051082479567, 0.0, 0.0, 
                                  0.0, 0.0051082479567, 0.0,
                                  0.0, 0.0, 0.0055125),
            }
        }
    }

    const fn get_wrist_3_link_desc() -> LinkDesc::<{UR10LinkEnum::Wrist3Link}> {
        LinkDesc {
            mass: PointMass {
                mass: 0.202,
                location: na::zero(),
                moment: Mat3::new(0.000526462289415, 0.0, 0.0, 
                                  0.0, 0.000526462289415, 0.0,
                                  0.0, 0.0, 0.000568125),
            }
        }
    }

    const fn get_ee_link_desc() -> LinkDesc::<{UR10LinkEnum::EELink}> {
        LinkDesc {
            mass: PointMass {
                mass: 0.0,
                location: na::zero(),
                moment: na::zero(),
            }
        }
    }
}
