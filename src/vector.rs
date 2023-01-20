use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct V3 {
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

impl V3 {
    pub fn new(x : f32, y : f32, z : f32) -> Self {
        V3 {x, y, z}
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn dot(&self, rh : V3) -> f32 {
        self.x * rh.x + self.y * rh.y + self.z * rh.z
    }

    pub fn cross(&self, rh : V3) -> V3 {
        V3 {
            x : self.y * rh.z - self.z * rh.y,
            y : -(self.x * rh.z - self.z * rh.x),
            z : self.x * rh.y - self.y - rh.x
        }
    }
    
    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn dist(&self, other : V3) -> f32 {
        (*self - other).magnitude()
    }

    
    pub fn normalized(&self) -> V3 {
        let scale_factor : f32 = 1.0 / self.magnitude();
        *self * scale_factor
    }

    pub fn project(&self, source : V3) -> V3 {
        let numerator = self.dot(source);
        let mag = self.magnitude();
        let denominator = mag*mag;
        if denominator < 0.0001 {
            return V3::zero();
        }
        else {
            return (numerator/denominator) * *self;
        }
    }
}

impl ops::Add<V3> for V3 {
    type Output = V3; 
    fn add(self, rh : V3) -> V3{
        V3 {
            x : self.x + rh.x,
            y : self.y + rh.y,
            z : self.z + rh.z,
        }
    }
}

impl ops::Sub<V3> for V3 {
    type Output = V3; 
    fn sub(self, rh : V3) -> V3{
        V3 {
            x : self.x - rh.x,
            y : self.y - rh.y,
            z : self.z - rh.z,
        }
    }
}


impl ops::Mul<f32> for V3 {
    type Output = V3; 
    fn mul(self, rh : f32) -> V3 {
        V3 {
            x : self.x * rh,
            y : self.y * rh,
            z : self.z * rh,
        }
    }
}

impl ops::Mul<V3> for f32 {
    type Output = V3; 
    fn mul(self, rh : V3) -> V3 {
        rh * self
    }
}


