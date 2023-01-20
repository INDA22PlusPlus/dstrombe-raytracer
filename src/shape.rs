use crate::ray::Ray;
use crate::vector::V3;

pub trait RayTraceShape {
    fn intersect(&self, ray : Ray) -> Option<f32>;
    fn reflect(&self, ray : &mut Ray, interx : V3);
}
