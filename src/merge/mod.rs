extern crate imagefmt;
use image::{ImageBuffer, Rgba};
use imagefmt::{ColFmt, ColType};

pub fn over_multi(inputs: Vec<&str>, dist: &str, transparent: bool) {
    let mut arr = inputs
        .iter()
        .map(|s| image::open(s).unwrap().into_rgba8())
        .enumerate();

    let (_, mut bottom) = arr.next().unwrap();
    
    bottom = set_background(bottom.clone(), transparent);

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
        ColType::ColorAlpha,
    )
    .unwrap();
}


pub fn set_background(mut image: ImageBuffer<Rgba<u8>, Vec<u8>>, transparent: bool)->ImageBuffer<Rgba<u8>, Vec<u8>>{
    for p in image.pixels_mut() {
        if p[3] == 0 {
            p[0] = 255;
            p[1] = 255;
            p[2] = 255;
            p[3] = if transparent { 0 } else { 255 };
        } else if p[0] == 255 && p[1] == 255 && p[2] == 255 {
            p[3] = if transparent { 0 } else { 255 };
        }
    }

    return image;
}