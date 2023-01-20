use crate::vector::V3;
use crate::ray::Ray;
use crate::shape::RayTraceShape;
use crate::color::Col3;
use crate::material::Material;

// A plane spanned by two basis vectors, offset by an origin vector.
#[derive(Clone, Copy)]
pub struct Sphere {
    pub origin : V3,
    pub radius : f32,
    pub material : Material,
}

impl Sphere {
    // normal of the sphere given a point on the sphere
    pub fn normal(&self, point : V3) -> V3 {
        (point - self.origin).normalized()
    }
    
}

impl RayTraceShape for Sphere {

    // My implementation that produces the overlap bug 
    fn intersect(&self, ray : Ray) -> Option<f32> {
        let closest_to_sph_center = ray.dir.project(self.origin - ray.origin);
        let dist_closest_to_center = (closest_to_sph_center + ray.origin).dist(self.origin);
        
        if dist_closest_to_center > self.radius {
            return None;
        }
        else {
            // ray intersects sphere
            let mut flag = false;
            if self.radius*self.radius - dist_closest_to_center*dist_closest_to_center < 0.01 {
                return None;
            }
            let dist_to_interx = (f64::sqrt( (self.radius as f64 * self.radius as f64 - dist_closest_to_center as f64 * dist_closest_to_center as f64)) as f64) as f32;
            if dist_to_interx < 0.001 {
                return None;
            }
            if flag {
                println!("dist to interx {}", dist_to_interx);
            }
            //if true || (ray.origin + ray.dir * dist_to_interx).magnitude() < (ray.origin + ray.dir * -dist_to_interx).magnitude() {
                return Some(f32::abs(dist_to_interx));
            //}
            //else {
            //    return Some(ray.origin + ray.dir * dist_to_interx);
                //return Some(ray.origin - ray.dir * dist_to_interx);
            //}
        }

    
    }
    // v Raytracing in a weekend's implementation v
    /*
    
    fn intersect(&self, r : Ray) -> Option<f32> {
        let oc : V3 = r.origin - self.origin;
        let a = r.dir.magnitude() * r.dir.magnitude();
        let half_b = oc.dot(r.dir);
        let c = oc.magnitude() * oc.magnitude() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;

        if (discriminant < 0.001) {
            return None;
        } else {
            let sqrtd = f32::sqrt(discriminant);
            let mut root = (-half_b - sqrtd) / a;
            if (root < 0.0 || 9999.0 < root) {
                root = (-half_b + sqrtd) / a;
                if (root < 0.0 || 9999.0 < root) {
                    return None; }
            }
            return Some(root);
        }
    }
    */  
    
    fn reflect(&self, ray : &mut Ray, interx : V3) {
        let proj_n_raydir = (V3::zero() - self.normal(interx)).project(ray.dir);
        let new_dir = V3::zero() - ((2.0 * proj_n_raydir) - ray.dir);
        
        ray.dir = new_dir;
        ray.origin = interx;
        self.material.shade_ray_reflection(ray, self.normal(interx));
        ray.bounces_remaining = ray.bounces_remaining - 1;
    }
}
