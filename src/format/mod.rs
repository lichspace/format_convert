extern crate imagefmt;
use std::{fs::File, io::Read};
use image::ImageFormat;

use crate::utils::{save, set_background};

pub fn convert(src: &str, dist: &str, transparent: bool) {
    let mut file = File::open(src).expect("无法打开文件");
    // 读取文件内容到缓冲区
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("无法读取文件");
    let bbox = buffer.into_boxed_slice();
    let buf = bbox.into_iter().as_slice();
    let mut  format_df = ImageFormat::from_path(src).unwrap();
    let gusess_format = image::guess_format(buf);
    if gusess_format.is_ok() {
        format_df = gusess_format.unwrap();
    }
    let image = image::load_from_memory_with_format(buf, format_df).unwrap();
    let mut image = image.to_rgba8();
    image = set_background(image, transparent);
    save(dist, image);
}
