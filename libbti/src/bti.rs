use std::io::{Read, Seek, Write};
use crate::enums::*;
use crate::palette::Palette;
use crate::*;
use crate::imadedataformat::Converter;
use crate::encoders::*;
use binrw::prelude::*;
use binrw::Endian;
use binrw::WriteOptions;
use image::*;

pub fn writer_options(endian: Option<Endian>) -> WriteOptions {
    let endian = match endian {
        Some(e) => e,
        None => Endian::Big
    };
    WriteOptions::new(endian)
}

#[derive(Debug, Clone, Default)]
pub struct BTI {
    pub format: TextureFormats,
    pub alphasetting: u8,
    pub width: u16,
    pub height: u16,
    pub wraps: WrapNodes,
    pub wrapt: WrapNodes,
    pub palettesenabled: bool,
    pub paletteformat: PaletteFormats,
    pub palettecount: u16,
    pub palettedataoffset: i32,
    pub embeddedpaletteoffset: i32,
    pub minfilter: FilterMode,
    pub magfilter: FilterMode,
    pub unknown2: i16,
    pub mipmapcount: u8,
    pub unknown3: u8,
    pub lodbias: i16,
    pub imagedataoffset: i32,
    pub imagepalette: Palette,
    pub rgbaimagedata: Vec<u8>
}

impl BTI {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Self {
        let mut res = Self::default();
        res.format = reader.read_ne().unwrap();
        res.alphasetting = reader.read_ne().unwrap();
        res.width = reader.read_be().unwrap();
        res.height = reader.read_be().unwrap();
        res.wraps = reader.read_ne().unwrap();
        res.wrapt = reader.read_ne().unwrap();
        res.palettesenabled = reader.read_type::<u8>(Endian::NATIVE).unwrap() != 0;
        res.paletteformat = reader.read_ne().unwrap();
        res.palettecount = reader.read_be().unwrap();
        res.palettedataoffset = reader.read_be().unwrap();
        res.embeddedpaletteoffset = reader.read_be().unwrap();
        res.minfilter = reader.read_ne().unwrap();
        res.magfilter = reader.read_ne().unwrap();
        res.unknown2 = reader.read_be().unwrap();
        res.mipmapcount = reader.read_be().unwrap();
        res.unknown3 = reader.read_be().unwrap();
        res.lodbias = reader.read_be().unwrap();
        res.imagedataoffset = reader.read_be().unwrap();
        res.imagepalette = Palette::read(reader, res.palettecount);
        res.rgbaimagedata = decoders::decode(reader, &res);
        res
    }

    pub fn into_image(self) -> RgbaImage {
        self.into()
    }

    pub fn from_image(img: RgbaImage) -> Self {
        Self::from(img)
    }

    pub fn write_header<W: Write + Seek>(&self, writer: &mut W) {
        let options = &writer_options(None);
        (self.format as u8).write_options(writer, options, ()).unwrap();
        self.alphasetting.write_options(writer, options, ()).unwrap();
        self.width.write_options(writer, options, ()).unwrap();
        self.height.write_options(writer, options, ()).unwrap();
        (self.wraps as u8).write_options(writer, options, ()).unwrap();
        (self.wrapt as u8).write_options(writer, options, ()).unwrap();
        match self.palettesenabled {
            true => 1u8,
            false => 0u8
        }.write_options(writer, options, ()).unwrap();
        (self.paletteformat as u8).write_options(writer, options, ()).unwrap();
        (self.palettecount as i16).write_options(writer, options, ()).unwrap();
        0i32.write_options(writer, options, ()).unwrap();
        self.embeddedpaletteoffset.write_options(writer, options, ()).unwrap();
        (self.minfilter as u8).write_options(writer, options, ()).unwrap();
        (self.magfilter as u8).write_options(writer, options, ()).unwrap();
        self.unknown2.write_options(writer, options, ()).unwrap();
        self.mipmapcount.write_options(writer, options, ()).unwrap();
        self.lodbias.write_options(writer, options, ()).unwrap();
        0i32.write_options(writer, options, ()).unwrap();
    }

    pub fn encode<W: Write + Seek>(&self, writer: &mut W) {
        let Self {
            width, height, rgbaimagedata, ..
        } = self;
        let width = *width as i32;
        let height = *height as i32;
        let data = match self.format {
            TextureFormats::I4 => I4::convertto(rgbaimagedata, width, height),
            TextureFormats::I8 => I8::convertto(rgbaimagedata, width, height),
            TextureFormats::IA4 => IA4::convertto(rgbaimagedata, width, height),
            TextureFormats::IA8 => IA8::convertto(rgbaimagedata, width, height),
            TextureFormats::RGB565 => RGB565::convertto(rgbaimagedata, width, height),
            TextureFormats::RGB5A3 => RGB5A3::convertto(rgbaimagedata, width, height),
            TextureFormats::RGBA32 => RGBA32::convertto(rgbaimagedata, width, height),
            TextureFormats::CMPR => CMPR::convertto(rgbaimagedata, width, height),
            _ => unimplemented!("Other Texture Formats are currently not supported.")
        };
        writer.write_all(&data).unwrap();
    }

    pub fn write_and_encode<W: Write + Seek>(&self, writer: &mut W) {
        self.write_header(writer);
        self.encode(writer);
    }
}

impl Into<RgbaImage> for BTI {
    fn into(self) -> RgbaImage {
        let mut res = RgbaImage::from_raw(self.width.into(),
        self.height.into(), self.rgbaimagedata)
        .unwrap();
        // Converts BGRA pixels into RGBA pixels.
        for w in 0..res.width() {
            for h in 0..res.height() {
                let mut pix = res[(w, h)];
                let b = pix[0];
                let r = pix[2];
                pix[0] = r;
                pix[2] = b;
                res[(w, h)] = pix;
            }
        }
        res
    }
}

impl From<RgbaImage> for BTI {
    fn from(img: RgbaImage) -> Self {
        let mut res = Self::default();
        res.format = TextureFormats::CMPR;
        res.magfilter = FilterMode::Linear;
        res.minfilter = FilterMode::Linear;
        res.width = img.width() as u16;
        res.height = img.height() as u16;
        let mut copy = img.clone();
        drop(img);
        for x in 0..copy.width() {
            for y in 0..copy.height() {
                let mut pix = copy[(x, y)];
                let b = pix[2];
                let r = pix[0];
                pix[0] = b;
                pix[2] = r;
                copy[(x, y)] = pix;
            }
        }
        res.rgbaimagedata = copy.into_raw();
        decectandsetsittingformat(&mut res);
        res
    }
}

pub fn decectandsetsittingformat(res: &mut BTI) {
    let mut is_gray = true;
    let mut complex_alpha = false;
    let mut has_alpha = false;
    for i in 0..(res.rgbaimagedata.len() / 4) {
        let r = res.rgbaimagedata[i * 4];
        let b = res.rgbaimagedata[i * 4 + 1];
        let g = res.rgbaimagedata[i * 4 + 2];
        let a = res.rgbaimagedata[i * 4 + 3];
        if is_gray && (r != g || g != b || b != r) {
            is_gray = false;
        }
        if a != 255 {
            has_alpha = true;
            if a != 0 {
                complex_alpha = true;
            }
        }
    }
    if is_gray {
        res.format = TextureFormats::I8;
    } else if complex_alpha {
        res.format = TextureFormats::RGB5A3;
    } else {
        res.format = TextureFormats::CMPR;
    }
    if has_alpha {
        res.alphasetting = 0x1;
    }
}

