use crate::vector::V3;
use crate::color::Col3;
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct Material {
    pub albedo : f32,
    pub smoothness : f32,
    pub color : Col3,
    pub brightness : f32,
}

impl Material {
    pub fn new(albedo : f32, smoothness : f32, color : Col3, brightness : f32) -> Self {
        Material {
            albedo,
            smoothness,
            color,
            brightness,
        }
    }
    pub fn shade_ray_reflection(&self, ray : &mut Ray, surface_normal : V3) {
        self.displace_ray(ray, surface_normal);
        self.shade_color(ray);
        self.shade_lighting(ray);
    }

    fn shade_lighting(&self, ray : &mut Ray) {
        ray.gamma = self.brightness;// * ray.gamma;
        if self.brightness > 1.9 {
            ray.bounces_remaining = 1;
        }
    }

    fn shade_color(&self, ray : &mut Ray) {
        let reflected = ray.color * self.color;
        let intrinsic = self.color;

        ray.color = reflected; //* (1.0 - self.albedo) + intrinsic * self.albedo;
    }

    fn displace_ray(&self, ray : &mut Ray, surface_normal : V3) {
        // TODO: replace with real pi
        let pi = 3.141052;
        
        // plane in parametric form, as derived from its normal vector 
        let basis_t = V3::new(surface_normal.y, 1.0, 0.0).normalized();
        let basis_s = V3::new(surface_normal.z, 0.0, 1.0).normalized();
        
        // combine all the bases + form a disk in 3d space
        let rand_disk = (basis_t * f32::cos(fastrand::f32() * pi) + basis_s * f32::cos(fastrand::f32() * pi));
        ray.dir = (ray.dir.normalized() * self.smoothness + rand_disk * (1.0-self.smoothness)).normalized();
    }
}
