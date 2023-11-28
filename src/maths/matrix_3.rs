use std::{fmt::{Debug, Formatter, self}, ops::{IndexMut, Index}};

use super::Vector3D;


pub type Matrix3DRow = [f64;3];
/// Collumn major order
/// [ 0 3 6 ]       [ Row ]
/// [ 1 4 7 ]  -->  [ Row ]
/// [ 2 5 8 ]       [ Row ]
/// 
/// 
#[derive(Clone, Copy,PartialEq)]
pub struct Matrix3D{
    pub n: [Matrix3DRow;3]
}

impl Default for Matrix3D {
    fn default() -> Self {
        Self {
            n: [
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0]
            ]
        }
    }
}

impl Debug for Matrix3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        /// Print matrix in row major order
        /// [ 0 1 2 ]
        /// [ 3 4 5 ]
        /// [ 6 7 8 ]
        /// 
        //TODO: Make this a dynamic utility function (AlexC / victoryforphil)
        write!(f, "\n[ ")?;
        write!(f, "{} ", self.n[0][0])?;
        write!(f, "{} ", self.n[0][1])?;
        write!(f, "{} ", self.n[0][2])?;
        write!(f, "]\n")?;
        write!(f, "[ ")?;
        write!(f, "{} ", self.n[1][0])?;
        write!(f, "{} ", self.n[1][1])?;
        write!(f, "{} ", self.n[1][2])?;
        write!(f, "]\n")?;
        write!(f, "[ ")?;
        write!(f, "{} ", self.n[2][0])?;
        write!(f, "{} ", self.n[2][1])?;
        write!(f, "{} ", self.n[2][2])?;
        write!(f, "]")?;
        Ok(())
        
    }
}


/// Indexing is in row major order
/// [ 0 1 2 ]
/// [ 3 4 5 ]
/// [ 6 7 8 ]
impl Index<(usize, usize)> for Matrix3D {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.n[col][row]
    }
}

impl IndexMut<(usize, usize)> for Matrix3D {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.n[col][row]
    }
}

impl Matrix3D {
    pub fn new( n00:f64,n01:f64,n02:f64,
                n10:f64,n11:f64,n12:f64,
                n20:f64,n21:f64,n22:f64) -> Self {
        Self {
            n: [
                [n00, n10, n20],
                [n01, n11, n21],
                [n02, n12, n22]
            ]
        }
    }

    /// Create a matrix from 3 vectors
    /// 
    /// 
    /// # Arguments
    /// - `v0:Vector3D` - The first vector
    /// - `v1:Vector3D` - The second vector
    /// - `v2:Vector3D` - The third vector
    /// 
    /// # Example
    /// ```
    /// use game_maths::maths::Matrix3D;
    /// use game_maths::maths::Vector3D;
    /// 
    /// let m = Matrix3D::new_vectors(
    ///    Vector3D::new(0.0,0.0,0.0),
    ///   Vector3D::new(1.0,1.0,1.0),
    ///  Vector3D::new(2.0,2.0,2.0)
    /// );
    /// 
    /// assert_eq!(m.n[0][0], 0.0);
    /// assert_eq!(m.n[1][0], 1.0);
    /// assert_eq!(m.n[2][0], 2.0);
    /// ```
    pub fn new_vectors( v0:Vector3D,v1:Vector3D,v2:Vector3D) -> Self {
        let mut m = Self::default();
        m.n[0][0] = v0.x; m.n[1][0] = v1.x; m.n[2][0] = v2.x;
        m.n[1][0] = v1.x; m.n[1][1] = v1.y; m.n[1][2] = v1.z;
        m.n[2][0] = v2.x; m.n[2][1] = v2.y; m.n[2][2] = v2.z;

        m
    }
    /// Create a matrix from a 3x3 array with the values in row major order:
    /// ```md
    /// [ 0 1 2 ]   
    /// [ 3 4 5 ]   
    /// [ 6 7 8 ]   
    /// ```
    /// 
    /// # Example
    /// ```
    /// use game_maths::maths::Matrix3D;
    /// 
    /// let m = Matrix3D::index_test();
    ///
    /// assert_eq!(m[(0,0)], 0.0);
    /// // ...
    /// assert_eq!(m[(2,2)], 8.0);
    /// ```
    pub fn index_test() -> Self{
        return Self::new(0.0,1.0,2.0,
                         3.0,4.0,5.0,
                         6.0,7.0,8.0);
    }
}
#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[test]
    fn test_matrix_3() {
        let matrix_3 = Matrix3D::default();
        info!("Game Maths: {:?}", matrix_3);
    }
    #[test]
    fn test_matrix_3_order_test() {
        let matrix_3 = Matrix3D::new(0.0,1.0,2.0,
                                     3.0,4.0,5.0,
                                     6.0,7.0,8.0);

      
        info!("Game Maths: {:?}", matrix_3);
        assert_eq!(matrix_3.n[2][0], 2.0);
        assert_eq!(matrix_3.n[0][2], 6.0);
    }
    #[test]
    fn test_matrix_3_array_stuff(){
        let m = Matrix3D::default();
        assert_eq!(m.n[0][0], 0.0);
        
    }
}