use crate::math::*;
use crate::canvas::Canvas;
use crate::prelude::{PI, pt};
use crate::kolor::rgb8;
use tiny_skia::Color;

pub struct SphereScene<'a> {
    pub camera: Point3,
    pub center: Point3,
    pub radius: f32,
    pub texture: &'a Canvas,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub rotation_z: f32,
    pub lights: Vec<Light>,
    pub specular: Option<f32>,
}

impl<'a> SphereScene<'a> {
    pub fn new(center: Point3, radius: f32, texture: &'a Canvas) -> Self {
        Self {
            camera: Point3::new(0.0, 0.0, 0.0),
            center,
            radius,
            texture,
            rotation_x: PI / 2.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
            lights: vec![],
            specular: None,
        }
    }

    pub fn color(&self, point: Point3) -> Color {
        let w = 2.0 * self.texture.width() as f32;
        let h = self.texture.height() as f32;
        let rot_point = (point - self.center)
            .rotate_x(self.rotation_x)
            .rotate_y(self.rotation_y)
            .rotate_z(self.rotation_z)
            + self.center;
        let s = rot_point.to_spherical(self.center);
        let u = w / 2.0 * (s.phi / PI + 1.0);
        let v = h * (1.0 - s.theta / PI);
        let c = self.texture.pixel(u as u32, v as u32).unwrap();
        let mut illumination = if self.lights.is_empty() {
            1.0
        } else {
            let mut normal = point - self.center;
            normal = normal * (1.0 / normal.dot_prod(&normal).sqrt());
            lighting(&self.lights, point, normal, self.center, self.specular)
        };
        illumination = illumination.clamp(0.0, 1.0);
        let red = c.red() as f32 * illumination;
        let green = c.green() as f32 * illumination;
        let blue = c.blue() as f32 * illumination;
        rgb8(red as u8, green as u8, blue as u8)
    }

    pub fn intersect(&self, direction: Point3) -> Option<(f32, f32)> {
        let w = self.camera - self.center;
        let a = direction.dot_prod(&direction);
        let b = 2.0 * w.dot_prod(&direction);
        let c = w.dot_prod(&w) - self.radius * self.radius;
        let discr = b * b - 4.0 * a * c;
        if discr < 0.0 {
            return None;
        }
        let t1 = (-b + discr.sqrt()) / (2.0 * a);
        let t2 = (-b - discr.sqrt()) / (2.0 * a);
        Some((t1, t2))
    }

    pub fn trace_ray(&self, direction: Point3) -> Option<Color> {
        if let Some((t1, t2)) = self.intersect(direction) {
            let t = t1.min(t2);
            let p = self.camera + direction * t;
            return Some(self.color(p));
        }
        return None;
    }

    pub fn on_sphere(&self, canvas: &mut Canvas) {
        let cw2 = canvas.width() as i32 / 2;
        let ch2 = canvas.height() as i32 / 2;
        for x in 1 - cw2..cw2 {
            let x32 = x as f32;
            for y in 1 - ch2..ch2 {
                let y32 = y as f32;
                let d = Point3::new(x32, y32, canvas.width() as f32) - self.camera;
                if let Some(c) = self.trace_ray(d) {
                    let p = pt(
                        canvas.width() as f32 / 2.0 + x32,
                        canvas.height() as f32 / 2.0 - y32,
                    );
                    canvas.dot(p.x, p.y, c);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LightSource {
    Ambient,
    Point,
    Directional,
}

#[derive(Debug, Clone, Copy)]
pub struct Light {
    source: LightSource,
    intesity: f32,
    vector: Point3,
}

impl Light {
    pub fn new(source: LightSource, intesity: f32, vector: Point3) -> Self {
        Self {
            source,
            intesity,
            vector,
        }
    }
}

pub fn lighting(
    lights: &Vec<Light>,
    point: Point3,
    normal: Point3,
    camera: Point3,
    specular: Option<f32>,
) -> f32 {
    let mut intensity = 0.0;
    for light in lights {
        let light_vec = match light.source {
            LightSource::Ambient => Point3::new(0.0, 0.0, 0.0),
            LightSource::Point => light.vector - point,
            LightSource::Directional => light.vector,
        };
        match light.source {
            LightSource::Ambient => intensity += light.intesity,
            LightSource::Point | LightSource::Directional => {
                let nl = normal.dot_prod(&light_vec);
                let lv = light_vec.mag();
                intensity += light.intesity * nl.max(0.0) / lv;
                if let Some(s) = specular {
                    let r = (normal * 2.0 * normal.dot_prod(&light_vec)) - light_vec;
                    let rv = r.dot_prod(&(point - camera));
                    if rv > 0.0 {
                        intensity +=
                            light.intesity * (rv / (r.mag() * (point - camera).mag())).powf(s)
                    }
                };
            }
        }
    }
    intensity
}
