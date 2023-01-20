use crate::vector::V3;
use crate::color::{Col3, Col4};
#[derive(Clone, Copy)]
pub struct Ray {
    pub origin : V3,
    pub dir : V3,
    pub color : Col3,
    pub bounces_remaining : u16,
    pub steps_remaining : u16, // discrete ray ticks, the ray ticks forward
    pub gamma : f32, // HDR brightness boost technology (tm)
}

impl Ray {
    pub fn new(origin : V3, dir : V3, color : Col3, bounce_depth : u16, step_timeout : u16) -> Ray {
        Ray {
            origin,
            dir,
            color,
            bounces_remaining : bounce_depth,
            steps_remaining : step_timeout,
            gamma : 0.0,
        }
    }
    
    // pub fn intersect() -> (V3, f32)
}
