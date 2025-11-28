mod raymod;
use raymod::*;

use rayon::prelude::*;

use std::io::Write;

fn radiance(r: &Ray, depth: u8,	scene: &Scene) -> Vec3 {
    let ir=scene.intersect(r);
    if ir.b==false {
        return Vec3::zero();
    }
    let obj = &scene.objects[ir.id];
    let x = r.o + r.d * ir.t;
    let n = (x - obj.p).norm();
    let nl = if n.dot(&r.d) < 0.0 { n } else { n * -1.0 };
    let mut f = obj.m.value();
    let p = f64::max(f.x,f64::max(f.y, f.z));
    let depth = depth + 1;
    if depth > 5 {
        if depth < 127 && random() < p {
            f = f * (1.0 / p);
        } else {
            return obj.m.emitted();
        }
    }
    let m_info = obj.m.trace_ray(&r,n,nl,x);
    obj.m.emitted() + f.mult(&radiance(&m_info.r, depth,scene))*m_info.cpc
}

struct Camera{
    p:Vec3,
    d:Vec3,
    PlaneDist:f64,
    cx:Vec3,cy:Vec3,
    w:usize,h:usize,
}

impl Camera {
    fn new(p:Vec3,d:Vec3,PlaneDist:f64,w:usize,h:usize)->Self{
        let cx=Vec3::new((w as f64) * 0.5135 / (h as f64), 0.0, 0.0);
        let cy= (cx % d).norm()*0.5135;
        Camera{p,d,PlaneDist,cx,cy,w,h}
    }
    fn at(&self,sx:usize,sy:usize,x:usize,y2:usize)->Ray{
        let r1 = 2.0 * random();
        let dx = if r1 < 1.0 { r1.sqrt() - 1.0 } else { 1.0 - (2.0 - r1).sqrt() };
        let r2 = 2.0 * random();
        let dy = if r2 < 1.0 { r2.sqrt() - 1.0 } else { 1.0 - (2.0 - r2).sqrt() };
        let d = self.cx * ((((sx as f64) + 0.5 + dx) / 2.0 + (x as f64)) / (self.w as f64) - 0.5)
            + self.cy * ((((sy as f64) + 0.5 + dy) / 2.0 + (y2 as f64)) / (self.h as f64)  - 0.5)
            + self.d;
        Ray{ o:self.p + d * self.PlaneDist, d:d.norm()}
    }
}
impl Scene {
    fn RandomScene(& mut self,w:usize,h:usize)->Camera{
        let Cen  = Vec3::new(50.0,40.8,-860.0);
        let Cen1 = Vec3::new(75.0,25.0, 85.0);
        let Cen2 = Vec3::new(45.0,25.0, 30.0);
        let Cen3 = Vec3::new(15.0,25.0,-25.0);
        self.add(Sphere::new(10000.0,  Cen+Vec3::new(0.0,0.0,-200.0)  ,
                             Box::new(Diffuse::new(Vec3::new(0.6, 0.5, 0.7)*0.8, Vec3::new(0.7,0.9,1.0))),  )); // sky
        self.add(Sphere::new(100000.0, Vec3::new(50.0, -100000.0, 0.0),
                             Box::new(Diffuse::new(BLACK, Vec3::new(0.4,0.4,0.4))),)); // grnd
        

        self.add(Sphere::new(25.0,  Cen1 ,
                             Box::new(Mirror::new(BLACK,Vec3::new(0.9,0.9,0.9))), ));// mirror
        self.add(Sphere::new(25.0,  Cen2 ,
                             Box::new(Refract::new(BLACK,Vec3::new(0.95,0.95,0.95))), )); // glass
        self.add(Sphere::new(25.0,  Cen3 ,
                             Box::new(Diffuse::new(BLACK,Vec3::new(1.0,0.6,0.6)*0.696)), ));    // 乱反射
        for a in -11 .. 12 {
            for b in -11 .. 12 {
                let RandomMatterial = random();
                let Cen = Vec3::new( ((a as f64)+random())*25.0,5.0,((b as f64)+random() )*25.0);
                if (Cen - Cen1) .length().sqrt()>(25.0*1.0) {
                    if RandomMatterial<0.8 {
                        self.add(Sphere::new(5.0,Cen,
                                             Box::new(Diffuse::new(BLACK,Vec3::new(random(),random(),random()))),));
                    } else if RandomMatterial <0.95 {
                        self.add(Sphere::new(5.0,Cen,
                                             Box::new(Mirror::new(BLACK,Vec3::new(random(),random(),random()))),));
                    } else {
                        self.add(Sphere::new(5.0,Cen,
                                             Box::new(Refract::new(BLACK,Vec3::new(random(),random(),random()) )),));
                    }
                }
            }
        }
        return Camera::new(
            Vec3::new(55.0, 58.0, 245.6),
            Vec3::new(0.0, -0.24, -1.0).norm(),
            50.0,
            w,h);
    }
}

fn main() {

    let args = parameters();
    println!("{:?}", args);
    

    
    let w: usize = args.w;
    let h: usize = (480.0*w as f64/640.0) as usize;
    let samps = args.s;
    println!("samps={}",samps);

    let mut cam = Camera::new(
        Vec3::new(50.0, 52.0, 295.6),
        Vec3::new(0.0, -0.042612, -1.0).norm(),
        140.0,
        w,h);
    let mut scene=Scene::new();
    match args.m{
        0=> scene.model_init0(),
        2=> scene.model_init2(),
        3=> scene.model_init3(),
        4=> scene.model_init4(),
        5=> scene.model_init5(),
        6=> scene.model_init6(),
        7=> scene.model_init7(),
        8=> scene.model_init8(),
        8=> scene.model_init8(),
        9=> scene.model_init9(),
        10=>{cam=scene.RandomScene(w,h);
             true}
        _=> scene.model_init0(),
    };
    println!("Model Name = {}",scene.model_name);

    let mut image = vec![Color::zero(); (w * h) as usize];

    let bands: Vec<(usize, &mut [Color])> = image.chunks_mut(w as usize).enumerate().collect();
    bands.into_par_iter().for_each(|(y, band)| {
        let y2 = h - (y as usize) - 1;
        if (y % 10) == 0 {
            writeln!(
                std::io::stderr(),
                "Rendering ({} spp) {:5.2}%",
                samps * 4,
                100.0 * (y as f64) / ((h as f64) - 1.0)
            )
                .unwrap();
        }
        for x in 0..w {
            let mut r = Vec3::zero();
            for sy in 0..2 {
                for sx in 0..2 {
                    for _s in 0..samps {
                        let ray = cam.at(sx,sy,x,y2);
                        r = r + radiance(&(ray), 0,&scene)
                            * (1.0 / (samps as f64));
                    }
                    band[x as usize] = band[x as usize] + r * (1.0 / 4.0 as f64);
                    r = Vec3::zero();
                }
            }
        }
    });

    save_ppm_file2(&args.output, image, w, h);
}
