use std::process::Child;

use super::link::*;
use crate::physics_types::{basic::*, jacobianable::Jacobianable};

pub trait Joint {}

struct RevoluteJoint<const parent_ndofs: usize, ChildLinkType: Link<{parent_ndofs + 1}>> {
    pivot_pos: Vec3,
    pivot_axis: Vec3,
    rotor_pos: Vec3,
    rotor_inertia: Vec3,

    child_link: ChildLinkType,
    pos: f64,
    vel: f64,
}
struct PrismaticJoint<const parent_ndofs: usize, ChildLinkType: Link<{parent_ndofs + 1}>> {
    zero_pos: Vec3,
    move_axis: Vec3,

    child_link: ChildLinkType,
    pos: f64,
    vel: f64,
}

struct FloatingBase<ChildLinkType: Link<6>> {
    child_link: ChildLinkType,
    pos: Vec6,
    vel: Vec6,
}

impl<const parent_ndofs: usize, ChildLinkType: Link<{parent_ndofs + 1}>> RevoluteJoint<parent_ndofs, ChildLinkType> { }

impl<const parent_ndofs: usize, ChildLinkType: Link<{parent_ndofs + 1}>> Joint for RevoluteJoint<parent_ndofs, ChildLinkType> {}

impl<const parent_ndofs: usize, ChildLinkType: Link<{parent_ndofs + 1}>> Joint for PrismaticJoint<parent_ndofs, ChildLinkType> {}
