use std::path::Path;
use std::time::Instant;
extern crate imagefmt;
use imagefmt::{ColFmt, ColType, Image};

pub fn convert(src: &str, dist: &str) {
    let extension = Path::new(&src)
        .extension()
        .map(|ext| ext.to_string_lossy())
        .unwrap_or_else(|| "".into());

    let start = Instant::now();
    let src_img = imagefmt::read(src, ColFmt::Auto).unwrap();
    println!("Image Info: {:?}", src_img);
    let mut buf: Vec<u8> = src_img.buf;

    if src_img.fmt == ColFmt::RGBA && extension == "png" {
        println!("PNG RGBA remove alpha");
        let Image { w, h, .. } = src_img;
        for y in 0..h {
            for x in 0..w {
                let index = (y * w + x) * 4;
                let a = buf[index + 3];
                if a == 0 {
                    buf[index + 0] = 255;
                    buf[index + 1] = 255;
                    buf[index + 2] = 255;
                }
            }
        }
    }

    imagefmt::write(
        dist,
        src_img.w,
        src_img.h,
        src_img.fmt,
        &buf,
        ColType::Color,
    )
    .unwrap();
    let duration = start.elapsed();
    println!("convert time: {:?}", duration);
}
