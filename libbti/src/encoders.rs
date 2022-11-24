use crate::prelude::{ImageDataFormat, Converter};

// ImageDataFormat impls
const I4: ImageDataFormat = ImageDataFormat::new("I4", "I4", 4, 0, 8, 8, 32, false, false, false, false, 0, 0);
const I8: ImageDataFormat = ImageDataFormat::new("I8", "I8", 8, 0, 8, 4, 32, false, false, false, false, 0, 0);
const IA4: ImageDataFormat = ImageDataFormat::new("IA4", "IA4", 8, 4, 8, 4, 32, false, false, false, false, 0, 0);
const IA8: ImageDataFormat = ImageDataFormat::new("IA8", "IA8", 16, 8, 4, 4, 32, false, false, false, false, 0, 0);
const RGB565: ImageDataFormat = ImageDataFormat::new("RGB565", "RGB565", 16, 0, 4, 4, 32, true, false, false, false, 0, 0);
const RGB5A3: ImageDataFormat = ImageDataFormat::new("RGB5A3", "RGB5A3", 16, 3, 4, 4, 32, true, false, false, false, 0, 0);
const RGBA32: ImageDataFormat = ImageDataFormat::new("RGBA32", "RGBA32", 32, 8, 4, 4, 64, true, false, false, false, 0, 0);
const CMPR: ImageDataFormat = ImageDataFormat::new("CMPR", "CMPR", 4, 1, 8, 8, 32, true, true, true, false, 0, 0);

// Encoder declarations
pub enum I4{}
pub enum I8{}
pub enum IA4{}
pub enum IA8{}
pub enum RGB565{}
pub enum RGB5A3{}
pub enum RGBA32{}
pub enum CMPR{}

// convblock impls (used in Converter trait)
impl I4 {
    fn convblock(_block: &Vec<u8>) -> Vec<u8> {
        vec![]
    }
}
impl I8 {
    fn convblock(_block: &Vec<u8>) -> Vec<u8> {
        vec![]
    }
}
impl IA4 {
    fn convblock(_block: &Vec<u8>) -> Vec<u8> {
        vec![]
    }
}
impl IA8 {
    fn convblock(_block: &Vec<u8>) -> Vec<u8> {
        vec![]
    }
}
impl RGB565 {
    fn convblock(_block: &Vec<u8>) -> Vec<u8> {
        vec![]
    }
}
impl RGB5A3 {
    fn convblock(_block: &Vec<u8>) -> Vec<u8> {
        vec![]
    }
}
impl RGBA32 {
    fn convblock(_block: &Vec<u8>) -> Vec<u8> {
        vec![]
    }
}
impl CMPR {
    fn convblock(block: &Vec<u8>) -> Vec<u8> {
        let mut result = vec![0u8; 32];
        let mut subblock = vec![0u8; 64];
        let mut x = 0;
        let mut y = 0;
        for i in 0..(block.len() / 64) {
            arrcopy(block, x + y + 0, &mut subblock, 0, 16);
            arrcopy(block, x + y + 32, &mut subblock, 16, 16);
            arrcopy(block, x + y + 64, &mut subblock, 32, 16);
            arrcopy(block, x + y + 96, &mut subblock, 48, 16);
            x = 16 - x;
            if x == 0 {
                y = 128;
            }
            let temp = convblocktoquatercmpr(&subblock);
            let sidx = i << 3;
            for j in 0..temp.len() {
                result[sidx+j] = temp[j];
            }
        }
        result
    }
}

// Converter impl macro
macro_rules! impl_converter {
    ($item:tt) => {
        impl Converter for $item {
            const FORMAT: ImageDataFormat = $item;
            const TO: fn(&Vec<u8>) -> Vec<u8> = $item::convblock;
        }
    };
    ($arg:tt, $($args:tt),+) => {
        impl_converter!($arg);
        impl_converter!($($args),+);
    };
}

// Macro uses
impl_converter!(I4, I8, IA4, IA8, RGB565, RGB5A3, RGBA32, CMPR);

// Special funcs
fn distance(color1: &Vec<u8>, off1: usize, color2: &Vec<u8>, off2: usize) -> i32 {
    let mut temp = 0;
    for i in 0..3 {
        let b1 = color1[off1 + i] as i32;
        let b2 = color2[off2 + i] as i32;
        let val = b1 - b2;
        temp += val * val;
    }
    temp
}
fn leastdistance(palette: &Vec<Vec<u8>>, color: &Vec<u8>, off: usize) -> i32 {
    if color[off + 3] < 8 {
        return 3;
    }
    let mut dist = i32::MAX;
    let mut best = 0;
    for i in 0..palette.len() {
        if palette[i][3] != 0xff {
            break;
        }
        let temp = distance(&palette[i], 0, color, off);
        if temp < dist {
            if temp == 0 {
                return i as i32;
            }
            dist = temp;
            best = i as i32;
        }
    }
    best
} 
fn arrcopy<T: Copy>(src: &Vec<T>, sidx: usize, dest: &mut Vec<T>, didx: usize, len: usize) {
    for i in 0..len {
        dest[didx+i] = src[sidx+i];
    }
}
fn convblocktoquatercmpr(block: &Vec<u8>) -> Vec<u8> {
    let mut dist = -1;
    let mut col1 = -1;
    let mut col2 = -1;
    let mut alpha = false;
    let mut result = vec![0u8; 8];
    let mut palette = vec![vec![0u8; 4]; 4];
    for i in 0..15 {
        if block[i * 4 + 3] < 16 {
            alpha = true;
        } else {
            for j in (i + 1)..16 {
                let temp = distance(block, i * 4, block, j * 4);
                if temp < dist {
                    dist = temp;
                    col1 = i as i32;
                    col2 = j as i32;
                }
            }
        }
    }
    if dist == -1 {
        palette[0] = vec![0, 0, 0, 0xff];
        palette[1].fill(0x0);
        palette[2].fill(0x0);
        palette[3].fill(0x0);
    } else {
        arrcopy(block, col1 as usize * 4, &mut palette[0], 0, 3);
        palette[0][3] = 0xff;
        arrcopy(block, col2 as usize * 4, &mut palette[1], 0, 3);
        palette[1][3] = 0xff;

        if palette[0][0] >> 3 == palette[1][0] >> 3 && palette[0][1] >> 2 == palette[1][1] >> 2 && palette[0][2] >> 3 == palette[1][2] >> 3 {
            if palette[0][0] >> 3 == 0 && palette[0][1] >> 2 == 0 && palette[0][2] >> 3 == 0 {
                palette[1][0] = 0xff;
                palette[1][1] = 0xff;
                palette[1][2] = 0xff;
            } else {
                palette[1][0] = 0x0;
                palette[1][1] = 0x0;
                palette[1][2] = 0x0;
            }
        }
    }
    result[0] = (palette[0][2] as i32 & 0xf8 | palette[0][1] as i32 >> 5) as u8;
    result[1] = ((palette[0][1] as i32) << 3 & 0xe0 | palette[0][0] as i32 >> 3) as u8;
    result[2] = (palette[1][2] as i32 & 0xf8 | palette[1][1] as i32 >> 5) as u8;
    result[3] = ((palette[1][1] as i32) << 3 & 0xe0 | palette[1][0] as i32 >> 3) as u8;

    if (result[0] > result[2] || (result[0] == result[2] && result[1] >= result[3])) == alpha {
        result.copy_within(0..2, 4);
        result.copy_within(2..4, 0);
        result.copy_within(4..6, 2);

        palette[2] = palette[0].clone();
        palette[0] = palette[1].clone();
        palette[1] = palette[2].clone();
    }

    if !alpha {
        let mut bits = vec![0xffu8; 4];
        for i in 0..3 {
            bits[i] = ((((palette[0][i] as i32) << 1) + palette[1][i] as i32) / 3) as u8;
        }
        palette[2].copy_from_slice(&bits);
        bits.fill(0xff);
        for i in 0..3 {
            bits[i] = ((palette[0][i] as i32 + ((palette[1][i] as i32) << 1)) / 3) as u8;
        }
        palette[3].copy_from_slice(&bits);
    } else {
        let mut bits = vec![0xffu8; 4];
        for i in 0..3 {
            bits[i] = ((palette[0][i] as i32 + palette[1][i] as i32) >> 1) as u8;
        }
        palette[2].copy_from_slice(&bits);
        palette[3].fill(0x0);
    }
    for i in 0..(block.len() >> 4) {
        let mut num = leastdistance(&palette, block, i * 16 + 0) << 6;
        num |= leastdistance(&palette, block, i * 16 + 4) << 4;
        num |= leastdistance(&palette, block, i * 16 + 8) << 2;
        num |= leastdistance(&palette, block, i * 16 + 12);
        result[4 + i] = num as u8;
    }
    result
}