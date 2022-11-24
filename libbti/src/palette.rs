use std::io::Read;

#[derive(Debug, Clone, Default)]
pub struct Palette {
    pub palettedata: Vec<u8>
}

impl Palette {
    pub fn read<R: Read, N: Into<usize> + Copy>(reader: &mut R, count: N) -> Self {
        let mut res = Palette::default();
        res.palettedata = vec![0u8; count.into()];
        reader.read_exact(&mut res.palettedata).unwrap();
        res
    }
}