use crate::direction::Direction2D;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use vec::Vector2D;
mod direction;

fn simple_random_walk(starting_pos: Vector2D<i32>, walk_length: i32) -> HashSet<Vector2D<i32>> {
    let mut path: HashSet<Vector2D<i32>> = HashSet::new();

    path.insert(starting_pos);
    let mut previous_pos = starting_pos;

    for _ in 0..walk_length {
        let new_pos = previous_pos + rand::random::<Direction2D>().value();
        path.insert(new_pos);
        previous_pos = new_pos;
    }

    path
}

pub struct Map {
    height: usize,
    width: usize,
    pub tiles: Vec<Vec<i32>>,
}

impl Map {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            tiles: vec![vec![0; width]; height],
        }
    }

    fn add_floor(&mut self, floor: &HashSet<Vector2D<i32>>) {
        for pos in floor.iter() {
            if pos.x >= 0
                && pos.x <= self.width as i32 - 1
                && pos.y >= 0
                && pos.y <= self.height as i32 - 1
            {
                self.tiles[pos.y as usize][pos.x as usize] = 1;
            }
        }
    }

    pub fn run_procedural_genartion(
        &mut self,
        start_pos: Vector2D<i32>,
        iterations: i32,
        walk_length: i32,
        start_random: bool,
    ) {
        let floor_pos =
            Self::run_random_walk(&self, start_pos, iterations, walk_length, start_random);
        self.add_floor(&floor_pos);
    }
    fn run_random_walk(
        &self,
        start_pos: Vector2D<i32>,
        iterations: i32,
        walk_length: i32,
        start_random: bool,
    ) -> HashSet<Vector2D<i32>> {
        let mut current_pos = start_pos;
        let mut floor_pos: HashSet<Vector2D<i32>> = HashSet::new();
        for _ in 0..iterations {
            let path = simple_random_walk(current_pos, walk_length);

            floor_pos = floor_pos.union(&path).copied().collect();
            if start_random {
                let index = thread_rng().gen_range(0..floor_pos.len());
                current_pos = *floor_pos.iter().nth(index).unwrap();
            }
        }

        floor_pos
    }
}
