extern crate imagefmt;
use std::{fs::File, io::Read};
use crate::utils::{save, set_background};

pub fn convert(src: &str, dist: &str, transparent: bool) {
    let mut file = File::open(src).expect("无法打开文件");
    // 读取文件内容到缓冲区
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("无法读取文件");
    let bbox = buffer.into_boxed_slice();
    let buf = bbox.into_iter().as_slice();
    let format = image::guess_format(buf).unwrap();
    let image = image::load_from_memory_with_format(buf, format).unwrap();
    let mut image = image.to_rgba8();
    image = set_background(image.clone(), transparent);
    save(dist, image);
}
