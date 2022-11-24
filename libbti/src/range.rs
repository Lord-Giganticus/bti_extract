use std::ops::*;


#[derive(Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NRange<N: AddAssign + Add + Copy + Ord + Add<Output = N>> {
    pub start: N,
    pub end: N,
    pub inc: N
}

impl<N: AddAssign + Add + Copy + Ord + Add<Output = N>> Iterator for NRange<N> {
    type Item = N;
    fn next(&mut self) -> Option<Self::Item> {
        let calc = self.start + self.inc;
        if calc > self.end {
            None
        } else {
            self.start = calc;
            Some(calc)
        }
    }
}

impl<N: AddAssign + Add + Copy + Ord + Add<Output = N>> NRange<N> {
    pub fn new(start: N, end: N, inc: N) -> Self {
        Self { start, end, inc }
    }
}