use super::basic::Matrix;

pub trait Jacobianable<const parent_ndofs: usize, const ndofs: usize> {
    fn get_ndofs(&self) -> usize;
    fn get_jacobian(&self, parent_jacobian: Matrix<6, parent_ndofs>) -> Matrix<6, ndofs>;
}