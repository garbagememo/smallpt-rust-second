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

pub trait Shape: Sync {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> ;
    fn bounding_box(&self) -> Option<AABB>;
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
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
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
    fn bounding_box(&self) -> Option<AABB> {
        let radius = Vec3::new(self.rad, self.rad, self.rad);
        let min = self.p - radius;
        let max = self.p + radius;
        Some(AABB { min, max })
    }
}


pub struct Scene {
    pub objects: Vec<Box<dyn Shape>>,
    pub model_name: String,
}

impl Scene {
    pub fn add(&mut self, obj: Box<dyn Shape>) {
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
