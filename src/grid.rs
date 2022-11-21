use std::{ops, fmt::Display};

pub struct Grid<T: Clone> {
    width: usize,
    height: usize,

    buf: Vec<T>
}

impl<T: Clone> Grid<T> {
    pub fn new(width: u32, height: u32, default: T) -> Grid<T> {
        Grid {
            width: width as usize,
            height: height as usize,

            buf: vec![default; width as usize * height as usize]
        }
    }
}

impl<T: Clone + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[(x as u32, y as u32)])?;
            }
            writeln!(f, "")?;
        }
        
        Ok(())
    }
}

impl<T: Clone> ops::Index<(u32, u32)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        &self.buf[index.1 as usize * self.width + index.0 as usize]
    }
}

impl<T: Clone> ops::IndexMut<(u32, u32)> for Grid<T> {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
        &mut self.buf[index.1 as usize * self.width + index.0 as usize]
    }
}