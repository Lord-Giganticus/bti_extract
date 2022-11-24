use binrw::prelude::*;
use std::default::Default;

#[derive(BinRead, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[br(repr = u8)]
#[repr(u8)]
pub enum TextureFormats {
    #[default] I4 = 0x00,
    I8 = 0x01,
    IA4 = 0x02,
    IA8 = 0x03,
    RGB565 = 0x04,
    RGB5A3 = 0x05,
    RGBA32 = 0x06,
    C4 = 0x08,
    C8 = 0x09,
    C14X2 = 0x0a,
    CMPR = 0x0e, 
}

impl<N: Into<u8>> From<N> for TextureFormats {
    fn from(n: N) -> Self {
        let u: u8 = n.into();
        use TextureFormats::*;
        let items = 
        vec![I4, I8, IA4, IA8, RGB565, RGB5A3, RGBA32, C4, C8, C14X2, CMPR];
        let nums = items.iter().map(|x| *x as u8).collect::<Vec<u8>>();
        let pos = nums.iter().position(|x| *x == u).unwrap();
        items[pos]
    }
}

#[derive(BinRead, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[br(repr = u8)]
#[repr(u8)]
pub enum WrapNodes {
    #[default] ClampToEdge = 0,
    Repeat = 1,
    MirroredRepeat = 2,
}

impl<N: Into<u8>> From<N> for WrapNodes {
    fn from(n: N) -> Self {
        let u: u8 = n.into();
        use WrapNodes::*;
        let items = vec![ClampToEdge, Repeat, MirroredRepeat];
        let nums = items.iter().map(|x| *x as u8).collect::<Vec<u8>>();
        let pos = nums.iter().position(|x| *x == u).unwrap();
        items[pos]
    }
}

#[derive(BinRead, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[br(repr = u8)]
#[repr(u8)]
pub enum PaletteFormats {
    #[default] IA8 = 0x00,
    RGB565 = 0x01,
    RGB5A3 = 0x02,
}

impl<N: Into<u8>> From<N> for PaletteFormats {
    fn from(n: N) -> Self {
        let u: u8 = n.into();
        use PaletteFormats::*;
        let items = vec![IA8, RGB565, RGB5A3];
        let nums = items.iter().map(|x| *x as u8).collect::<Vec<u8>>();
        let pos = nums.iter().position(|x| *x == u).unwrap();
        items[pos]
    }
}

#[derive(BinRead, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[br(repr = u8)]
#[repr(u8)]
pub enum FilterMode {
    #[default] Nearest = 0x0,
    Linear = 0x1,
    NearestMipmapNearest = 0x2,
    NearestMipmapLinear = 0x3,
    LinearMipmapNearest = 0x4,
    LinearMipmapLinear = 0x5,
}

impl<N: Into<u8>> From<N> for FilterMode {
    fn from(n: N) -> Self {
        let u = n.into();
        use FilterMode::*;
        let items = vec![Nearest, Linear, NearestMipmapNearest, NearestMipmapLinear,
        LinearMipmapNearest, LinearMipmapLinear];
        let nums = items.iter().map(|x| *x as u8).collect::<Vec<u8>>();
        let pos = nums.iter().position(|x| *x == u).unwrap();
        items[pos]
    }
}