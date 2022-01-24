use crate::physics_types::{basic::*};
use super::link::*;

struct Joint {
    pivot_pos: Vec3,
    pivot_axis: Vec3,
    rotor_pos: Vec3,
    rotor_inertia: Vec3,

    parent_link: Link,
    child_link: Link,

    pos: f64,
    vel: f64,
}

struct FloatingBase {
    child_link: Vec<Joint>,
    pos: Vec6,
    vel: Vec6,
}

trait Ndof {
    fn ndof() -> i64;
}

impl Joint {
    fn new(
        pivot_pos: Vec3,
        pivot_axis: Vec3,
        rotor_pos: Vec3,
        rotor_inertia: Vec3,
        parent_link: Link,
        child_link: Link,
    ) -> Self {
        Self {
            pivot_pos: pivot_axis,
            pivot_axis: pivot_axis,
            rotor_pos: rotor_pos,
            rotor_inertia: rotor_inertia,
            parent_link: parent_link,
            child_link: child_link,
            pos: 0.0,
            vel: 0.0,
        }
    }
}

impl Ndof for Joint {
    fn ndof() -> i64 { 1 }
}

impl Ndof for FloatingBase {
    fn ndof() -> i64 { 6 }
}

