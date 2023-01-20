

pub struct point_light {
    point : V3,
    radius : f32,
}

impl RayTraceShape for point_light {
    fn intersect(&self, ray : Ray) -> Option
}

pub struct directional_light {
    direction : V3,
}

