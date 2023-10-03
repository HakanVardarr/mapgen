use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Debug, Hash, Copy, Clone)]
pub struct Vector2D<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Debug
        + PartialEq
        + Copy,
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Debug
        + PartialEq
        + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Add for Vector2D<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Debug
        + PartialEq
        + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign for Vector2D<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Debug
        + PartialEq
        + Copy,
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Vector2D<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Debug
        + PartialEq
        + Copy,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> SubAssign for Vector2D<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Debug
        + PartialEq
        + Copy,
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Mul for Vector2D<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Debug
        + PartialEq
        + Copy,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> MulAssign for Vector2D<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Debug
        + PartialEq
        + Copy,
{
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> Div for Vector2D<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Debug
        + PartialEq
        + Copy,
{
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T> DivAssign for Vector2D<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Debug
        + PartialEq
        + Copy,
{
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T> PartialEq for Vector2D<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Debug
        + PartialEq
        + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Eq for Vector2D<T> where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Debug
        + PartialEq
        + Copy
{
}
