use crate::raymod::*;

pub struct TraceInfo {
    pub r:Ray,
    pub cpc:f64,//Color Probability Correction
}
impl TraceInfo{
    pub fn new(r:Ray,cpc:f64)->Self{
        Self{r,cpc}
    }
}

pub trait Material: Sync + Send {
    fn value(&self) -> Color;
    fn emitted(&self) -> Color;
    fn trace_ray(&self,r:&Ray,n:Vec3,nl:Vec3,o:Vec3)->TraceInfo ;
}

pub struct Diffuse {
    pub color: Vec3,
    pub emit:Vec3,
}

impl Diffuse {
    pub const fn new(emit:Vec3,color: Vec3) -> Self {
        Self { color,emit }
    }
}

impl Material for Diffuse {
    fn value(&self) -> Color {
        self.color
    }
    fn emitted(&self) -> Color {
        self.emit
    }
    fn trace_ray(&self,_r:&Ray,_n:Vec3,nl:Vec3,o:Vec3)->TraceInfo {
        let r1 = 2.0 * std::f64::consts::PI * random();
        let r2 = random();
        let r2s = r2.sqrt();
        let w = nl;
        let u = ((if w.x.abs() > 0.1 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        }) % w)
            .norm();
        let v = w % u;
        let d = (u * f64::cos(r1) * r2s + v * f64::sin(r1) * r2s + w * (1.0 - r2).sqrt()).norm();
        TraceInfo::new(Ray::new(o,d),1.0)
    }
}

pub struct Mirror {
    pub color: Vec3,
    pub emit: Vec3,
}

impl Mirror {
    pub fn new(emit:Vec3,color:Vec3) -> Self {
        Self { color,emit }
    }
}
impl Material for Mirror {
    fn value(&self) -> Color {
        self.color
    }
    fn emitted(&self)->Color{
        self.emit
    }
    fn trace_ray(&self,r:&Ray,n:Vec3,_nl:Vec3,o:Vec3)->TraceInfo{
       TraceInfo::new(Ray::new(o, r.d - n * 2.0 * n.dot(&r.d)),1.0)
    }
}

pub struct Refract {
    pub color:Vec3,
    pub emit:Vec3,
}

impl Refract {
    pub const fn new(emit:Vec3,color:Vec3) -> Self {
        Self { emit,color }
    }
}

impl Material for Refract {
    fn value(&self) -> Color {
        self.color
    }
    fn emitted(&self)->Color{
        self.emit
    }
    fn trace_ray(&self,r:&Ray,n:Vec3,nl:Vec3,o:Vec3)->TraceInfo{
        // Refl.Refr
        let refl_ray = Ray::new(o, r.d - n * 2.0 * n.dot(&r.d));
        let into = n.dot(&nl) > 0.0;
        let nc = 1.0;
        let nt = 1.5;
        let nnt = if into { nc / nt } else { nt / nc };
        let ddn = r.d.dot(&nl);
        let cos2t = 1.0 - nnt * nnt * (1.0 - ddn * ddn);
        if cos2t < 0.0 {
           TraceInfo::new(refl_ray,1.0)
        } else {
            let tdir =
                r.d * nnt - n * ((if into { 1.0 } else { -1.0 }) * (ddn * nnt + cos2t.sqrt()));
            tdir.norm();
            let a = nt - nc;
            let b = nt + nc;
            let r0 = a * a / (b * b);
            let c = 1.0 - (if into { -ddn } else { tdir.dot(&n) });
            let re = r0 + (1.0 - r0) * c * c * c * c * c;
            let tr = 1.0 - re;
            let p = 0.25 + 0.5 * re;
            let rp = re / p;
            let tp = tr / (1.0 - p);
            if random() < p {
                TraceInfo::new(refl_ray,rp)
            } else {
                TraceInfo::new(Ray::new(o, tdir),tp)
            }
        }
    }
}


