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
        //TODO: Make this a dynamic utility function (AlexC / victoryforphil)
        write!(f, " Row: \t \t Col (mem):\n\t [{:?} {:?} {:?}] \t [{:?} {:?} {:?}]  \n", self[(0,0)], self[(0,1)], self[(0,2)], self.n[0][0], self.n[0][1],  self.n[0][2])?;
        write!(f, "\t [{:?} {:?} {:?}] \t [{:?} {:?} {:?}] \n", self[(1,0)], self[(1,1)], self[(1,2)], self.n[1][0], self.n[1][1],  self.n[1][2])?;
        write!(f, "\t [{:?} {:?} {:?}] \t [{:?} {:?} {:?}] \n", self[(2,0)], self[(2,1)], self[(2,2)], self.n[2][0], self.n[0][1],  self.n[2][2])?;

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
        let mut m = Self::default();
        m.n[0][0] = n00; m.n[0][1] = n10; m.n[0][2] = n20;
        m.n[1][0] = n01; m.n[1][1] = n11; m.n[1][2] = n21;
        m.n[2][0] = n02; m.n[2][1] = n12; m.n[2][2] = n22;
        m
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
        m.n[0][0] = v0.x; m.n[0][1] = v0.y; m.n[0][2] = v0.z;
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

        let matrix_index = Matrix3D::index_test();
        let matrix_new = Matrix3D::new(0.0,1.0,2.0,
                                       3.0,4.0,5.0,
                                       6.0,7.0,8.0);
        assert_eq!(matrix_index, matrix_new);
    }

    #[test]
    fn test_matrix_3_vector(){
        let matrix_vectors = Matrix3D::new_vectors(
            Vector3D::new(0.0,1.0,2.0),
            Vector3D::new(3.0,4.0,5.0),
            Vector3D::new(6.0,7.0,8.0)
        );
        assert_eq!(matrix_vectors.n[0], [0.0,1.0,2.0]);
        assert_eq!(matrix_vectors.n[1], [3.0,4.0,5.0]);
        assert_eq!(matrix_vectors.n[2], [6.0,7.0,8.0]);
    }
    #[test]
    fn test_matrix_3_order_test() {
        let matrix_3 = Matrix3D::index_test();
      
        info!("Game Maths: {:?}", matrix_3);
        // Test that we store the matrix in column major order
        assert_eq!(matrix_3.n[0][0], 0.0);
        assert_eq!(matrix_3.n[1][0], 1.0);
        assert_eq!(matrix_3.n[2][0], 2.0);
        assert_eq!(matrix_3.n[0][1], 3.0);
        assert_eq!(matrix_3.n[1][1], 4.0);
        assert_eq!(matrix_3.n[2][1], 5.0);
        assert_eq!(matrix_3.n[0][2], 6.0);
        assert_eq!(matrix_3.n[1][2], 7.0);
        assert_eq!(matrix_3.n[2][2], 8.0);

    }
    #[test]
    fn test_matrix_3_array_stuff(){
        let mut m = Matrix3D::index_test();
        //Test that we can access the matrix using the index trait in row major order
        assert_eq!(m[(0,0)], 0.0);
        assert_eq!(m[(0,1)], 1.0);
        assert_eq!(m[(0,2)], 2.0);
        assert_eq!(m[(1,0)], 3.0);
        assert_eq!(m[(1,1)], 4.0);
        assert_eq!(m[(1,2)], 5.0);
        assert_eq!(m[(2,0)], 6.0);
        assert_eq!(m[(2,1)], 7.0);
        assert_eq!(m[(2,2)], 8.0);

        //Test Indedx Mut
        m[(0,0)] = 10.0;
        assert_eq!(m[(0,0)], 10.0);
        
        
    }
}