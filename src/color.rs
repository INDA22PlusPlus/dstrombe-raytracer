use std::ops;
pub struct Col4 {
    pub r : u8,
    pub g : u8,
    pub b : u8,
    pub a : u8,
}

#[derive(Copy, Clone)]
pub struct Col3 {
    pub r : u8,
    pub g : u8,
    pub b : u8,
}

impl ops::Add<Col3> for Col3 {
    type Output = Col3;

    fn add(self, rh : Col3) -> Col3 {
        Col3::new(
            u16::max(self.r as u16 + rh.r as u16, 0xff) as u8,
            u16::max(self.g as u16 + rh.g as u16, 0xff) as u8,
            u16::max(self.b as u16 + rh.b as u16, 0xff) as u8,
        )
    }
}

impl ops::Sub<Col3> for Col3 {
    type Output = Col3;

    fn sub(self, rh : Col3) -> Col3 {
        Col3::new(
            u16::max(self.r as u16 - rh.r as u16, 0xff) as u8,
            u16::max(self.g as u16 - rh.g as u16, 0xff) as u8,
            u16::max(self.b as u16 - rh.b as u16, 0xff) as u8,
        )
    }
}
// multiplicative blending
// ((u8 / 255) * (u8 / 255)) * 255 = u8*u8 / 255
impl ops::Mul<Col3> for Col3 {
    type Output = Col3;
    
    fn mul(self, rh : Col3) -> Col3 {
        Col3::new(
            ((self.r as f32 * rh.r as f32) / 255.0) as u8,
            ((self.b as f32 * rh.b as f32) / 255.0) as u8,
            ((self.g as f32 * rh.g as f32) / 255.0) as u8,
        )
    }
}

impl ops::Mul<f32> for Col3 {
    type Output = Col3;
    
    fn mul(self, rh : f32) -> Col3 {
        Col3::new(
            ((self.r as f32 * rh)) as u8,
            ((self.b as f32 * rh)) as u8,
            ((self.g as f32 * rh)) as u8,
        )
    }
}

impl ops::Mul<Col3> for f32 {
    type Output = Col3;
    
    fn mul(self, rh : Col3) -> Col3 {
        rh * self
    }
}

impl Col3 {
    // returns a color whose constituent colors are 0xff - itself
    pub fn inverse(&self) -> Col3 {
        let mut raw_invert = (Col3::white() - *self);
        raw_invert
    }
    // generate a color as if it was reflected by a surface that has color col. 
    // equivalent to having the inverse absorption spectrum of said color
    pub fn reflect(&self, to_reflect : Col3) -> Col3 {
         self.inverse() * to_reflect
    }

    pub fn white() -> Self {
        Col3::new(
            0xff,
            0xff,
            0xff,
        )
    }

    pub fn black() -> Self {
        Col3::new(
            0x00,
            0x00,
            0x00,
        )
    }

    pub fn new(r : u8, g : u8, b : u8) -> Self {
        Col3 {
            r, 
            g, 
            b,
        }
    }

    
}
