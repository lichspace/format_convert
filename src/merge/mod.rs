extern crate imagefmt;
use std::time::Instant;

use imagefmt::{ColFmt, ColType, Image};

pub fn over(src: &String, over: &String, dist: &String) {
    let start = Instant::now();
    let src_img = imagefmt::read(src, ColFmt::Auto).unwrap();
    let over_img = imagefmt::read(over, ColFmt::Auto).unwrap();

    let mut buf: Vec<u8> = src_img.buf;
    let buf_over: Vec<u8> = over_img.buf;

    let Image { w, h, .. } = src_img;
    let w2 = over_img.w;
    let is_rgba = src_img.fmt == ColFmt::RGBA;
    let channel: usize = if src_img.fmt == ColFmt::RGBA {4} else {3};
    let channel2: usize = if over_img.fmt == ColFmt::RGBA {4} else {3};


    for y in 0..h {
        for x in 0..w {
            let index = (y * w + x) * channel;

            // remove alpha
            if is_rgba && buf[index + 3] == 0 {
                buf[index + 0] = 255;
                buf[index + 1] = 255;
                buf[index + 2] = 255;
            }

            let index2 = (y * w2 + x) * channel2;
            let r_o: u8 = buf_over[index2 + 0];
            let g_o: u8 = buf_over[index2 + 1];
            let b_o: u8 = buf_over[index2 + 2];

            // 非透明、非白色部分合并
            let w: bool = r_o == 255 && g_o == 255 && b_o == 255;

            if x > over_img.w || y > over_img.h {
                return;
            }

            if over_img.fmt == ColFmt::RGBA && buf_over[index2 + 3] != 0 && !w {
                buf[index + 0] = r_o;
                buf[index + 1] = g_o;
                buf[index + 2] = b_o;
            }

            if over_img.fmt == ColFmt::RGB && !w {
                buf[index + 0] = r_o;
                buf[index + 1] = g_o;
                buf[index + 2] = b_o;
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
