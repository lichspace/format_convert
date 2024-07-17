extern crate imagefmt;
use std::time::Instant;
use imagefmt::{ColFmt, ColType};

pub fn over_multi(inputs: Vec<&str>, dist: &str) {
    let start = Instant::now();
    let mut arr = inputs
        .iter()
        .map(|s| image::open(s).unwrap().into_rgba8())
        .enumerate();

    let (_, mut bottom) = arr.next().unwrap();

    // 底图变为白色
    for p in bottom.pixels_mut() {
        if p[3] == 0 {
            p[0] = 255;
            p[1] = 255;
            p[2] = 255;
            p[3] = 255;
        }
    }

    while let Some((_, mut top)) = arr.next() {
        for p in top.pixels_mut() {
            if p[0] == 255 && p[1] == 255 && p[2] == 255 {
                p[3] = 0;
            }
        }

        image::imageops::overlay(&mut bottom, &top, 0, 0);
    }

    // use image-rs save
    // bottom.save(dist).unwrap();

    //  why: can open in paintman
    let buf = bottom.as_raw();
    let (w, h) = bottom.dimensions();
    imagefmt::write(
        dist,
        w as usize,
        h as usize,
        ColFmt::RGBA,
        buf,
        ColType::Color,
    )
    .unwrap();

    let duration = start.elapsed();
    println!("over_multi time: {:?}", duration);
}
