use std::cell::Cell;

pub struct CellIter<I>(Cell<Option<I>>);

impl<I> CellIter<I> {
    pub fn new(iter: I) -> Self {
        Self(Cell::new(Some(iter)))
    }
}

impl<I: Iterator> From<I> for CellIter<I> {
    fn from(iter: I) -> Self {
        Self::new(iter)
    }
}

impl<I: Iterator> Iterator for &CellIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iter = self.0.take()?;
        let next = iter.next()?;

        self.0.set(Some(iter));
        Some(next)
    }
}

macro_rules! fa {
    ($name:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/fa/", $name, ".svg"))
    };
}

pub(crate) use fa;
