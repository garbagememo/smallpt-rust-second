use crate::raymod::*;

use std::sync::Arc;

pub struct Ray {
    pub o: Vec3,
    pub d: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Ray {
        Ray { o, d }
    }
}
pub struct HitInfo {
    pub t: f64,
    pub p: Vec3,
    pub n: Vec3,
    pub m: Arc<dyn Material>,
}

impl HitInfo {
    pub fn new(t: f64, p: Vec3, n: Vec3, m: Arc<dyn Material>, ) -> Self {
        Self { t, p, n, m, }
    }
}

pub struct Camera{
    p:Vec3,
    d:Vec3,
    plane_dist:f64,
    cx:Vec3,cy:Vec3,
    w:usize,h:usize,
}

impl Camera {
    pub fn new(p:Vec3,d:Vec3,plane_dist:f64,w:usize,h:usize)->Self{
        let cx=Vec3::new((w as f64) * 0.5135 / (h as f64), 0.0, 0.0);
        let cy= (cx % d).norm()*0.5135;
        Camera{p,d,plane_dist,cx,cy,w,h}
    }
    pub fn at(&self,sx:usize,sy:usize,x:usize,y2:usize)->Ray{
        let r1 = 2.0 * random();
        let dx = if r1 < 1.0 { r1.sqrt() - 1.0 } else { 1.0 - (2.0 - r1).sqrt() };
        let r2 = 2.0 * random();
        let dy = if r2 < 1.0 { r2.sqrt() - 1.0 } else { 1.0 - (2.0 - r2).sqrt() };
        let d = self.cx * ((((sx as f64) + 0.5 + dx) / 2.0 + (x as f64)) / (self.w as f64) - 0.5)
            + self.cy * ((((sy as f64) + 0.5 + dy) / 2.0 + (y2 as f64)) / (self.h as f64)  - 0.5)
            + self.d;
        Ray{ o:self.p + d * self.plane_dist, d:d.norm()}
    }
}

pub struct Sphere {
    pub rad: f64,
    pub p: Vec3,
    pub m: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(rad: f64, p: Vec3, m: Arc<dyn Material>) -> Sphere {
        Sphere { rad, p, m }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        let po = self.p - ray.o;
        let b = po.dot(&ray.d);
        let d4 = b * b - po.dot(&po) + self.rad * self.rad;

        if d4 < 0.0 {
            return None;
        }

        let sqrt_d4 = d4.sqrt();
        let t1 = b - sqrt_d4;
        let t2 = b + sqrt_d4;

        if t1 < EPS && t2 < EPS {
            return None;
        }

        if t1 > EPS {
           let p = ray.o+ray.d*t1;
           let n = (p - self.p) / self.rad;
           return Some(HitInfo::new(t1, p, n, Arc::clone(&self.m),));
        } else {
           let p = ray.o+ray.d*t2;
           let n = (p - self.p) / self.rad;
           return Some(HitInfo::new(t2, p, n, Arc::clone(&self.m),));
        }
    }
}


pub struct Scene {
    pub objects: Vec<Sphere>,
    pub model_name: String,
}

impl Scene {
    pub fn add(&mut self, obj: Sphere) {
        self.objects.push(obj);
    }

    pub fn new() -> Scene {
        Scene {
            objects: vec![],
            model_name: String::from("null"),
        }
    }
    pub fn intersect(&self, r: &Ray) -> Option<HitInfo>{
        let mut hit_info: Option<HitInfo> = None;
        let mut closest_so_far = INF;
        for object in &self.objects {
            if let Some(info) = object.intersect(&r,) {
                if closest_so_far>info.t {
                    closest_so_far = info.t;
                    hit_info = Some(info);
                }
            }
        }
        hit_info
    }
}
