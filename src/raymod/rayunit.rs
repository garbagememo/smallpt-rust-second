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
	
	pub fn model_init0(&mut self)->bool{
 //-------------Debug Scene sc1-------------
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
	

	pub fn model_init1(&mut self)->bool{
   //----------cornel box sc1-----------
        self.objects.push( Sphere::new( 1e5,   Vec3::new(1e5 + 1.0,      40.8, 81.6), Vec3::zero(),                 Vec3::new(0.75, 0.25, 0.25), Refl::Diff ));
        self.objects.push( Sphere::new( 1e5,   Vec3::new(-1e5 + 99.0,    40.8, 81.6), Vec3::zero(),                 Vec3::new(0.25, 0.25, 0.75), Refl::Diff ));
        self.objects.push( Sphere::new( 1e5,   Vec3::new(50.0,           40.8, 1e5 ), Vec3::zero(),                 Vec3::new(0.75, 0.75, 0.75), Refl::Diff ));
        self.objects.push( Sphere::new( 1e5,   Vec3::new(50.0,           40.8,-1e5 + 170.0), Vec3::zero(),          Vec3::zero(), Refl::Diff ));
        self.objects.push( Sphere::new( 1e5,   Vec3::new(50.0,            1e5, 81.6), Vec3::zero(),                 Vec3::new(0.75, 0.75, 0.75), Refl::Diff ));
        self.objects.push( Sphere::new( 1e5,   Vec3::new(50.0,-1e5 + 81.6+4.0, 81.6), Vec3::zero(),                 Vec3::new(0.75, 0.75, 0.75), Refl::Diff ));
        self.objects.push( Sphere::new( 16.5,  Vec3::new(27.0,           16.5, 47.0), Vec3::zero(),                 Vec3::new(1.0, 1.0, 1.0) * 0.999, Refl::Spec ));
        self.objects.push( Sphere::new( 16.5,  Vec3::new(73.0,           16.5, 78.0), Vec3::zero(),                 Vec3::new(1.0, 1.0, 1.0) * 0.999, Refl::Refr ));
        self.objects.push( Sphere::new( 1.5,   Vec3::new(50.0,      81.6-16.5, 81.6), Vec3::new(4.0,4.0,4.0)*100.0, Vec3::zero(), Refl::Diff ));
		true
    }
	

	pub fn model_init2(&mut self)->bool{
  //-----------sky sc2--------------
        let Cen:Vec3=Vec3{x:50.0,y:40.8,z:-860.0};
		self.objects.push( Sphere::new(1600.0, Vec3::new(1.0,0.0,2.0)*3000.0, Vec3::new(1.0,0.9,0.8)*1.2e1*1.56*2.0,  Vec3::zero(), Refl::Diff)); // sun
		self.objects.push( Sphere::new(1560.0, Vec3::new(1.0,0.0,2.0)*3500.0, Vec3::new(1.0,0.5,0.05)*4.8e1*1.56*2.0, Vec3::zero(), Refl::Diff)); // horizon sun2
        self.objects.push( Sphere::new(10000.0,
				 Cen+Vec3::new(0.0,0.0,-200.0),
				 Vec3::new(0.00063842, 0.02001478, 0.28923243)*6e-2*8.0,
				 Vec3::new(0.7,0.7,1.0)*0.25,  Refl::Diff)); // sky

		self.objects.push( Sphere::new(100000.0,Vec3::new(50.0,-100000.0,0.0),Vec3::zero(),				Vec3::new(0.3,0.3,0.3),Refl::Diff)); // grnd
		self.objects.push( Sphere::new(110000.0,Vec3::new(50.0,-110048.5,0.0),Vec3::new(0.9,0.5,0.05)*4.0,Vec3::zero(),Refl::Diff));// horizon brightener
		self.objects.push( Sphere::new(4e4, 	  Vec3::new(50.0,-4e4-30.0,-3000.0),Vec3::zero(),			Vec3::new(0.2,0.2,0.2),Refl::Diff));// mountains

        self.objects.push( Sphere::new(26.5,	Vec3::new(22.0,26.5,42.0),Vec3::zero(),						Vec3::new(1.0,1.0,1.0)*0.596, Refl::Spec)); // white Mirr
        self.objects.push( Sphere::new(13.0,	Vec3::new(75.0,13.0,82.0),Vec3::zero(),						Vec3::new(0.96,0.96,0.96)*0.96, Refl::Refr));// Glas
        self.objects.push( Sphere::new(22.0,	Vec3::new(87.0,22.0,24.0),Vec3::zero(),						Vec3::new(0.6,0.6,0.6)*0.696, Refl::Refr));    // Glas2
        true
	}

	

	 pub fn model_init3(&mut self)->bool{
  //------------nightsky sc3----
         self.objects.push(Sphere::new(2.5e3,Vec3::new(0.82,0.92,-2.0)*1e4,   Vec3::new(1.0,1.0,1.0)*0.8e2,        Vec3::zero(), Refl::Diff)); // moon
         self.objects.push(Sphere::new(2.5e4,Vec3::new(50.0, 0.0, 0.0),       Vec3::new(0.114, 0.133, 0.212)*1e-2, Vec3::new(0.216,0.384,1.0)*0.003, Refl::Diff)); // sky
         self.objects.push(Sphere::new(5e0,  Vec3::new(-0.2,0.16,-1.0)*1e4,   Vec3::new(1.00, 0.843, 0.698)*1e2,   Vec3::zero(), Refl::Diff));  // star
         self.objects.push(Sphere::new(5e0,  Vec3::new(0.0,0.18,-1.0)*1e4,    Vec3::new(1.00, 0.851, 0.710)*1e2,   Vec3::zero(), Refl::Diff));  // star
         self.objects.push(Sphere::new(5e0,  Vec3::new(0.3,0.15,-1.0)*1e4,    Vec3::new(0.671, 0.780, 1.00)*1e2,   Vec3::zero(), Refl::Diff));  // star
         self.objects.push(Sphere::new(3.5e4,Vec3::new(600.0,-3.5e4+1.0,300.0), Vec3::zero(),                      Vec3::new(0.6,0.8,1.0)*0.01,  Refl::Refr));   //pool
         self.objects.push(Sphere::new(5e4,  Vec3::new(-500.0,-5e4  ,0.0),    Vec3::zero(),                        Vec3::new(1.0,1.0,1.0)*0.35,  Refl::Diff));    //hill
         self.objects.push(Sphere::new(16.5, Vec3::new(27.0,0.0,47.0),        Vec3::zero(),                        Vec3::new(1.0,1.0,1.0)*0.33, Refl::Diff)); //hut
         self.objects.push(Sphere::new(7.0,  Vec3::new(27.0+8.0*SQRT_2,0.0,47.0+8.0*SQRT_2),Vec3::zero(),          Vec3::new(1.0,1.0,1.0)*0.33,  Refl::Diff)); //door
         self.objects.push(Sphere::new(500.0,Vec3::new(-1e3,-300.0,-3e3),     Vec3::zero(),                        Vec3::new(1.0,1.0,1.0)*0.351,    Refl::Diff));  //mnt
         self.objects.push(Sphere::new(830.0,Vec3::new(0.0,-500.0,-3e3),     Vec3::zero(),                         Vec3::new(1.0,1.0,1.0)*0.354,    Refl::Diff));  //mnt
         self.objects.push(Sphere::new(490.0,Vec3::new(1e3,-300.0,-3e3),      Vec3::zero(),                        Vec3::new(1.0,1.0,1.0)*0.352,    Refl::Diff));  //mnt 
         true
	 }


     pub fn model_init4(&mut self)->bool{
//-----------island sc4-------
         let  Cen:Vec3=Vec3{x:50.0,y:-20.0,z:-860.0};
         self.objects.push( Sphere::new(160.0, Cen+Vec3::new(0.0, 600.0, -500.0), Vec3::new(1.0,1.0,1.0)*2e2,          Vec3::zero(),  Refl::Diff)); // sun
         self.objects.push( Sphere::new(800.0, Cen+Vec3::new(0.0,-880.0,-9120.0), Vec3::new(1.0,1.0,1.0)*2e1,          Vec3::zero(),  Refl::Diff)); // horizon
         self.objects.push( Sphere::new(10000.0,Cen+Vec3::new(0.0,0.0,-200.00),   Vec3::new(0.0627, 0.188, 0.569)*1e0, Vec3::new(1.0,1.0,1.0)*0.4,  Refl::Diff)); // sky
         self.objects.push( Sphere::new(800.0, Cen+Vec3::new(0.0,-720.0,-200.0),  Vec3::zero(),              Vec3::new(0.110, 0.898, 1.00)*0.996,  Refl::Refr)); // water
         self.objects.push( Sphere::new(790.0, Cen+Vec3::new(0.0,-720.0,-200.0),  Vec3::zero(),               Vec3::new(0.4,0.3,0.04)*0.6, Refl::Diff)); // earth
         self.objects.push( Sphere::new(325.0, Cen+Vec3::new(0.0,-255.0,-50.0),   Vec3::zero(),               Vec3::new(0.4,0.3,0.04)*0.8, Refl::Diff)); // island
          self.objects.push( Sphere::new(275.0, Cen+Vec3::new(0.0,-205.0,-33.0),   Vec3::zero(),               Vec3::new(0.02,0.3,0.02)*0.75,Refl::Diff)); // grass
        true
	 }


  pub fn model_init5(&mut self)->bool{
  //-------------Vista sc5------------
         let  Cen:Vec3=Vec3{x:50.0,y:-20.0,z:-860.0};
        self.objects.push( Sphere::new(8000.0, Cen+Vec3::new(0.0,-8000.0,-900.0),Vec3::new(1.0,0.4,0.1)*5e-1,        Vec3::zero(),  Refl::Diff)); // sun
        self.objects.push( Sphere::new(1e4,    Cen+Vec3::zero(),                 Vec3::new(0.631, 0.753, 1.00)*3e-1, Vec3::new(1.0,1.0,1.0)*0.5,  Refl::Diff)); // sky

        self.objects.push( Sphere::new(150.0,  Cen+Vec3::new(-350.0, 0.0,-100.0), Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.3,  Refl::Diff)); // mnt
        self.objects.push( Sphere::new(200.0,  Cen+Vec3::new(-210.0, 0.0,-100.0), Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.3,  Refl::Diff)); // mnt
        self.objects.push( Sphere::new(145.0,  Cen+Vec3::new(-210.0,85.0,-100.0), Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.8,  Refl::Diff)); // snow
        self.objects.push( Sphere::new(150.0,  Cen+Vec3::new(-50.0,  0.0,-100.0), Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.3,  Refl::Diff)); // mnt
        self.objects.push( Sphere::new(150.0,  Cen+Vec3::new(100.0,  0.0,-100.0), Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.3,  Refl::Diff)); // mnt
        self.objects.push( Sphere::new(125.0,  Cen+Vec3::new(250.0,  0.0,-100.0), Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.3,  Refl::Diff)); // mnt
        self.objects.push( Sphere::new(150.0,  Cen+Vec3::new(375.0,  0.0,-100.0), Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.3,  Refl::Diff)); // mnt

        self.objects.push( Sphere::new(2500.0, Cen+Vec3::new(0.0,-2400.0,-500.0), Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.1,  Refl::Diff)); // mnt base

        self.objects.push( Sphere::new(8000.0, Cen+Vec3::new(0.0,-8000.0, 200.0), Vec3::zero(),           Vec3::new(0.2,0.2,1.0),    Refl::Refr)); // water
        self.objects.push( Sphere::new(8000.0, Cen+Vec3::new(0.0,-8000.0,1100.0), Vec3::zero(),           Vec3::new(0.0,0.3,0.0),   Refl::Diff)); // grass
        self.objects.push( Sphere::new(8.0   , Cen+Vec3::new(-75.0, -5.0, 850.0), Vec3::zero(),           Vec3::new(0.0,0.3,0.0),     Refl::Diff)); // bush
        self.objects.push( Sphere::new(30.0,   Cen+Vec3::new(0.0,   23.0, 825.0), Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.996, Refl::Refr)); // ball

        self.objects.push( Sphere::new(30.0,   Cen+Vec3::new(200.0,280.0,-400.0), Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.8,  Refl::Diff));   // clouds
        self.objects.push( Sphere::new(37.0,   Cen+Vec3::new(237.0,280.0,-400.0), Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.8,  Refl::Diff));   // clouds
        self.objects.push( Sphere::new(28.0,   Cen+Vec3::new(267.0,280.0,-400.0), Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.8,  Refl::Diff));   // clouds

        self.objects.push( Sphere::new(40.0,   Cen+Vec3::new(150.0,280.0,-1000.0),Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.8,  Refl::Diff));  // clouds
        self.objects.push( Sphere::new(37.0,   Cen+Vec3::new(187.0,280.0,-1000.0),Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.8,  Refl::Diff));  // clouds

        self.objects.push( Sphere::new(40.0,   Cen+Vec3::new(600.0,280.0,-1100.0),Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.8,  Refl::Diff));  // clouds
        self.objects.push( Sphere::new(37.0,   Cen+Vec3::new(637.0,280.0,-1100.0),Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.8,  Refl::Diff));  // clouds

        self.objects.push( Sphere::new(37.0, Cen+Vec3::new(-800.0,280.0,-1400.0), Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.8,  Refl::Diff)); // clouds
        self.objects.push( Sphere::new(37.0, Cen+Vec3::new(0.0,   280.0,-1600.0), Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.8,  Refl::Diff));    // clouds
        self.objects.push( Sphere::new(37.0, Cen+Vec3::new(537.0, 280.0,-1800.0), Vec3::zero(),           Vec3::new(1.0,1.0,1.0)*0.8,  Refl::Diff));  // clouds
        true
	}
    pub fn model_init6(&mut self)->bool{
//----------------Overlap  sc6-----------------
        let D:f64=50.0;
        let R:f64=40.0;

        self.objects.push( Sphere::new(150.0, Vec3::new(50.0+75.0,28.0,62.0), Vec3::new(1.0,1.0,1.0)*0e-3, Vec3::new(1.0,0.9,0.8)*0.93, Refl::Refr));
        self.objects.push( Sphere::new(28.0,  Vec3::new(50.0+5.0,-28.0,62.0), Vec3::new(1.0,1.0,1.0)*1e1,  Vec3::zero(), Refl::Diff));
        self.objects.push( Sphere::new(300.0, Vec3::new(50.0,     28.0,62.0), Vec3::new(1.0,1.0,1.0)*0e-3, Vec3::new(1.0,1.0,1.0)*0.93, Refl::Spec));
        true
	}


    pub fn model_init7(&mut self)->bool{
  //----------------wada  sc7-------------
	
        let R:f64=60.0;
        let T:f64=FRAC_PI_6;//30.0*PI/180.0;
        let D:f64=R/(FRAC_SQRT_3/2.0) ;//cos(T);
        let Z:f64=60.0;

        self.objects.push( Sphere::new(1e5, Vec3::new(50.0, 100.0, 0.0),      Vec3::new(1.0,1.0,1.0)*3e0, Vec3::zero(), Refl::Diff)); // sky
        self.objects.push( Sphere::new(1e5, Vec3::new(50.0, -1e5-D-R, 0.0),   Vec3::zero(),               Vec3::new(0.1,0.1,0.1),Refl::Diff));           //grnd

        self.objects.push( Sphere::new(R, Vec3::new(50.0,40.8,62.0)+Vec3::new( T.cos(),T.sin(),0.0)*D, Vec3::zero(), Vec3::new(1.0,0.3,0.3)*0.999, Refl::Spec)); //red
        self.objects.push( Sphere::new(R, Vec3::new(50.0,40.8,62.0)+Vec3::new(-T.cos(),T.sin(),0.0)*D, Vec3::zero(), Vec3::new(0.3,1.0,0.3)*0.999, Refl::Spec)); //grn
        self.objects.push( Sphere::new(R, Vec3::new(50.0,40.8,62.0)+Vec3::new(0.0,-1.0,0.0)*D,         Vec3::zero(), Vec3::new(0.3,0.3,1.0)*0.999, Refl::Spec)); //blue
        self.objects.push( Sphere::new(R, Vec3::new(50.0,40.8,62.0)+Vec3::new(0.0, 0.0,-1.0)*D,     Vec3::zero(), Vec3::new(0.53,0.53,0.53)*0.999, Refl::Spec)); //back
        self.objects.push( Sphere::new(R, Vec3::new(50.0,40.8,62.0)+Vec3::new(0.0, 0.0, 1.0)*D,        Vec3::zero(), Vec3::new(1.0,1.0,1.0)*0.999, Refl::Refr)); //front
        true
	}



    pub fn model_init8(&mut self)->bool{
  //-----------------wada2 sc8----------

        let R:f64=60.0;
        let T:f64=FRAC_PI_6;//30.0*PI/180.0;
        let D:f64=R/(FRAC_SQRT_3/2.0) ;//cos(T);
        let Z:f64=62.0;
        let C:Vec3=Vec3{x:0.275,y:0.612,z:0.949};

        self.objects.push( Sphere::new(R, Vec3::new(50.0,28.0,Z)+Vec3::new( T.cos(),T.sin(),0.0)*D,           C*6e-2, Vec3::new(1.0,1.0,1.0)*0.996, Refl::Spec)); //red
        self.objects.push( Sphere::new(R, Vec3::new(50.0,28.0,Z)+Vec3::new(-T.cos(),T.sin(),0.0)*D,           C*6e-2, Vec3::new(1.0,1.0,1.0)*0.996, Refl::Spec)); //grn
        self.objects.push( Sphere::new(R, Vec3::new(50.0,28.0,Z)+Vec3::new(0.0,-1.0,0.0)*D,                   C*6e-2, Vec3::new(1.0,1.0,1.0)*0.996, Refl::Spec)); //blue
        self.objects.push( Sphere::new(R,
                                       Vec3::new(50.0,28.0,Z)+Vec3::new(0.0, 0.0,-1.0)*R*2.0*f64::sqrt(2.0/3.0),
                                       C*0e-2, Vec3::new(1.0,1.0,1.0)*0.996,
                                       Refl::Spec)); //back
        self.objects.push( Sphere::new(2.0*2.0*R*2.0*f64::sqrt(2.0/3.0)-R*2.0*f64::sqrt(2.0/3.0)/3.0,
			                           Vec3::new(50.0,28.0,Z)+Vec3::new(0.0,0.0,-R*2.0*f64::sqrt(2.0/3.0)/3.0),
                                       Vec3::new(1.0,1.0,1.0)*0.0,Vec3::new(1.0,1.0,1.0)*0.5,
                                       Refl::Spec)); //front

	    true
	}



    pub fn model_init9(&mut self)->bool{
      //---------------forest sc9-----------
        let tc:Vec3=Vec3{x:0.0588, y:0.361, z:0.0941};
        let scc:Vec3=Vec3{x:0.7,y:0.7,z:0.7};

        self.objects.push( Sphere::new(1e5, Vec3::new(50.0, 1e5+130.0, 0.0),  Vec3::new(1.0,1.0,1.0)*1.3,Vec3::zero(),Refl::Diff)); //lite
        self.objects.push( Sphere::new(1e2, Vec3::new(50.0, -1e2+2.0, 47.0),  Vec3::zero(),              Vec3::new(1.0,1.0,1.0)*0.7,Refl::Diff)); //grnd

		self.objects.push( Sphere::new(1e4,
				 Vec3::new(50.0, -30.0, 300.0)+Vec3::new(-f64::sin(50.0*PI/180.0),0.0,f64::cos(50.0*PI/180.0))*1e4,
				 Vec3::zero(),
				 Vec3::new(1.0,1.0,1.0)*0.99,Refl::Spec));// mirr L
		self.objects.push( Sphere::new(1e4,
				 Vec3::new(50.0, -30.0, 300.0)+Vec3::new(f64::sin(50.0*PI/180.0),0.0,f64::cos(50.0*PI/180.0))*1e4,
				 Vec3::zero(),
				 Vec3::new(1.0,1.0,1.0)*0.99,Refl::Spec));// mirr R
		self.objects.push( Sphere::new(1e4,
				 Vec3::new(50.0, -30.0, -50.0)+Vec3::new(-f64::sin(30.0*PI/180.0),0.0,-f64::cos(30.0*PI/180.0))*1e4,
				 Vec3::zero(),
				 Vec3::new(1.0,1.0,1.0)*0.99,Refl::Spec));// mirr FL
		self.objects.push( Sphere::new(1e4,
				 Vec3::new(50.0, -30.0, -50.0)+Vec3::new(f64::sin(30.0*PI/180.0),0.0,-f64::cos(30.0*PI/180.0))*1e4,
				 Vec3::zero(),
				 Vec3::new(1.0,1.0,1.0)*0.99,Refl::Spec));// mirr


        self.objects.push( Sphere::new(4.0, Vec3::new(50.0,6.0*0.6,47.0),                              Vec3::zero(), Vec3::new(0.13,0.066,0.033), Refl::Diff));//"tree"
        self.objects.push( Sphere::new(16.0,Vec3::new(50.0,6.0*2.0+16.0*0.6,47.0),                          Vec3::zero(), tc,  Refl::Diff));//"tree"
        self.objects.push( Sphere::new(11.0,Vec3::new(50.0,6.0*2.0+16.0*0.6*2.0+11.0*0.6,47.0),             Vec3::zero(), tc,  Refl::Diff));//"tree"
        self.objects.push( Sphere::new(7.0 ,Vec3::new(50.0,6.0*2.0+16.0*0.6*2.0+11.0*0.6*2.0+7.0*0.6,47.0), Vec3::zero(), tc,  Refl::Diff));//"tree"

        self.objects.push( Sphere::new(15.5,Vec3::new(50.0,1.8+6.0*2.0+16.0*0.6,47.0),   Vec3::zero(), scc,  Refl::Diff));//"tree"
        self.objects.push( Sphere::new(10.5,Vec3::new(50.0,1.8+6.0*2.0+16.0*0.6*2.0+11.0*0.6,47.0),   Vec3::zero(), scc,  Refl::Diff));//"tree"
        self.objects.push( Sphere::new(6.5, Vec3::new(50.0,1.8+6.0*2.0+16.0*0.6*2.0+11.0*0.6*2.0+7.0*0.6,47.0),   Vec3::zero(), scc,  Refl::Diff));//"tree"
	    true
	}

}
	
