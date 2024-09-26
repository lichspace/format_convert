use image::{codecs::jpeg::JpegEncoder, DynamicImage, ImageBuffer, Rgb, Rgba};
use imagefmt::{ColFmt, ColType};
use std::{fs::File, path::Path, time::Instant};

pub fn set_background(
    mut image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    transparent: bool,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    image.pixels_mut().for_each(|p| {
        let a = if transparent { 0 } else { 255 };
        if p[3] == 0 {
            p[0] = 255;
            p[1] = 255;
            p[2] = 255;
            p[3] = a;
        } else if p[0] == 255 && p[1] == 255 && p[2] == 255 {
            p[3] = a;
        }
    });

    return image;
}

pub fn save(dist: &str, ibuf: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let ext = get_file_extension(dist);
    let (w, h) = ibuf.dimensions();
    println!("save format: {}", ext.unwrap());
    match ext {
        Option::Some("tga") => {
            let buf = ibuf.as_raw();

            // can open in paintman
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
        Option::Some("jpg") => {
            let rgb = rbga2rgb(ibuf);
            let mut file = File::create(dist).unwrap();
            let encoder = JpegEncoder::new_with_quality(&mut file, 10);
            let start = Instant::now();
            rgb.write_with_encoder(encoder).unwrap();
            let duration = start.elapsed();
            println!("save: {:?}", duration);
        }
        _ => ibuf.save(dist).unwrap(),
    }
}

pub fn get_file_extension(path: &str) -> Option<&str> {
    let file_path = Path::new(path);
    let ext = file_path.extension().and_then(|ext| ext.to_str());
    return ext;
}

pub fn rbga2rgb(input: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let dy = DynamicImage::from(input);
    return  dy.into_rgb8();
}
