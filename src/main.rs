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
    let src_img = imagefmt::read(src, ColFmt::Auto).unwrap();
    println!("Image Info: {:?}", src_img);
    imagefmt::write(dist, src_img.w, src_img.h, src_img.fmt, &src_img.buf, ColType::ColorAlpha).unwrap();
    let duration = start.elapsed();
    println!("imagefmt time: {:?}", duration);
}
