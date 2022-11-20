use std::ops;

pub struct Grid<T: Default + Clone> {
    width: usize,
    height: usize,

    buf: Vec<T>
}

impl<T: Default + Clone> Grid<T> {
    pub fn new(width: u32, height: u32) -> Grid<T> {
        Grid {
            width: width as usize,
            height: height as usize,

            buf: vec![T::default(); width as usize * height as usize]
        }
    }
}

impl<T: Default + Clone> ops::Index<(u32, u32)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        &self.buf[index.0 as usize * self.width + index.1 as usize]
    }
}

impl<T: Default + Clone> ops::IndexMut<(u32, u32)> for Grid<T> {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
        &mut self.buf[index.0 as usize * self.width + index.1 as usize]
    }
}