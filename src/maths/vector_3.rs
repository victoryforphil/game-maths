use std::{
    fmt::Debug,
    ops::{Index, IndexMut, Mul, DivAssign, MulAssign, Div},
};

#[derive(Clone, Copy)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Default Impl
impl Default for Vector3D {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Debug for Vector3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector3D")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

impl Index<usize> for Vector3D {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vector3D"),
        }
    }
}

/// Convert from Vector3D to array
/// 
/// # Example
/// ```
/// use game_maths::maths::Vector3D;
/// 
/// let vector_3d = Vector3D::new(1.0, 2.0, 3.0);
/// let array: [f64; 3] = vector_3d.into();
/// assert_eq!(array, [1.0, 2.0, 3.0]);
impl From<Vector3D> for [f64; 3] {
    fn from(vector_3d: Vector3D) -> Self {
        [vector_3d.x, vector_3d.y, vector_3d.z]
    }
}

impl From<[f64; 3]> for Vector3D {
    fn from(array: [f64; 3]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}
impl From<Vector3D> for [f32; 3] {
    fn from(vector_3d: Vector3D) -> Self {
        [vector_3d.x as f32, vector_3d.y as f32, vector_3d.z as f32]
    }
}

impl PartialEq for Vector3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == self.z
    } 
}

/// IndexMut impl for Vector3D
///
/// # Example
/// ```
///     use game_maths::maths::Vector3D;
///
///    let mut vector_3d = Vector3D::new(1.0, 2.0, 3.0);
///     vector_3d[0] = 4.0;
///     vector_3d[1] = 5.0;
///     vector_3d[2] = 6.0;
///     assert_eq!(vector_3d, [4.0, 5.0, 6.0].into());
/// ```
impl IndexMut<usize> for Vector3D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Vector3D"),
        }
    }
}

/// Scalar multiplication for Vector3D
/// 
/// # Example
/// ```
/// use game_maths::maths::Vector3D;
/// 
/// let vector_3d = Vector3D::new(1.0, 2.0, 3.0);
/// let vector_3d = vector_3d * 2.0;
/// assert_eq!(vector_3d, [2.0, 4.0, 6.0].into());
/// ```
impl Mul<f64> for Vector3D {
    type Output = Self;
    fn mul(self, s: f64) -> Self::Output {
        // S = scalar
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl MulAssign<f64> for Vector3D {
    fn mul_assign(&mut self, s: f64) {
        // S = scalar
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }
}
/// Scalar division for Vector3D
/// 
/// # Example
/// ```
/// use game_maths::maths::Vector3D;
/// 
/// let vector_3d = Vector3D::new(1.0, 2.0, 3.0);
/// let vector_3d = vector_3d / 2.0;
/// assert_eq!(vector_3d, [0.5, 1.0, 1.5].into());

impl Div<f64> for Vector3D {
    type Output = Self;
    fn div(self, s: f64) -> Self::Output {
        // S = scalar
        Self {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
        }
    }
}

impl DivAssign<f64> for Vector3D {
    fn div_assign(&mut self, s: f64) {
        // S = scalar
        self.x /= s;
        self.y /= s;
        self.z /= s;
    }
}
impl Vector3D {
    /// Creates a new Vector3D
    ///
    /// # Arguments
    /// - `x` - The x value of the Vector3D (Index 0)
    /// - `y` - The y value of the Vector3D (Index 1)
    /// - `z` - The z value of the Vector3D (Index 2)
    ///
    /// # Example
    /// ```
    /// use game_maths::maths::Vector3D;
    ///
    /// let vector_3d = Vector3D::new(1.0, 2.0, 3.0);
    /// assert_eq!(vector_3d, [1.0, 2.0, 3.0].into());
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Creates a new Vector3D from a slice
    ///
    /// # Arguments
    /// - `slice` - The slice to create the Vector3D from
    ///
    /// # Example
    /// ```
    /// use game_maths::maths::Vector3D;
    ///
    /// let vector_3d = Vector3D::from_slice(&[1.0, 2.0, 3.0]);
    /// assert_eq!(vector_3d, [1.0, 2.0, 3.0].into());
    /// ```
    pub fn from_slice(slice: &[f64]) -> Self {
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }

    /// Creates a new Vector3D from an array
    ///
    /// # Arguments
    /// - `array` - The array to create the Vector3D from
    ///
    /// # Example
    /// ```
    /// use game_maths::maths::Vector3D;
    ///
    /// let vector_3d = Vector3D::from_array([1.0, 2.0, 3.0]);
    /// assert_eq!(vector_3d, [1.0, 2.0, 3.0].into());
    /// ```
    pub fn from_array(array: [f64; 3]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[test]
    fn test_vector_3d() {
        let vector_3d = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(vector_3d, [1.0, 2.0, 3.0].into());

        let vector_3d = Vector3D::default();
        assert_eq!(vector_3d, [0.0, 0.0, 0.0].into());

        info!("Game Maths: {:?}", vector_3d);
    }

    #[test]
    fn test_vector_3d_from_slice() {
        let vector_3d = Vector3D::from_slice(&[1.0, 2.0, 3.0]);
        assert_eq!(vector_3d.x, 1.0);
        assert_eq!(vector_3d.y, 2.0);
        assert_eq!(vector_3d.z, 3.0);
    }

    #[test]
    fn test_vector_3d_from_array() {
        let vector_3d = Vector3D::from_array([1.0, 2.0, 3.0]);
        assert_eq!(vector_3d.x, 1.0);
        assert_eq!(vector_3d.y, 2.0);
        assert_eq!(vector_3d.z, 3.0);
    }

    #[test]
    fn test_vector_3d_index() {
        let vector_3d = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(vector_3d[0], 1.0);
        assert_eq!(vector_3d[1], 2.0);
        assert_eq!(vector_3d[2], 3.0);
    }

    #[test]
    fn test_vector_3d_index_mut() {
        let mut vector_3d = Vector3D::new(1.0, 2.0, 3.0);
        vector_3d[0] = 4.0;
        vector_3d[1] = 5.0;
        vector_3d[2] = 6.0;
        assert_eq!(vector_3d[0], 4.0);
        assert_eq!(vector_3d[1], 5.0);
        assert_eq!(vector_3d[2], 6.0);
    }

    #[test]
    fn test_vector_3d_into() {
        let vector_3d = Vector3D::new(1.0, 2.0, 3.0);
        let array: [f64; 3] = vector_3d.into();
        assert_eq!(array, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_vector_3d_from() {
        let vector_3d = Vector3D::from([1.0, 2.0, 3.0]);
        assert_eq!(vector_3d, [1.0, 2.0, 3.0].into());
    }

    #[test]
    fn test_vector_3d_scalar() {
        let vector_3d = Vector3D::new(1.0, 2.0, 3.0);
        let mut vector_3d = vector_3d * 2.0;
        assert_eq!(vector_3d, [2.0, 4.0, 6.0].into());
        vector_3d *= 2.0;
        assert_eq!(vector_3d, [4.0, 8.0, 12.0].into());

        let vector_3d = Vector3D::new(1.0, 2.0, 3.0);
        let mut vector_3d = vector_3d / 2.0;
        assert_eq!(vector_3d, [0.5, 1.0, 1.5].into());
        vector_3d /= 2.0;
        assert_eq!(vector_3d, [0.25, 0.5, 0.75].into());
    }
}
