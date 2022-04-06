use std::marker::PhantomData;

use crate::physics_types::{basic::*, spatial::*};

use self::joint::{RevoluteJointDesc, UR10JointEnum};

pub(crate) mod defines;
pub(crate) mod joint;
pub(crate) mod link;
pub(crate) mod poi;

struct Robot {}

impl Robot {}
