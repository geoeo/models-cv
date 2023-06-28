extern crate png;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::result::Result;
use png::EncodingError;

pub fn write_data_to_file(path_str: &str, data_vec: &Vec<u8>, screen_width: u32, screen_height: u32) -> Result<(),EncodingError> {
    let path = Path::new(path_str);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, screen_width, screen_height); 
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data_vec[..]) // Save
}