use rand::{distributions::Standard, prelude::Distribution, Rng};

use vec::Vector2D;

pub enum Direction2D {
    Up,
    Right,
    Down,
    Left,
}

impl Direction2D {
    pub fn value(&self) -> Vector2D<i32> {
        match *self {
            Direction2D::Up => Vector2D::new(0, -1),
            Direction2D::Right => Vector2D::new(1, 0),
            Direction2D::Down => Vector2D::new(0, 1),
            Direction2D::Left => Vector2D::new(-1, 0),
        }
    }
}

impl Distribution<Direction2D> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction2D {
        match rng.gen_range(0..=3) {
            0 => Direction2D::Up,
            1 => Direction2D::Right,
            2 => Direction2D::Down,
            _ => Direction2D::Left,
        }
    }
}
