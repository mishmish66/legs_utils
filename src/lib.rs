#![feature(const_trait_impl, adt_const_params)]

pub(crate) extern crate nalgebra as na;

pub(crate) mod physics_types;
pub(crate) mod robot;
mod tests;

pub struct Robot {
}
