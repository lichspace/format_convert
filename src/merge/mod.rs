extern crate imagefmt;
use imagefmt::{ColFmt, ColType};
use super::utils::set_background;

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
