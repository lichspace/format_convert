use std::env;
use std::time::Instant;
extern crate imagefmt;
use imagefmt::{ColFmt, ColType};

fn main() {
    let args: Vec<String> = env::args().collect();
    let src: &String = &args[1];
    let dist = &args[2];
    println!("convert: {} ==> {}", src, dist);
    let start = Instant::now();
    let png = imagefmt::read(src, ColFmt::RGBA).unwrap();
    println!("Image Info: {:?}", png);
    imagefmt::write(dist, png.w, png.h, png.fmt, &png.buf, ColType::ColorAlpha).unwrap();
    let duration = start.elapsed();
    println!("imagefmt time: {:?}", duration);
}
