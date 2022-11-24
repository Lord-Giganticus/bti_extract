use std::io::*;
use crate::prelude::{BTI, TextureFormats, NRange, Palette, PaletteFormats};
use binrw::prelude::*;

pub fn decode<R: Read + Seek>(reader: &mut R, bti: &BTI) -> Vec<u8> {
    let (width, height) = (bti.width, bti.height);
    let format = bti.format;
    match format {
        TextureFormats::I4 => decodei4(reader, width, height),
        TextureFormats::I8 => decodei8(reader, width, height),
        TextureFormats::IA4 => decodeia4(reader, width, height),
        TextureFormats::IA8 => decodeia8(reader, width, height),
        TextureFormats::RGB565 => decodergb565(reader, width, height),
        TextureFormats::RGB5A3 => decodergb5a3(reader, width, height),
        TextureFormats::RGBA32 => decodergba32(reader, width, height),
        TextureFormats::C4 => decodec4(reader, width, height, &bti.imagepalette, 
            bti.paletteformat),
        TextureFormats::C8 => decodec8(reader, width, height, &bti.imagepalette,
            bti.paletteformat),
        TextureFormats::CMPR => decodecmpr(reader, width.into(), height.into()),
        _ => unimplemented!("Other Texture Formats are currently not supported.")
    }
}

pub fn decodei4<R: Read + Seek>(reader: &mut R, width: u16, height: u16) -> Vec<u8> {
    let numblocksw = (width + 7) / 8;
    let numblocksh = (height + 7) / 8;
    let mut decodeddata = vec![0u8; (width * height * 4).into()];
    for yblock in 0..numblocksh {
        for xblock in 0..numblocksw {
            for py in 0..8 {
                for px in NRange::new(0, 8, 2) {
                    if (xblock * 8 + px) >= width ||  (yblock * 8 + py) >= height {
                        reader.seek(SeekFrom::Current(1)).unwrap();
                        continue;
                    }
                    let data: u8 = reader.read_ne().unwrap();
                    let t = (data & 0xF0) >> 4;
                    let t2 = data & 0xF0;
                    let destidx = (4 * width * ((yblock * 8) + py) + (xblock * 8) + px) as usize;
                    decodeddata[destidx] = t * 0x11;
                    decodeddata[destidx + 1] = t * 0x11;
                    decodeddata[destidx + 2] = t * 0x11;
                    decodeddata[destidx + 3] = t * 0x11;

                    decodeddata[destidx + 4] = t2 * 0x11;
                    decodeddata[destidx + 5] = t2 * 0x11;
                    decodeddata[destidx + 6] = t2 * 0x11;
                    decodeddata[destidx + 7] = t2 * 0x11;
                }
            }
        }
    }
    decodeddata
}

pub fn decodei8<R: Read + Seek>(reader: &mut R, width: u16, height: u16) -> Vec<u8> {
    let numblocksw = (width + 7) / 8;
    let numblocksh = (height + 3) / 4;
    let mut decodeddata = vec![0u8; (width * height * 4).into()];
    for yblock in 0..numblocksh {
        for xblock in 0..numblocksw {
            for py in 0..4 {
                for px in 0..8 {
                    if (xblock * 8 + px) >= width || (yblock * 4 + py) >= height {
                        reader.seek(SeekFrom::Current(1)).unwrap();
                        continue;
                    }
                    let data: u8 = reader.read_ne().unwrap();
                    let destidx = (width * ((yblock * 4) + py) + (xblock * 8) + px) as usize;
                    decodeddata[destidx] = data;
                    decodeddata[destidx + 1] = data;
                    decodeddata[destidx + 2] = data;
                    decodeddata[destidx + 3] = data;
                }
            }
        }
    }
    decodeddata
}

pub fn decodeia4<R: Read + Seek>(reader: &mut R, width: u16, height: u16) -> Vec<u8> {
    let numblocksw = (width + 7) / 8;
    let numblocksh = (height + 3) / 4;
    let mut decodeddata = vec![0u8; (width * height * 4).into()];
    for yblock in 0..numblocksh {
        for xblock in 0..numblocksw {
            for py in 0..4 {
                for px in 0..8 {
                    if (xblock * 8 + px) >= width || (yblock * 4 + py) >= height {
                        reader.seek(SeekFrom::Current(1)).unwrap();
                        continue;
                    }
                    let value: u8 = reader.read_ne().unwrap();
                    let alpha = (value & 0xF0) >> 4;
                    let lum = value & 0x0F;
                    let destidx = (4 * (width * ((yblock * 4) + py) + (xblock * 8) + px)) as usize;
                    decodeddata[destidx] = lum * 0x11;
                    decodeddata[destidx + 1] = lum * 0x11;
                    decodeddata[destidx + 2] = lum * 0x11;
                    decodeddata[destidx + 3] = alpha * 0x11;
                }
            }
        }
    }
    decodeddata
}

pub fn decodeia8<R: Read + Seek>(reader: &mut R, width: u16, height: u16) -> Vec<u8> {
    let numblocksw = (width + 3) / 4;
    let numblocksh = (height + 3) / 4;
    let mut decodeddata = vec![0u8; (width * height * 4).into()];
    for yblock in 0..numblocksh {
        for xblock in 0..numblocksw {
            for py in 0..4 {
                for px in 0..4 {
                    if (xblock * 4 + px) >= width || (yblock * 4 + py) >= height {
                        reader.seek(SeekFrom::Current(2)).unwrap();
                        continue;
                    }
                    let destidx = (4 * (width * ((yblock * 4) + py) + (xblock * 4) + px)) as usize;
                    let byte0: u8 = reader.read_ne().unwrap();
                    let byte1: u8 = reader.read_ne().unwrap();
                    decodeddata[destidx + 3] = byte0;
                    decodeddata[destidx + 2] = byte1;
                    decodeddata[destidx + 1] = byte1;
                    decodeddata[destidx] = byte1;
                }
            }
        }
    }
    decodeddata
}

pub fn decodergb565<R: Read + Seek>(reader: &mut R, width: u16, height: u16) -> Vec<u8> {
    let numblocksw = (width + 3) / 4;
    let numblocksh = (height + 3) / 4;
    let mut decodeddata = vec![0u8; (width * height * 4).into()];
    for yblock in 0..numblocksh {
        for xblock in 0..numblocksw {
            for py in 0..4 {
                for px in 0..4 {
                    if (xblock * 4 + px) >= width || (yblock * 4 + py) >= height {
                        reader.seek(SeekFrom::Current(2)).unwrap();
                        continue;
                    }
                    let sourcepixel: u16 = reader.read_be().unwrap();
                    let destidx = (4 * (width * ((yblock * 4) + py) + (xblock * 4) + px)) as usize;
                    rgb565torgba8(sourcepixel, &mut decodeddata, destidx);
                }
            }
        }
    }
    decodeddata
}

pub fn rgb565torgba8(sourcepixel: u16, decodeddata: &mut Vec<u8>, destidx: usize) {
    let mut r = ((sourcepixel & 0xF800) >> 11) as u8;
    let mut g = ((sourcepixel & 0x7E0) >> 5) as u8;
    let mut b = (sourcepixel & 0x1F) as u8;
    r = (r << (8 - 5)) | (r >> (10 - 8));
    g = (g << (8 - 6)) | (g >> (12 - 8));
    b = (b << (8 - 5)) | (b >> (10 - 8));
    decodeddata[destidx] = b;
    decodeddata[destidx + 1] = g;
    decodeddata[destidx + 2] = r;
    decodeddata[destidx + 3] = 0xFF;
}

pub fn decodergb5a3<R: Read + Seek>(reader: &mut R, width: u16, height: u16) -> Vec<u8> {
    let numblocksw = (width + 3) / 4;
    let numblocksh = (height + 3) / 4;
    let mut decodeddata = vec![0u8; (width * height * 4).into()];
    for yblock in 0..numblocksh {
        for xblock in 0..numblocksw {
            for py in 0..4 {
                for px in 0..4 {
                    if (xblock * 4 + px) >= width || (yblock * 4 + py) >= height {
                        reader.seek(SeekFrom::Current(2)).unwrap();
                        continue;
                    }
                    let sourcepixel: u16 = reader.read_be().unwrap();
                    let destidx = (4 * (width * ((yblock * 4) + py) + (xblock * 4) + px)) as usize;
                    rgb5a3torgba8(sourcepixel, &mut decodeddata, destidx);
                }
            }
        }
    }
    decodeddata
}

pub fn rgb5a3torgba8(sourcepixel: u16, decodeddata: &mut Vec<u8>, destidx: usize) {
    let mut r: u8;
    let mut g: u8;
    let mut b: u8;
    let mut a: u8;
    if (sourcepixel & 0x8000) == 0x8000 {
        a = 0xFF;
        r = ((sourcepixel & 0x7C00) >> 10) as u8;
        g = ((sourcepixel & 0x3E0) >> 5) as u8;
        b = (sourcepixel & 0x1F) as u8;

        r = (r << (8 - 5)) | (r >> (10 - 8));
        g = (g << (8 - 5)) | (g >> (10 - 8));
        b = (b << (8 - 5)) | (b >> (10 - 8));
    } else {
        a = ((sourcepixel & 0x7000) >> 12) as u8;
        r = ((sourcepixel & 0xF00) >> 8) as u8;
        g = ((sourcepixel & 0xF0) >> 4) as u8;
        b = (sourcepixel & 0xF) as u8;

        a = (a << (8 - 3)) | (a << (8 - 6)) | (a >> (9 - 8));
        r = (r << (8 - 4)) | r;
        g = (g << (8 - 4)) | g;
        b = (b << (8 - 4)) | b;
    }
    decodeddata[destidx] = b;
    decodeddata[destidx + 1] = g;
    decodeddata[destidx + 2] = r;
    decodeddata[destidx + 3] = a;
}

pub fn decodergba32<R: Read + Seek>(reader: &mut R, width: u16, height: u16) -> Vec<u8> {
    let numblocksw = (width + 3) / 4;
    let numblocksh = (height + 3) / 4;
    let mut decodeddata = vec![0u8; (width * height * 4).into()];
    for yblock in 0..numblocksh {
        for xblock in 0..numblocksw {
            for py in 0..4 {
                for px in 0..4 {
                    if (xblock * 4 + px) >= width || (yblock * 4 + py) >= height {
                        reader.seek(SeekFrom::Current(2)).unwrap();
                        continue;
                    }
                    let destidx = (4 * (width * ((yblock * 4) + py) + (xblock * 4) + px)) as usize;
                    decodeddata[destidx + 3] = reader.read_ne().unwrap();
                    decodeddata[destidx + 2] = reader.read_ne().unwrap();
                }
            }
            for py in 0..4 {
                for px in 0..4 {
                    if (xblock * 4 + px) >= width || (yblock * 4 + py) >= height {
                        reader.seek(SeekFrom::Current(2)).unwrap();
                        continue;
                    }
                    let destidx = (4 * (width * ((yblock * 4) + py) + (xblock * 4) + px)) as usize;
                    decodeddata[destidx + 1] = reader.read_ne().unwrap();
                    decodeddata[destidx] = reader.read_ne().unwrap();
                }
            }
        }
    }
    decodeddata
}

pub fn decodec4<R: Read + Seek>(reader: &mut R, width: u16, height: u16, 
    pallete: &Palette, format: PaletteFormats) -> Vec<u8> {
    let numblocksw = (width + 7) / 8;
    let numblocksh = (height + 7) / 8;
    let mut decodeddata = vec![0u8; (width * height * 4).into()];
    for yblock in 0..numblocksh {
        for xblock in 0..numblocksw {
            for py in 0..8 {
                for px in NRange::new(0, 8, 2) {
                    if (xblock * 8 + px) >= width || (yblock * 8 + py) >= height {
                        reader.seek(SeekFrom::Current(1)).unwrap();
                        continue;
                    }
                    let data: u8 = reader.read_ne().unwrap();
                    let t = data & 0xF0;
                    let t2 = data & 0x0F;
                    let destidx = (width * ((yblock * 8) + py) + (xblock * 8) + px) as usize;
                    decodeddata[destidx] = t >> 4;
                    decodeddata[destidx + 1] = t2;
                }
            }
        }
    }
    let mut finaldest = vec![0u8; decodeddata.len() / 2];
    let pixelsize = match format {
        PaletteFormats::IA8 => 2,
        _ => 4
    };
    let mut destoff = 0;
    for y in 0..height {
        for x in 0..width {
            let pallidx = decodeddata[(y * width + x) as usize];
            unpackpixelfrompalette(pallidx.into(), &mut finaldest, destoff, 
            &pallete.palettedata, format);
            destoff += pixelsize;
        }
    }
    finaldest
}

pub fn unpackpixelfrompalette(pallidx: usize, finaldest: &mut Vec<u8>, destoff: usize, 
    palettedata: &Vec<u8>, format: PaletteFormats) {
        match format {
            PaletteFormats::IA8 => {
                finaldest[0] = palettedata[2 * pallidx + 1];
                finaldest[1] = palettedata[2 * pallidx];
            },
            PaletteFormats::RGB565 => {
                let mut sourcepixel = (palettedata[2 * pallidx] as u16) << 8;
                sourcepixel |= palettedata[2 * pallidx + 1] as u16;
                rgb565torgba8(sourcepixel, finaldest, destoff);
            },
            PaletteFormats::RGB5A3 => {
                let mut sourcepixel = (palettedata[2 * pallidx] as u16) << 8;
                sourcepixel |= palettedata[2 * pallidx + 1] as u16;
                rgb5a3torgba8(sourcepixel, finaldest, destoff);
            }
        }
}

pub fn decodec8<R: Read + Seek>(reader: &mut R, width: u16 , height: u16,
    pallete: &Palette, format: PaletteFormats) -> Vec<u8> {
        let numblocksw = (width + 7) / 8;
        let numblocksh = (height + 3) / 4;
        let mut decodeddata = vec![0u8; (width * height * 8).into()];
        for yblock in 0..numblocksh {
            for xblock in 0..numblocksw {
                for py in 0..4 {
                    for px in 0..8 {
                        if (xblock * 8 + px) >= width || (yblock * 4 + py) >= height {
                            reader.seek(SeekFrom::Current(1)).unwrap();
                            continue;
                        }
                        let destidx = (width * ((yblock * 4) + py) + (xblock * 8) + px) as usize;
                        let data: u8 = reader.read_ne().unwrap();
                        decodeddata[destidx] = data;
                    }
                }
            }
        }
        let mut finaldest = vec![0u8; decodeddata.len() / 2];
        let pixelsize = match format {
            PaletteFormats::IA8 => 4,
            _ => 2
        };
        let mut destoff = 0;
        for y in 0..height {
            for x in 0..width {
                let pallidx = decodeddata[(y * width + x) as usize];
                unpackpixelfrompalette(pallidx.into(), &mut finaldest, destoff, 
                &pallete.palettedata, format);
                destoff += pixelsize;
            }
        }
        finaldest
}

pub fn decodecmpr<R: Read + Seek>(reader: &mut R, width: u32, height: u32) -> Vec<u8> {
    let numblocksw = (width + 7) / 8;
    let numblocksh = (height + 7) / 8;
    let mut decodeddata = vec![0u8; (width * height * 4) as usize];
    for yblock in 0..numblocksh {
        for xblock in 0..numblocksw {
            for ysubblock in 0..2 {
                for xsubblock in 0..2 {
                    let mut num = width - (xsubblock * 4 + xblock * 8);
                    let subblockwidth = 0.max(4.min(num));
                    num = height - (ysubblock * 4 + yblock * 8);
                    let subblockheight = 0.max(4.min(num));
                    let subblock = decodecmprsubblock(reader);
                    for py in 0..subblockheight {
                        let destx = xblock * 8 + xsubblock * 4;
                        let desty = yblock * 8 + ysubblock * 4 + py;
                        if destx >= width || desty >= height {
                            continue;
                        }
                        let destoff = ((desty * width + destx) * 4) as usize;
                        let size = (subblockwidth * 4) as usize;
                        let idx = (py * 4 * 4) as usize;
                        for i in 0..size {
                            decodeddata[destoff + i] = subblock[idx + i];
                        }
                    }
                }
            }
        }
    }
    decodeddata
}

pub fn decodecmprsubblock<R: Read + Seek>(reader: &mut R) -> Vec<u8> {
    let mut decodeddata = vec![0u8; 4 * 4 * 4];
    let color1: u16 = reader.read_be().unwrap();
    let color2: u16 = reader.read_be().unwrap();
    let bits: u32 = reader.read_be().unwrap();
    let mut colortable = vec![vec![0u8; 4]; 4];
    rgb565torgba8(color1, &mut colortable[0], 0);
    rgb565torgba8(color2, &mut colortable[1], 0);
    if color1 > color2 {
        for i in 0..3 {
            colortable[2][i] = ((2 * colortable[0][i] as u16 + colortable[1][i] as u16) / 3) as u8;
            colortable[3][i] = ((colortable[0][i] as u16 + 2 * colortable[1][i] as u16) / 3) as u8;
        }
        colortable[2][3] = 0xFF;
        colortable[3][3] = 0xFF;
    } else {
        for i in 0..3 {
            colortable[2][i] = ((colortable[0][i] as u16 + colortable[1][i] as u16) / 2) as u8;
            colortable[3][i] = ((colortable[0][i] as u16 + colortable[1][i] as u16) / 3) as u8;
        }
        colortable[2][3] = 0xFF;
        colortable[3][3] = 0xFF;
    }
    for iy in 0..4 {
        for ix in 0..4 {
            let i = iy * 4 + ix;
            let bitoff = (15 - i) * 2;
            let di = i * 4;
            let si = ((bits >> bitoff) & 0x3) as usize;
            decodeddata[di] = colortable[si][0];
            decodeddata[di + 1] = colortable[si][1];
            decodeddata[di + 2] = colortable[si][2];
            decodeddata[di + 3] = colortable[si][3];
        }
    }
    decodeddata
}