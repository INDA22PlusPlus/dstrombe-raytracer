use crate::vector::V3;
use crate::ray::Ray;
use crate::color::Col3;
use crate::shape::RayTraceShape;
use crate::matrix::Matrix3;
use rayon::prelude::*;

// TODO
// bright rays bloom onto adjacent pixels with gaussian convolution

pub struct Scene {
    pub geometry : Vec<Box<dyn RayTraceShape>>,
    pub camera : Camera
}

pub struct Camera {
    pub location : V3,
    pub rotation_y : f32,
    pub rotation_x : f32,
    pub viewport_anchor : V3, // relative to the camera. The botleft corner of the viewport in camera
                          // space
    pub size_x : u16,
    pub size_y : u16,
    pub rays_per_pixel : u16,
    pub bounce_depth : u16,
    pub max_steps : u16, 
    pub step_len : f32, // discretization of the ray allows for the modelling of non linear space
}

impl Camera {
    // returns all the rays for a given pixel, which need to be simulated by the caller. 
    pub fn generate_rays_for_pixel(&self, x_idx : u16, y_idx : u16) -> Vec<Ray> {
        // base ray
        
        let pixel_stride = (self.viewport_anchor.x * 2.0) as f32 / self.size_x as f32;

        let mut viewport_interx_camera_space = self.viewport_anchor + self.location;         //randomness, to prevent outlier rays
        let ray_origin = viewport_interx_camera_space;
        //viewport_interx_camera_space = viewport_interx_camera_space +         

        let ray_dir = viewport_interx_camera_space; 
        // TODO transform ray_dir with change-of-basis matrix so that self.direction is used

        let ray_color = Col3::white();
        
        let mut rays = Vec::with_capacity(self.rays_per_pixel as usize);
       
        // sorry for the long line
        // TODO: split up the rotation transforms etc
        for i in 0..self.rays_per_pixel {
            rays.push(Ray {
                origin : ray_origin,
                dir : Matrix3::rotation_x(self.rotation_x).transform_vec3(Matrix3::rotation_y(self.rotation_y).transform_vec3((ray_dir + pixel_stride * V3::new(x_idx as f32, y_idx as f32, 0.0) + pixel_stride * V3::new(fastrand::f32() - 0.5, fastrand::f32() - 0.5, 0.0)).normalized())),
                color : ray_color,
                bounces_remaining : self.bounce_depth,
                steps_remaining : self.max_steps,
                gamma : 0.2
            });
        }

        rays
    }
}

impl Scene {
    pub fn render(&self) -> Vec<Col3> {
        // TODO write TODO
        let mut to_ret : Vec<Col3> = Vec::new();
        for y in 0..self.camera.size_y {
            for x in (0..self.camera.size_x) {
                let rays = self.camera.generate_rays_for_pixel(x, y);
                let mut pix_col_r : u64 = 0;
                let mut pix_col_g : u64 = 0;
                let mut pix_col_b : u64 = 0;
                for mut ray in rays {
                    let pix_col = self.path_trace_ray(&mut ray);
                     
                    pix_col_r = pix_col_r + (pix_col.r as f32 * (ray.gamma)) as u64;
                    pix_col_g = pix_col_g + (pix_col.g as f32 * (ray.gamma)) as u64;
                    pix_col_b = pix_col_b + (pix_col.b as f32 * (ray.gamma)) as u64;
                }
                to_ret.push(Col3 {
                    r : u64::min((pix_col_r / self.camera.rays_per_pixel as u64), 255) as u8,
                    g : u64::min((pix_col_g / self.camera.rays_per_pixel as u64), 255) as u8,
                    b : u64::min((pix_col_b / self.camera.rays_per_pixel as u64), 255) as u8,
                });
            }
        }
        to_ret
    }
    
    fn path_trace_ray(&self, ray : &mut Ray) -> Col3 {
        let mut closest_interx : Option<(f32, &Box<dyn RayTraceShape>)> = None;
        for shape in &self.geometry {
            match shape.intersect(*ray) {
                Some(interx) => {
                    if closest_interx.is_none() || closest_interx.unwrap().0 > interx {
                        closest_interx = Some((interx, shape));
                    }
                    
                }
                None => {
                    
                }
            }
        }

        match closest_interx {
           
            Some((interx, shape)) => {
                if false && interx > self.camera.step_len * 5.0 {
                    ray.origin = ray.origin + ray.dir * self.camera.step_len * 0.95;
                    if ray.steps_remaining <= 0 {
                        return Col3::black();
                    }
                    ray.steps_remaining = ray.steps_remaining - 1;
                    return self.path_trace_ray(ray);
                }
                ray.steps_remaining = self.camera.max_steps;
                shape.reflect(ray, ray.origin + ray.dir * interx);
                if ray.bounces_remaining <= 0 {
                    
                    if ray.gamma > 0.0 {
                        return ray.color;
                    }
                    else {
                        return Col3::black();
                    }
                }
                else {
                    return self.path_trace_ray(ray);
                }
            }
            None => {
                return Col3::black();
            }
        }
    }
}

