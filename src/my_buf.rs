use std::ops::Add;

#[derive(Debug)]
pub struct Buffer<T> {
    pub content: Vec<T>,
}

impl<T> Buffer<T>
where
    T: Add<Output = T> + Default + Copy, // So that it has a zero unit, and addable
{
    pub fn sum(&self) -> T {
        self.content.iter().fold(T::default(), |sum, x| sum + *x)
    }
}
