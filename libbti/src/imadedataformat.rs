#[derive(Debug, Default, Clone)]
pub struct ImageDataFormat {
    pub name: &'static str,
    pub desc: &'static str,
    pub bitsperpixel: i32,
    pub alphadepth: i32,
    pub blockwidth: i32,
    pub blockheight: i32,
    pub blockstride: i32,
    pub hascolor: bool,
    pub iscompressed: bool,
    pub lossy: bool,
    pub palette: bool,
    pub palettesize: i32,
    pub palettebitsperentry: i32,
}

impl ImageDataFormat {
    pub const fn new(name: &'static str, desc: &'static str, bitsperpixel: i32, alphadepth: i32,
    blockwidth: i32, blockheight: i32, blockstride: i32, hascolor: bool, iscompressed: bool,
    lossy: bool, palette: bool, palettesize: i32, palettebitsperentry: i32) -> Self {
        Self {
            name, desc, bitsperpixel, alphadepth, blockwidth, blockheight, blockstride, hascolor,
            iscompressed, lossy, palette, palettesize, palettebitsperentry
        }
    }
    pub fn roundwidth(&self, width: i32) -> i32 {
        width + ((self.blockwidth - (width % self.blockwidth)) % self.blockwidth)
    }
    pub fn roundheight(&self, height: i32) -> i32 {
        height + ((self.blockheight - (height % self.blockheight)) % self.blockheight)
    }
}

pub trait Converter {
    const FORMAT: ImageDataFormat;
    const TO: fn(&Vec<u8>) -> Vec<u8>;
    fn convertto(data: &Vec<u8>, width: i32, height: i32) -> Vec<u8> {
        let ImageDataFormat {
            blockheight, blockwidth, blockstride, ..
        } = Self::FORMAT;
        let ressize = Self::FORMAT.roundwidth(width) /
        blockwidth * Self::FORMAT.roundheight(height) / blockheight * blockstride;
        let mut result = vec![0u8; ressize as usize];
        let mut block = vec![0u8; (blockwidth * blockheight << 2) as usize];
        let mut i = 0;
        for y in (0..height).step_by(blockheight as usize) {
            let y = y as usize;
            for x in (0..width).step_by(blockwidth as usize) {
                let x = x as usize;
                block.fill(0x0);
                let blockheight = blockheight as usize;
                let height = height as usize;
                let blockwidth = blockwidth as usize;
                let width = width as usize;
                for dy in 0..(blockheight.min(height)) {
                    let sidx = ((y + dy) * width + x) << 2;
                    let didx = dy * blockwidth << 2;
                    let len = blockwidth.min(width - x) << 2;
                    for j in 0..len {
                        block[didx+j] = data[sidx+j];
                    }
                }
                let blockresult = Self::TO(&block);
                let sidx = (i * blockstride) as usize;
                for j in 0..blockresult.len() {
                    result[sidx+j] = blockresult[j];
                }
            }
            i += 1;
        }
        result
    }

}