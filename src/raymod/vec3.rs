use std::fs;
use std::io::Write;
use std::path::{Path};
use std::ops::{Add, Mul, Rem, Sub};

pub fn random() -> f64 {
    rand::random::<f64>()
}

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Color = Vec3;
pub const BLACK:Vec3 = Vec3{x:0.0,y:0.0,z:0.0};

#[allow(dead_code)]
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
    pub fn mult(&self, b: &Vec3) -> Vec3 {
        Vec3::new(self.x * b.x, self.y * b.y, self.z * b.z)
    }
    pub fn norm(mut self) -> Vec3 {
        let l = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x = self.x * l;
        self.y = self.y * l;
        self.z = self.z * l;
        self
    }
    pub fn dot(&self, b: &Vec3) -> f64 {
        return self.x * b.x + self.y * b.y + self.z * b.z;
    }
    pub fn length(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Rem for Vec3 {
    type Output = Vec3;
    fn rem(self, rhs: Self) -> Self {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

fn clamp(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    }
}

fn to_int(x: f64) -> u8 {
    (clamp(x).powf(1.0 / 2.2) * 255.0 + 0.5) as u8
}

#[allow(dead_code)]
pub fn save_ppm_file(filename: &str, image: Vec<Color>, width: usize, height: usize) {
    let mut f = fs::File::create(filename).unwrap();
    
    writeln!(f, "P3\n{} {}\n{}", width, height, 255).unwrap();
    for i in 0..(width * (height)) {
        write!(
            f,
            "{} {} {} ",
            to_int(image[i as usize].x),
            to_int(image[i as usize].y),
            to_int(image[i as usize].z)
        )
        .unwrap();
    }
}

/// 画像データをPPM形式でファイルに保存します。
/// filenameに拡張子がない、または".ppm"でない場合は、自動で".ppm"に修正します。
pub fn save_ppm_file2(filename: &str, image: Vec<Color>, width: usize, height: usize) {
    // 1. PathBufを作成し、拡張子をチェック・修正する
    let path = Path::new(filename);
    let mut final_path = path.to_path_buf();

    // 既存の拡張子を取得し、小文字にして".ppm"と比較します
    let extension_is_ppm = final_path
        .extension()
        .and_then(|ext| ext.to_str()) // OsStrから&strへ変換
        .map(|ext| ext.eq_ignore_ascii_case("ppm")) // 小文字・大文字を無視して"ppm"と比較
        .unwrap_or(false); // 拡張子がない場合はfalseとする

    if !extension_is_ppm {
        // 拡張子がない、または"ppm"でない場合は、"ppm"に設定/置換する
        // 例: "output" -> "output.ppm"
        // 例: "output.jpg" -> "output.ppm"
        final_path.set_extension("ppm");
    }

    // 処理後のファイル名を表示（確認用）
    println!("Saving image to: {}", final_path.display());

    // 2. ファイルを作成し、失敗したらパニック
    // final_pathはPathBufなので、参照(&Path)として渡します
    let mut f = fs::File::create(&final_path).unwrap();
    
    // 3. ヘッダー情報の書き込み
    writeln!(f, "P3\n{} {}\n{}", width, height, 255).unwrap();

    // 4. ピクセルデータの書き込み
    // PPMファイルは通常、3ピクセルごとに改行やスペースを入れますが、
    // 簡易的なPPMビューアの互換性を考慮し、一行に収まるように書き出します。
    for i in 0..(width * (height)) {
        write!(
            f,
            "{} {} {} ",
            to_int(image[i as usize].x),
            to_int(image[i as usize].y),
            to_int(image[i as usize].z)
        )
        .unwrap();
    }
    println!("File saved successfully.");
}

