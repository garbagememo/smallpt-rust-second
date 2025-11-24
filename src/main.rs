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
    let trace_info = obj.m.trace_ray(&r,n,nl,x);
    obj.m.emitted()+f.mult(&radiance(&trace_info.r,depth,scene))*trace_info.revise
}

fn main() {

    let args = parameters();
    println!("{:?}", args);
    
	let mut scene=Scene::init();
    scene.model_init0();

	
    let w: usize = args.w;
    let h: usize = (480.0*w as f64/640.0) as usize;
    let samps = args.s;
    println!("samps={}",samps);

    let cam = Ray::new(
        Vec3::new(50.0, 52.0, 295.6),
        Vec3::new(0.0, -0.042612, -1.0).norm(),
    );

    let cx = Vec3::new((w as f64) * 0.5135 / (h as f64), 0.0, 0.0);
    let cy = (cx % cam.d).norm() * 0.5135;
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
                        let r1 = 2.0 * random();
                        let dx = if r1 < 1.0 {
                            r1.sqrt() - 1.0
                        } else {
                            1.0 - (2.0 - r1).sqrt()
                        };
                        let r2 = 2.0 * random();
                        let dy = if r2 < 1.0 {
                            r2.sqrt() - 1.0
                        } else {
                            1.0 - (2.0 - r2).sqrt()
                        };
                        let d = cx
                            * ((((sx as f64) + 0.5 + dx) / 2.0 + (x as f64)) / (w as f64) - 0.5)
                            + cy * ((((sy as f64) + 0.5 + dy) / 2.0 + (y2 as f64)) / (h as f64) - 0.5)
                            + cam.d;
                        r = r + radiance(&(Ray::new(cam.o + d * 140.0, d.norm())), 0,&scene) / (samps as f64);
                    }
                    band[x as usize] = band[x as usize] + r * (1.0 / 4.0 as f64);
                    r = Vec3::zero();
                }
            }
        }
    });

    save_ppm_file("image.ppm", image, w, h);
}
