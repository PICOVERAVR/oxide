use std::fs::File;
use std::io::{BufWriter, Write};

/// Basic trait for interacting with images of different file formats.
pub trait Output {
    /// Write an array of bytes to the file.
    fn write(&mut self, buf: &[u8]);
}

/// Struct for .ppm output.
/// Incredibly easy to create and parse but inefficient in terms of memory and does not support advanced features such as transparency or sRGB.
pub struct PPM {
    f: std::fs::File,
}

impl PPM {
    pub fn new(name: String, w: usize, h: usize, bits: usize) -> PPM {
        let mut f = File::create(name).expect("could not create .ppm image");
        let header = format!("P6 {} {} {}\n", w, h, 2usize.pow(bits as u32) - 1);
        f.write_all(header.as_bytes())
            .expect("could not write .ppm image header");

        PPM { f }
    }
}

impl Output for PPM {
    fn write(&mut self, buf: &[u8]) {
        self.f
            .write_all(buf)
            .expect("could not write .ppm image content");
    }
}

/// Struct for .png output.
pub struct PNG {
    writer: png::Writer<std::io::BufWriter<std::fs::File>>,
}

impl PNG {
    pub fn new(name: String, w: usize, h: usize, bits: usize) -> PNG {
        let f = File::create(name).expect("could not create file");
        let b = BufWriter::new(f);
        let mut enc = png::Encoder::new(b, w as u32, h as u32);

        enc.set_color(png::ColorType::Rgb);

        let depth = match bits {
            1 => png::BitDepth::One,
            2 => png::BitDepth::Two,
            4 => png::BitDepth::Four,
            8 => png::BitDepth::Eight,
            16 => png::BitDepth::Sixteen,
            _ => panic!("cannot represent bit depth in .png output"),
        };

        enc.set_depth(depth);

        // optimize sRGB gamma for output device gamut vs absolute color accuracy
        enc.set_srgb(png::SrgbRenderingIntent::Perceptual);

        let writer = enc
            .write_header()
            .expect("could not write .png image header");

        PNG { writer }
    }
}

impl Output for PNG {
    fn write(&mut self, buf: &[u8]) {
        self.writer
            .write_image_data(buf)
            .expect("could not write .png image content");
    }
}
