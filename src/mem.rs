use std::ops::{Index, IndexMut};
use std::borrow::BorrowMut;

pub struct Mem(Vec<i8>);

impl Mem {
    pub fn new(size: usize) -> Self {
        Self (
            vec![0; size],
        )
    }
}

impl Index<usize> for Mem {
    type Output = i8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a> IndexMut<usize> for Mem {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0[index].borrow_mut()
    }
}