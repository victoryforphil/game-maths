mod vector_3;
mod matrix_3;

pub use vector_3::Vector3D;
pub use matrix_3::Matrix3D;

#[derive(Debug, PartialEq)]
pub struct Maths;
impl Maths {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maths() {
        let maths = Maths::new();
        assert_eq!(maths, maths);
    }
}