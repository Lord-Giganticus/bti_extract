use libbti::prelude::BTI;
use libbti::prelude::TextureFormats;
use libbti::prelude::image::*;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;
use std::env;
use std::fs::File;

fn main() {
    let envargs: Vec<String> = env::args().collect();
    let args = (&envargs[1..]).iter().map(|x| Path::new(x))
    .collect::<Vec<_>>();
    for arg in args {
        let ext = arg.extension().unwrap().to_string_lossy();
        let stem = arg.file_stem().unwrap().to_string_lossy();
        if ext == "bti" {
            let mut file = File::open(arg).unwrap();
            let bti = BTI::read(&mut file);
            let img = bti.into_image();
            img.save_with_format(format!("{}.png", stem), ImageFormat::Png).unwrap();
        } else if ext == "png" {
            let mut file = File::open(arg).unwrap();
            let mut buf = vec![];
            file.read_to_end(&mut buf).unwrap();
            let mut reader = io::Reader::new(Cursor::new(buf));
            reader.set_format(ImageFormat::Png);
            let img = reader.decode().unwrap();
            let img = img.into_rgba8();
            let mut bti = BTI::from(img);
            let path = format!("{}.bti", stem);
            let mut file = File::create(path).unwrap();
            bti.format = TextureFormats::CMPR;
            bti.write_and_encode(&mut file);
        }
    }
}
