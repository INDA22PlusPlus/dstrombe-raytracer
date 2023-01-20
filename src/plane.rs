use crate::vector::V3;
use crate::ray::Ray;
use crate::shape::RayTraceShape;
use crate::color::Col3;
use crate::material::Material;

// A plane spanned by two basis vectors, offset by an origin vector.
#[derive(Clone, Copy)]
pub struct Plane {
    pub origin : V3,
    pub base_one : V3,
    pub base_two : V3,
    pub material : Material,
}

impl Plane {
    pub fn normal(&self) -> V3 {
        self.base_one.cross(self.base_two)
    }
    pub fn project(&self, source : V3) -> V3 {
        let perp_pi_v = self.normal().project(source);
        let proj_pi_v = source - perp_pi_v;
        proj_pi_v
    }
}

impl RayTraceShape for Plane {
    fn intersect(&self, ray : Ray) -> Option<f32> {
        let normal = self.normal();
        if f32::abs(normal.dot(ray.dir)) < 0.0001 {
            return None;
        }

        let t = -normal.dot(ray.origin - self.origin) / normal.dot(ray.dir);
        if t > 0.001 {
            Some(t)
        }
        else {
            None
        }
    
    }

    fn reflect(&self, ray : &mut Ray, interx : V3) {
        let proj_n_raydir = (V3::zero() + self.normal()).project(ray.dir);
        let new_dir = V3::zero() + ((2.0 * proj_n_raydir) - ray.dir); // reflection formula

        ray.origin = interx;
        ray.dir = new_dir;
        self.material.shade_ray_reflection(ray, self.normal());
        ray.bounces_remaining = ray.bounces_remaining - 1;
    }
}
