pub(crate) mod inertial;
pub(crate) mod mass;
pub(crate) mod transform;
pub(crate) mod jacobianable;

pub(crate) mod basic {
    use crate::na;
    pub type Vec3 = na::Vector3<f64>;
    pub type Mat3 = na::Matrix3<f64>;
    pub type Mat4 = na::Matrix4<f64>;
    pub type Vec6 = na::Vector6<f64>;
    pub type RotMat = na::Rotation3<f64>;
    pub type Matrix<const R: usize, const C: usize> = na::Matrix<f64, na::Const<R>, na::Const<C>, na::ArrayStorage<f64, R, C>>;
}
