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
		
    pub fn intersect(&self, r: &Ray) -> f64 {
        let op = self.p - r.o;
        let b = op.dot(&r.d);
        let mut det = b * b - op.dot(&op) + self.rad * self.rad;
        if det < 0.0 {
            return INF;
        }
        det = det.sqrt();
        let t = b - det;
        if t > EPS {
            return t;
        }
        let t = b + det;
        if t > EPS {
            return t;
        } else {
            return INF;
        }
    }
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
pub fn init2(&mut self)->bool{
    self.objects.push( Sphere::new( 1e5,   Vec3::new( 1e5 + 1.0,     40.8, 81.6),Vec3::zero(),                Vec3::new(0.75, 0.25, 0.25), Refl::Diff ));//left
   self.objects.push( Sphere::new( 1e5,   Vec3::new(-1e5 + 99.0,    40.8, 81.6),Vec3::zero(),                Vec3::new(0.25, 0.25, 0.75), Refl::Diff ));//right
   self.objects.push( Sphere::new( 1e5,   Vec3::new(50.0,            40.8, 1e5),Vec3::zero(),                Vec3::new(0.75, 0.75, 0.75), Refl::Diff ));//front
   self.objects.push( Sphere::new( 1e5,   Vec3::new(50.0,    40.8,-1e5 + 170.0),Vec3::zero(),                Vec3::zero(), Refl::Diff ));//back
   self.objects.push( Sphere::new( 1e5,   Vec3::new(50.0,            1e5, 81.6),Vec3::zero(),                Vec3::new(0.75, 0.75, 0.75), Refl::Diff ));//bottom
   self.objects.push( Sphere::new( 1e5,   Vec3::new(50.0,-1e5 + 81.6+4.0, 81.6),Vec3::zero(),                Vec3::new(0.75, 0.75, 0.75), Refl::Diff ));//top
   self.objects.push( Sphere::new( 16.5,  Vec3::new(27.0,           16.5, 47.0),Vec3::zero(),                Vec3::new(1.0, 1.0, 1.0) * 0.999, Refl::Spec));
   self.objects.push( Sphere::new( 16.5,  Vec3::new(73.0,           16.5, 78.0),Vec3::zero(),                Vec3::new(1.0, 1.0, 1.0) * 0.999, Refl::Refr));
   self.objects.push( Sphere::new( 600.0, Vec3::new(50.0, 681.6-0.27+4.0, 81.6),Vec3::new(12.0, 12.0, 12.0), Vec3::zero(), Refl::Diff));
   true
}


    pub fn intersect(&self,r: &Ray, t: &mut f64, id: &mut usize) -> bool {
       let n = self.objects.len();
       *t = INF + 20.0;
       for i in (0..n).rev() {
           let d = self.objects[i].intersect(r);
		   if d < *t {
			   *t = d;
               *id = i;
           }
       }
       return *t < INF; 
    }
}
	
