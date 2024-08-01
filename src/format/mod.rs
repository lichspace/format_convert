extern crate imagefmt;
use imagefmt::{ColFmt, ColType};
use crate::merge::set_background;

pub fn convert(src: &str, dist: &str, transparent: bool) {
    let mut image = image::open(src).unwrap().into_rgba8();

    image = set_background(image.clone(), transparent);


    let buf = image.as_raw();
    let (w, h) = image.dimensions();
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
