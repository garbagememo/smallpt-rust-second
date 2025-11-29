use crate::raymod::*;

use std::f64::consts::*;
use std::sync::Arc;

// #[derive(Default)]

impl Scene {
    pub fn model_init0(&mut self)  {
        //-------------Debug Scene sc1-------------
        self.model_name = String::from("Cornell");
        self.add(Sphere::new(
            1e5,
            Vec3::new(1e5 + 1.0, 40.8, 81.6),
            Arc::new(Diffuse::new(BLACK,Vec3::new(0.75, 0.25, 0.25))),
        )); //left
        self.add(Sphere::new(
            1e5,
            Vec3::new(-1e5 + 99.0, 40.8, 81.6),
            Arc::new(Diffuse::new(BLACK,Vec3::new(0.25, 0.25, 0.75))),
        )); //right
        self.add(Sphere::new(
            1e5,
            Vec3::new(50.0, 40.8, 1e5),
            Arc::new(Diffuse::new(BLACK,Vec3::new(0.75, 0.75, 0.75))),
        )); //front
        self.add(Sphere::new(
            1e5,
            Vec3::new(50.0, 40.8, -1e5 + 170.0),
            Arc::new(Diffuse::new(BLACK,BLACK)),
        )); //back
        self.add(Sphere::new(
            1e5,
            Vec3::new(50.0, 1e5, 81.6),
            Arc::new(Diffuse::new(BLACK,Vec3::new(0.75, 0.75, 0.75))),
        )); //bottom
        self.add(Sphere::new(
            1e5,
            Vec3::new(50.0, -1e5 + 81.6 + 4.0, 81.6),
            Arc::new(Diffuse::new(BLACK,Vec3::new(0.75, 0.75, 0.75))),
        )); //top
        self.add(Sphere::new(
            16.5,
            Vec3::new(27.0, 16.5, 47.0),
            Arc::new(Mirror::new(BLACK,Vec3::new(1.0, 1.0, 1.0) * 0.999)),
        )); //mirror
        self.add(Sphere::new(
            16.5,
            Vec3::new(73.0, 16.5, 78.0),
            Arc::new(Refract::new(BLACK,Vec3::new(1.0, 1.0, 1.0) * 0.999)),
        )); //透明体
        self.add(Sphere::new(
            600.0,
            Vec3::new(50.0, 681.6 - 0.27 + 4.0, 81.6),
            Arc::new(Diffuse::new(Vec3::new(12.0, 12.0, 12.0), BLACK)),
        )); //light
    }
    
    pub fn model_init2(&mut self)  {
        //-----------sky sc2--------------
        self.model_name = String::from("Sky");
        let cen: Vec3 = Vec3 {
            x: 50.0,
            y: 40.8,
            z: -860.0,
        };
        self.add(Sphere::new(
            1600.0,
            Vec3::new(1.0, 0.0, 2.0) * 3000.0,
            Arc::new(Diffuse::new(
                Vec3::new(1.0, 0.9, 0.8) * 1.2e1 * 1.56 * 2.0,
                BLACK,
            )),
        )); // sun
        self.add(Sphere::new(
            1560.0,
            Vec3::new(1.0, 0.0, 2.0) * 3500.0,
            Arc::new(Diffuse::new(
                Vec3::new(1.0, 0.5, 0.05) * 4.8e1 * 1.56 * 2.0,
                BLACK,
            )),
        )); // horizon sun2
        self.add(Sphere::new(
            10000.0,
            cen + Vec3::new(0.0, 0.0, -200.0),
            Arc::new(Diffuse::new(
                Vec3::new(0.00063842, 0.02001478, 0.28923243) * 6e-2 * 8.0,
                Vec3::new(0.7, 0.7, 1.0) * 0.25,
            )),
        )); // sky

        self.add(Sphere::new(
            100000.0,
            Vec3::new(50.0, -100000.0, 0.0),
            Arc::new(Diffuse::new(BLACK,Vec3::new(0.3, 0.3, 0.3))),
        )); // grnd
        self.add(Sphere::new(
            110000.0,
            Vec3::new(50.0, -110048.5, 0.0),
            Arc::new(Diffuse::new(Vec3::new(0.9, 0.5, 0.05) * 4.0, BLACK)),
        )); // horizon brightener
        self.add(Sphere::new(
            4e4,
            Vec3::new(50.0, -4e4 - 30.0, -3000.0),
            Arc::new(Diffuse::new(BLACK,Vec3::new(0.2, 0.2, 0.2))),
        )); // mountains

        self.add(Sphere::new(
            26.5,
            Vec3::new(22.0, 26.5, 42.0),
            Arc::new(Mirror::new(BLACK,Vec3::new(1.0, 1.0, 1.0) * 0.596)),
        )); // white Mirr
        self.add(Sphere::new(
            13.0,
            Vec3::new(75.0, 13.0, 82.0),
            Arc::new(Refract::new(BLACK,Vec3::new(0.96, 0.96, 0.96) * 0.96)),
        )); // Glas
        self.add(Sphere::new(
            22.0,
            Vec3::new(87.0, 22.0, 24.0),
            Arc::new(Refract::new(BLACK,Vec3::new(0.6, 0.6, 0.6) * 0.696)),
        )); // Glas2
    }

    pub fn model_init3(&mut self) {
             //------------nightsky sc3----
        self.model_name=String::from("NightSky");         
        self.add(Sphere::new(2.5e3,Vec3::new(0.82,0.92,-2.0)*1e4,
                             Arc::new(Diffuse::new(Vec3::new(1.0,1.0,1.0)*0.8e2,BLACK)), )); // moon
        self.add(Sphere::new(2.5e4,Vec3::new(50.0, 0.0, 0.0),
                             Arc::new(Diffuse::new(Vec3::new(0.114, 0.133, 0.212)*1e-2,
                                                   Vec3::new(0.216,0.384,1.0)*0.003)),)); // sky
        self.add(Sphere::new(5e0,  Vec3::new(-0.2,0.16,-1.0)*1e4,
                             Arc::new(Diffuse::new(Vec3::new(1.00, 0.843, 0.698)*1e2,BLACK)),));  // star
        self.add(Sphere::new(5e0,  Vec3::new(0.0,0.18,-1.0)*1e4,
                             Arc::new(Diffuse::new(Vec3::new(1.00, 0.851, 0.710)*1e2,BLACK)), ));  // star
        self.add(Sphere::new(5e0,  Vec3::new(0.3,0.15,-1.0)*1e4,
                             Arc::new(Diffuse::new(Vec3::new(0.671, 0.780, 1.00)*1e2,BLACK)),));  // star
        self.add(Sphere::new(3.5e4,Vec3::new(600.0,-3.5e4+1.0,300.0),
                             Arc::new(Refract::new(BLACK,Vec3::new(0.6,0.8,1.0)*0.01)),));   //pool
        self.add(Sphere::new(5e4,  Vec3::new(-500.0,-5e4  ,0.0),
                             Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.35)),));    //hill
        self.add(Sphere::new(16.5, Vec3::new(27.0,0.0,47.0),
                             Arc::new(Diffuse::new(BLACK, Vec3::new(1.0,1.0,1.0)*0.33)),)); //hut
        self.add(Sphere::new(7.0,  Vec3::new(27.0+8.0*SQRT_2,0.0,47.0+8.0*SQRT_2),
                             Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.33)),)); //door
        self.add(Sphere::new(500.0,Vec3::new(-1e3,-300.0,-3e3),
                             Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.351)),));  //mnt
        self.add(Sphere::new(830.0,Vec3::new(0.0,-500.0,-3e3),
                             Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.354)),));  //mnt
        self.add(Sphere::new(490.0,Vec3::new(1e3,-300.0,-3e3),
                             Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.352)),));  //mnt 
    }

    pub fn model_init4(&mut self)  {
        //-----------island sc4-------
        self.model_name = String::from("island");
        let cen: Vec3 = Vec3 {
            x: 50.0,
            y: -20.0,
            z: -860.0,
        };
        self.add(Sphere::new(
            160.0,
            cen + Vec3::new(0.0, 600.0, -500.0),
            Arc::new(Diffuse::new(Vec3::new(1.0, 1.0, 1.0) * 2e2, BLACK)),
        )); // sun
        self.add(Sphere::new(
            800.0,
            cen + Vec3::new(0.0, -880.0, -9120.0),
            Arc::new(Diffuse::new(Vec3::new(1.0, 1.0, 1.0) * 2e1, BLACK)),
        )); // horizon
        self.add(Sphere::new(
            10000.0,
            cen + Vec3::new(0.0, 0.0, -200.00),
            Arc::new(Diffuse::new(
                Vec3::new(0.0627, 0.188, 0.569) * 1e0,
                Vec3::new(1.0, 1.0, 1.0) * 0.4,
            )),
        )); // sky
        self.add(Sphere::new(
            800.0,
            cen + Vec3::new(0.0, -720.0, -200.0),
            Arc::new(Refract::new(BLACK,Vec3::new(0.110, 0.898, 1.00) * 0.996)),
        )); // water
        self.add(Sphere::new(
            790.0,
            cen + Vec3::new(0.0, -720.0, -200.0),
            Arc::new(Diffuse::new(BLACK,Vec3::new(0.4, 0.3, 0.04) * 0.6)),
        )); // earth
        self.add(Sphere::new(
            325.0,
            cen + Vec3::new(0.0, -255.0, -50.0),
            Arc::new(Diffuse::new(BLACK,Vec3::new(0.4, 0.3, 0.04) * 0.8)),
        )); // island
        self.add(Sphere::new(
            275.0,
            cen + Vec3::new(0.0, -205.0, -33.0),
            Arc::new(Diffuse::new(BLACK,Vec3::new(0.02, 0.3, 0.02) * 0.75)),
        )); // grass
    }

    pub fn model_init5(&mut self) {
        //-------------Vista sc5------------
        let  cen:Vec3=Vec3{x:50.0,y:-20.0,z:-860.0};
        self.add( Sphere::new(8000.0, cen+Vec3::new(0.0,-8000.0,-900.0),
                              Arc::new(Diffuse::new(Vec3::new(1.0,0.4,0.1)*5e-1,BLACK)), )); // sun
        self.add( Sphere::new(1e4,    cen+Vec3::zero(),
                              Arc::new(Diffuse::new(Vec3::new(0.631, 0.753, 1.00)*3e-1,
                                                    Vec3::new(1.0,1.0,1.0)*0.5)),)); // sky

        self.add( Sphere::new(150.0,  cen+Vec3::new(-350.0, 0.0,-100.0),
                              Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.3)),)); // mnt
        self.add( Sphere::new(200.0,  cen+Vec3::new(-210.0, 0.0,-100.0),
                              Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.3)),)); // mnt
        self.add( Sphere::new(145.0,  cen+Vec3::new(-210.0,85.0,-100.0),
                              Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.8)),)); // snow
        self.add( Sphere::new(150.0,  cen+Vec3::new(-50.0,  0.0,-100.0),
                              Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.3)),)); // mnt
        self.add( Sphere::new(150.0,  cen+Vec3::new(100.0,  0.0,-100.0),
                              Arc::new(Diffuse::new(BLACK, Vec3::new(1.0,1.0,1.0)*0.3)),)); // mnt
        self.add( Sphere::new(125.0,  cen+Vec3::new(250.0,  0.0,-100.0),
                              Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.3)),)); // mnt
        self.add( Sphere::new(150.0,  cen+Vec3::new(375.0,  0.0,-100.0),
                              Arc::new(Diffuse::new(BLACK, Vec3::new(1.0,1.0,1.0)*0.3)),)); // mnt
        self.add( Sphere::new(2500.0, cen+Vec3::new(0.0,-2400.0,-500.0),
                              Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.1)),)); // mnt base

        self.add( Sphere::new(8000.0, cen+Vec3::new(0.0,-8000.0, 200.0),
                              Arc::new(Refract::new(BLACK,Vec3::new(0.2,0.2,1.0))),)); // water
        self.add( Sphere::new(8000.0, cen+Vec3::new(0.0,-8000.0,1100.0),
                              Arc::new(Diffuse::new(BLACK,Vec3::new(0.0,0.3,0.0) )),)); // grass
        self.add( Sphere::new(8.0   , cen+Vec3::new(-75.0, -5.0, 850.0),
                              Arc::new(Diffuse::new(BLACK,Vec3::new(0.0,0.3,0.0) )),)); // bush
        self.add( Sphere::new(30.0,   cen+Vec3::new(0.0,   23.0, 825.0),
                              Arc::new(Refract::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.996)),)); // ball

        self.add( Sphere::new(30.0,   cen+Vec3::new(200.0,280.0,-400.0),
                              Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.8)),));   // clouds
        self.add( Sphere::new(37.0,   cen+Vec3::new(237.0,280.0,-400.0),
                              Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.8)),));   // clouds
        self.add( Sphere::new(28.0,   cen+Vec3::new(267.0,280.0,-400.0),
                              Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.8)),));   // clouds

        self.add( Sphere::new(40.0,   cen+Vec3::new(150.0,280.0,-1000.0),
                              Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.8)),));  // clouds
        self.add( Sphere::new(37.0,   cen+Vec3::new(187.0,280.0,-1000.0),
                              Arc::new(Diffuse::new(BLACK, Vec3::new(1.0,1.0,1.0)*0.8)),));  // clouds

        self.add( Sphere::new(40.0,   cen+Vec3::new(600.0,280.0,-1100.0),
                              Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.8)), ));  // clouds
        self.add( Sphere::new(37.0,   cen+Vec3::new(637.0,280.0,-1100.0),
                              Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.8)),));  // clouds
        self.add( Sphere::new(37.0, cen+Vec3::new(-800.0,280.0,-1400.0),
                              Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.8)),)); // clouds
        self.add( Sphere::new(37.0, cen+Vec3::new(0.0,   280.0,-1600.0),
                              Arc::new(Diffuse::new(BLACK, Vec3::new(1.0,1.0,1.0)*0.8)),));    // clouds
        self.add( Sphere::new(37.0, cen+Vec3::new(537.0, 280.0,-1800.0),
                              Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.8)),  ));  // clouds
    }

    pub fn model_init6(&mut self) {
        //----------------Overlap  sc6-----------------
        self.model_name=String::from("Overlap");
        self.add( Sphere::new(150.0, Vec3::new(50.0+75.0,28.0,62.0),
                              Arc::new(Refract::new(Vec3::new(1.0,1.0,1.0)*0e-3,
                                        Vec3::new(1.0,0.9,0.8)*0.93)), ));
        self.add( Sphere::new(28.0,  Vec3::new(50.0+5.0,-28.0,62.0), 
                              Arc::new(Diffuse::new(Vec3::new(1.0,1.0,1.0)*1e1, BLACK)),));
        self.add( Sphere::new(300.0, Vec3::new(50.0,     28.0,62.0), 
                              Arc::new(Mirror::new(Vec3::new(1.0,1.0,1.0)*0e-3,
                                        Vec3::new(1.0,1.0,1.0)*0.93)), ));
    }


    pub fn model_init7(&mut self)  {
        //----------------wada  sc7-------------
        self.model_name = String::from("Wada");

        let r: f64 = 60.0;
        let t: f64 = FRAC_PI_6; //30.0*PI/180.0;
        let d: f64 = r / (FRAC_SQRT_3 / 2.0); //cos(T);
        //let Z:f64=60.0;

        self.add(Sphere::new(
            1e5,
            Vec3::new(50.0, 100.0, 0.0),
            Arc::new(Diffuse::new(Vec3::new(1.0, 1.0, 1.0) * 3e0, BLACK)),
        )); // sky
        self.add(Sphere::new(
            1e5,
            Vec3::new(50.0, -1e5 - d - r, 0.0),
            Arc::new(Diffuse::new(BLACK,Vec3::new(0.1, 0.1, 0.1))),
        )); //grnd
        self.add(Sphere::new(
            r,
            Vec3::new(50.0, 40.8, 62.0) + Vec3::new(t.cos(), t.sin(), 0.0) * d,
            Arc::new(Mirror::new(BLACK,Vec3::new(1.0, 0.3, 0.3) * 0.999)),
        )); //red
        self.add(Sphere::new(
            r,
            Vec3::new(50.0, 40.8, 62.0) + Vec3::new(-t.cos(), t.sin(), 0.0) * d,
            Arc::new(Mirror::new(BLACK,Vec3::new(0.3, 1.0, 0.3) * 0.999)),
        )); //grn
        self.add(Sphere::new(
            r,
            Vec3::new(50.0, 40.8, 62.0) + Vec3::new(0.0, -1.0, 0.0) * d,
            Arc::new(Mirror::new(BLACK,Vec3::new(0.3, 0.3, 1.0) * 0.999)),
        )); //blue
        self.add(Sphere::new(
            r,
            Vec3::new(50.0, 40.8, 62.0) + Vec3::new(0.0, 0.0, -1.0) * d,
            Arc::new(Mirror::new(BLACK,Vec3::new(0.53, 0.53, 0.53) * 0.999)),
        )); //back
        self.add(Sphere::new(
            r,
            Vec3::new(50.0, 40.8, 62.0) + Vec3::new(0.0, 0.0, 1.0) * d,
            Arc::new(Refract::new(BLACK,Vec3::new(1.0, 1.0, 1.0) * 0.999)),
        )); //front
    }

    pub fn model_init8(&mut self) {
        //-----------------wada2 sc8----------
        self.model_name=String::from("Wada2");
        let r:f64=60.0;
        let t:f64=FRAC_PI_6;//30.0*PI/180.0;
        let d:f64=r/(FRAC_SQRT_3/2.0) ;//cos(T);
        let z:f64=62.0;
        let c:Vec3=Vec3{x:0.275,y:0.612,z:0.949};

        self.add( Sphere::new(
            r, Vec3::new(50.0,28.0,z)+Vec3::new( t.cos(),t.sin(),0.0)*d,
            Arc::new(Mirror::new(c*6e-2, Vec3::new(1.0,1.0,1.0)*0.996)),)); //red
        self.add( Sphere::new(
            r, Vec3::new(50.0,28.0,z)+Vec3::new(-t.cos(),t.sin(),0.0)*d,
            Arc::new(Mirror::new(c*6e-2, Vec3::new(1.0,1.0,1.0)*0.996)),)); //grn
        self.add( Sphere::new(
            r, Vec3::new(50.0,28.0,z)+Vec3::new(0.0,-1.0,0.0)*d,
            Arc::new(Mirror::new(c*6e-2, Vec3::new(1.0,1.0,1.0)*0.996)),)); //blue
        self.add( Sphere::new(
            r,Vec3::new(50.0,28.0,z)+Vec3::new(0.0, 0.0,-1.0)*r*2.0*f64::sqrt(2.0/3.0),
            Arc::new(Mirror::new(c*0e-2, Vec3::new(1.0,1.0,1.0)*0.996)),)); //back
        self.add( Sphere::new(
            2.0*2.0*r*2.0*f64::sqrt(2.0/3.0)-r*2.0*f64::sqrt(2.0/3.0)/3.0,
            Vec3::new(50.0,28.0,z)+Vec3::new(0.0,0.0,-r*2.0*f64::sqrt(2.0/3.0)/3.0),
            Arc::new(Refract::new(
                Vec3::new(1.0,1.0,1.0)*0.0,
                Vec3::new(1.0,1.0,1.0)*0.5)),)); //front
    }

    pub fn model_init9(&mut self) {
        //---------------forest sc9-----------
        self.model_name=String::from("Forest");
        let tc:Vec3=Vec3{x:0.0588, y:0.361, z:0.0941};
        let scc:Vec3=Vec3{x:0.7,y:0.7,z:0.7};

        self.add( Sphere::new(
            1e5, Vec3::new(50.0, 1e5+130.0, 0.0),
            Arc::new(Diffuse::new(Vec3::new(1.0,1.0,1.0)*1.3,BLACK)),)); //lite
        self.add( Sphere::new(
            1e2, Vec3::new(50.0, -1e2+2.0, 47.0), 
            Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.7)),)); //grnd

        self.add(
            Sphere::new(
                1e4,
                Vec3::new(50.0, -30.0, 300.0)+Vec3::new(-f64::sin(50.0*PI/180.0),0.0,f64::cos(50.0*PI/180.0))*1e4,
                Arc::new(Mirror::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.99)),));// mirr L
        self.add(
            Sphere::new(
                1e4,
                Vec3::new(50.0, -30.0, 300.0)+Vec3::new(f64::sin(50.0*PI/180.0),0.0,f64::cos(50.0*PI/180.0))*1e4,
                Arc::new(Mirror::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.99)),));// mirr R
        self.add(
            Sphere::new(
                1e4,
                Vec3::new(50.0, -30.0, -50.0)+Vec3::new(-f64::sin(30.0*PI/180.0),0.0,-f64::cos(30.0*PI/180.0))*1e4,
                Arc::new(Mirror::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.99)),));// mirr FL
        self.add(
            Sphere::new(
                1e4,
                Vec3::new(50.0, -30.0, -50.0)+Vec3::new(f64::sin(30.0*PI/180.0),0.0,-f64::cos(30.0*PI/180.0))*1e4,
                Arc::new(Mirror::new(BLACK,Vec3::new(1.0,1.0,1.0)*0.99)),));// mirr

        self.add( Sphere::new(4.0, Vec3::new(50.0,6.0*0.6,47.0),
                              Arc::new(Diffuse::new(BLACK, Vec3::new(0.13,0.066,0.033) )),));//"tree"
        self.add( Sphere::new(16.0,Vec3::new(50.0,6.0*2.0+16.0*0.6,47.0),
                              Arc::new(Diffuse::new(BLACK, tc)),  ));//"tree"
        self.add( Sphere::new(11.0,Vec3::new(50.0,6.0*2.0+16.0*0.6*2.0+11.0*0.6,47.0),
                              Arc::new(Diffuse::new(BLACK, tc)),  ));//"tree"
        self.add( Sphere::new(7.0 ,Vec3::new(50.0,6.0*2.0+16.0*0.6*2.0+11.0*0.6*2.0+7.0*0.6,47.0),
                              Arc::new(Diffuse::new(BLACK, tc)), ));//"tree"

        self.add( Sphere::new(15.5,Vec3::new(50.0,1.8+6.0*2.0+16.0*0.6,47.0),
                              Arc::new(Diffuse::new(BLACK, scc)),));//"tree"
        self.add( Sphere::new(10.5,Vec3::new(50.0,1.8+6.0*2.0+16.0*0.6*2.0+11.0*0.6,47.0),
                              Arc::new(Diffuse::new(  BLACK, scc)), ));//"tree"
        self.add( Sphere::new(6.5, Vec3::new(50.0,1.8+6.0*2.0+16.0*0.6*2.0+11.0*0.6*2.0+7.0*0.6,47.0),
                              Arc::new(Diffuse::new(  BLACK, scc)),  ));//"tree"
    }
    
    pub fn random_scene(& mut self,w:usize,h:usize)->Camera{
        let cen  = Vec3::new(50.0,40.8,-860.0);
        let cen1 = Vec3::new(75.0,25.0, 85.0);
        let cen2 = Vec3::new(45.0,25.0, 30.0);
        let cen3 = Vec3::new(15.0,25.0,-25.0);
        self.add(Sphere::new(
            10000.0,
            cen+Vec3::new(0.0,0.0,-200.0)  ,
            Arc::new(Diffuse::new(Vec3::new(0.6, 0.5, 0.7)*0.8, Vec3::new(0.7,0.9,1.0))),  )); // sky
        self.add(Sphere::new(100000.0, Vec3::new(50.0, -100000.0, 0.0),
                             Arc::new(Diffuse::new(BLACK, Vec3::new(0.4,0.4,0.4))),)); // grnd
        self.add(Sphere::new(25.0,  cen1 ,
                             Arc::new(Mirror::new(BLACK,Vec3::new(0.9,0.9,0.9))), ));// mirror
        self.add(Sphere::new(25.0,  cen2 ,
                             Arc::new(Refract::new(BLACK,Vec3::new(0.95,0.95,0.95))), )); // glass
        self.add(Sphere::new(25.0,  cen3 ,
                             Arc::new(Diffuse::new(BLACK,Vec3::new(1.0,0.6,0.6)*0.696)), ));    // 乱反射
        for a in -11 .. 12 {
            for b in -11 .. 12 {
                let random_material = random();
                let cen = Vec3::new( ((a as f64)+random())*25.0,5.0,((b as f64)+random() )*25.0);
                if (cen - cen1) .length().sqrt()>(25.0*1.0) {
                    if random_material < 0.8 {
                        self.add(Sphere::new(
                            5.0, cen,
                            Arc::new(Diffuse::new(BLACK,Vec3::new(random(),random(),random()))),));
                    } else if random_material <0.95 {
                        self.add(Sphere::new(
                            5.0,cen,
                            Arc::new(Mirror::new(BLACK,Vec3::new(random(),random(),random()))),));
                    } else {
                        self.add(Sphere::new(
                            5.0,cen,
                            Arc::new(Refract::new(BLACK,Vec3::new(random(),random(),random()) )),));
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
/*
    pub fn model_init1(&mut self)->bool{
        //----------cornel box sc1-----------
        self.add( Sphere::new( 1e5,   Vec3::new(1e5 + 1.0,      40.8, 81.6),BLACK,                 Vec3::new(0.75, 0.25, 0.25), Refl::Diff ));
        self.add( Sphere::new( 1e5,   Vec3::new(-1e5 + 99.0,    40.8, 81.6),BLACK,                 Vec3::new(0.25, 0.25, 0.75), Refl::Diff ));
        self.add( Sphere::new( 1e5,   Vec3::new(50.0,           40.8, 1e5 ),BLACK,                 Vec3::new(0.75, 0.75, 0.75), Refl::Diff ));
        self.add( Sphere::new( 1e5,   Vec3::new(50.0,           40.8,-1e5 + 170.0),BLACK,         BLACK, Refl::Diff ));
        self.add( Sphere::new( 1e5,   Vec3::new(50.0,            1e5, 81.6),BLACK,                 Vec3::new(0.75, 0.75, 0.75), Refl::Diff ));
        self.add( Sphere::new( 1e5,   Vec3::new(50.0,-1e5 + 81.6+4.0, 81.6),BLACK,                 Vec3::new(0.75, 0.75, 0.75), Refl::Diff ));
        self.add( Sphere::new( 16.5,  Vec3::new(27.0,           16.5, 47.0),BLACK,                 Vec3::new(1.0, 1.0, 1.0) * 0.999, Refl::Spec ));
        self.add( Sphere::new( 16.5,  Vec3::new(73.0,           16.5, 78.0),BLACK,                 Vec3::new(1.0, 1.0, 1.0) * 0.999, Refl::Refr ));
        self.add( Sphere::new( 1.5,   Vec3::new(50.0,      81.6-16.5, 81.6), Vec3::new(4.0,4.0,4.0)*100.0,BLACK, Refl::Diff ));
    true
    }







*/
