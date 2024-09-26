extern crate imagefmt;
use crate::utils::{save, set_background};

pub fn convert(src: &str, dist: &str, transparent: bool) {
    let mut image = image::open(src).unwrap().into_rgba8();
    image = set_background(image.clone(), transparent);
    save(dist, image);
}
