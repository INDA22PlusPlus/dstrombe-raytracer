use crate::vector::V3;
pub struct Matrix3 {
    pub i_hat : V3, // what x will be transformed by
    pub j_hat : V3, // y 
    pub k_hat : V3, // z 
}
impl Matrix3 {
    // transform a vector with this matrix
    pub fn transform_vec3(&self, vector : V3) -> V3 {
        return vector.x * self.i_hat + vector.y * self.j_hat + vector.z * self.k_hat;
    }
    
    // rotation matrix generation 
    pub fn rotation_x(theta : f32) -> Self {
        Self::new(
            V3::new(1.0, 
                    0.0,
                    0.0),
            V3::new(0.0,
                    f32::cos(theta),
                    f32::sin(theta)),
            V3::new(0.0,
                    -f32::sin(theta),
                    f32::cos(theta))
        )
    }
    pub fn rotation_z(theta : f32) -> Self {
        Self::new(
            V3::new(f32::cos(theta),
                    f32::sin(theta),
                    0.0),
            V3::new(-f32::sin(theta),
                    f32::cos(theta),
                    0.0),
            V3::new(0.0,0.0,1.0)
        )
    }
    pub fn rotation_y(theta : f32) -> Self {
        Self::new(
            V3::new(f32::cos(theta),
                    0.0,
                    -f32::sin(theta)),
            V3::new(0.0,1.0,0.0),
            V3::new(f32::sin(theta),
                    0.0,
                    f32::cos(theta))
            
        )
    }
    pub fn new(i_hat : V3, j_hat : V3, k_hat : V3) -> Self {
        Self {
            i_hat,
            j_hat,
            k_hat
        }
    }
}

// op overloads that call transform? Appropriating arithmetic operators for matrix purposes might
// be bad practice. OTOH we get to overload the * operator so that it works with matrices and
// vectors without two different function names
