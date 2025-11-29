mod raymod;
use raymod::*;
use rayon::prelude::*;
use std::io::Write;
use std::time::*;

fn radiance(r: &Ray, depth: u8,	scene: &Scene) -> Vec3 {
    let hit_info = scene.intersect(r);
    if let Some(info)=hit_info {
        let nl = if info.n.dot(&r.d) < 0.0 { info.n } else { info.n * -1.0 };
        let mut f = info.m.value();
        let p = f64::max(f.x,f64::max(f.y, f.z));
        let depth = depth + 1;
        if depth > 5 {
            if depth < 127 && random() < p {
                f = f * (1.0 / p);
            } else {
                return info.m.emitted();
            }
        }
        let m_info = info.m.trace_ray(&r,info.n,nl,info.p);
        info.m.emitted() + f.mult(&radiance(&m_info.r, depth,scene))*m_info.cpc
    } else {
        return BLACK;
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
        0=> {scene.model_init0();},
//        2=> scene.model_init2(),
//        3=> scene.model_init3(),
//        4=> scene.model_init4(),
//        5=> scene.model_init5(),
//        6=> scene.model_init6(),
//        7=> scene.model_init7(),
//        8=> scene.model_init8(),
//        9=> scene.model_init9(),
        10=>{cam=scene.random_scene(w,h);},
        _=> scene.model_init0(),
    };
    println!("Model Name = {}",scene.model_name);

    let mut image = vec![Color::zero(); (w * h) as usize];

    println!("-> 処理を開始します...");
    let start = Instant::now();

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
    println!("-> 処理を終了しました...");
    let duration = start.elapsed();
    println!("   秒: {:.4}s", duration.as_secs_f64());
    
    save_ppm_file2(&args.output, image, w, h);
}
