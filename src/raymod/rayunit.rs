use crate::raymod::*;

use std::f64::consts::*;

pub fn random() -> f64 {
    rand::random::<f64>()
}

#[derive(Debug)]
pub struct Ray {
    pub o: Vec3,
    pub d: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Ray {
        Ray { o, d }
    }
}

pub enum Refl {
    Diff,
    Spec,
    Refr,
}

pub struct Sphere {
    pub rad: f64,
    pub p: Vec3,
    pub e: Vec3,
    pub c: Vec3,
    pub refl: Refl,
}


impl Sphere {
	pub fn new(rad:f64,p:Vec3,e:Vec3,c:Vec3,refl:Refl)->Sphere{
		Sphere {
			rad,
			p,e,c,
			refl,
		}
	}
		
    pub fn intersect(&self, ray: &Ray) -> Option<f64> {
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
            return Some(t1);
        } else {
            return Some(t2);
        }
    }
}

pub struct InterStruct{
    pub b:bool,
    pub t:f64,
    pub id:usize,
}

// #[derive(Default)]
pub struct Scene {
	pub objects: Vec<Sphere>,
}

impl Scene {
    pub fn add(&mut self, obj: Sphere) {
	self.objects.push(obj);
    }

    pub fn init() -> Scene {
	Scene {
	    objects: vec![],
        }
    }
    pub fn intersect(&self, r: &Ray) -> InterStruct {
        let mut ir=InterStruct{b:false,t:INF,id:0};
        for i in (0..self.objects.len() ).rev() {
            match self.objects[i].intersect(r) {
                Some(d)=>{
                    if d < ir.t {
                        ir.t=d;
                        ir.id=i;
                    }
                },
                None=>{},
            }
        }
        ir.b=ir.t<INF;
        return ir;
    }
}
