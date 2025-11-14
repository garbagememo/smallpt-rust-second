use std::env;
use std::path::Path;
use image::{self, ImageFormat};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // コマンドライン引数のチェック
    if args.len() != 3 {
        eprintln!("使用方法: {} <入力_PPMファイル> <出力_PNGファイル>", args[0]);
        eprintln!("例: {} input.ppm output.png", args[0]);
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);

    println!("OK! 変換を開始します...");
    println!("入力ファイル: {:?}", input_path);
    println!("出力ファイル: {:?}", output_path);

    // 1. 画像の読み込み (PPMを含む様々な形式に対応)
    match image::open(input_path) {
        Ok(img) => {
            println!("OK! 画像ファイルの読み込みに成功しました。");

            // 2. PNGフォーマットでファイルに書き出す
            // `img.save_with_format()` は、ファイルパスとフォーマットを指定して書き込みます。
            match img.save_with_format(output_path, ImageFormat::Png) {
                Ok(_) => {
                    println!("OK! 変換と書き出しが正常に完了しました！");
                }
                Err(e) => {
                    eprintln!("NG! 書き出しエラー: PNGファイルへの書き込みに失敗しました: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("NG! 読み込みエラー: 画像ファイルの読み込みに失敗しました: {}", e);
            eprintln!("PPMファイルが **P3** または **P6** 形式であることを確認してください。");
        }
    }
}
