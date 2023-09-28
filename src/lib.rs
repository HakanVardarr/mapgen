use euclid::Vector2D;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::collections::HashSet;

pub struct ProcedualGenerationAlgorithms;

impl ProcedualGenerationAlgorithms {
    pub fn simple_random_walk(
        starting_pos: Vector2D<i32, i32>,
        walk_length: i32,
    ) -> HashSet<Vector2D<i32, i32>> {
        let mut path: HashSet<Vector2D<i32, i32>> = HashSet::new();

        path.insert(starting_pos);
        let mut previous_pos = starting_pos;

        for _ in 0..walk_length {
            let new_pos = previous_pos + Direction2D::get_random_cardinal_direction();
            path.insert(new_pos);
            previous_pos = new_pos;
        }

        path
    }
}

struct Direction2D;

impl Direction2D {
    fn cardinal_directions_list() -> Vec<Vector2D<i32, i32>> {
        let mut vec = Vec::with_capacity(4);
        vec.push(Vector2D::new(0, 1));
        vec.push(Vector2D::new(1, 0));
        vec.push(Vector2D::new(0, -1));
        vec.push(Vector2D::new(-1, 0));
        vec
    }
    pub fn get_random_cardinal_direction() -> Vector2D<i32, i32> {
        *Self::cardinal_directions_list()
            .choose(&mut thread_rng())
            .unwrap()
    }
}

pub struct SimpleRandomWalkDungeonGenerator;

impl SimpleRandomWalkDungeonGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn run_procedural_genartion(
        &self,
        map: &mut Map,
        start_pos: Vector2D<i32, i32>,
        iterations: i32,
        walk_length: i32,
        start_random: bool,
    ) {
        let floor_pos =
            Self::run_random_walk(&self, start_pos, iterations, walk_length, start_random);
        map.add_floor(&floor_pos);
    }
    fn run_random_walk(
        &self,
        start_pos: Vector2D<i32, i32>,
        iterations: i32,
        walk_length: i32,
        start_random: bool,
    ) -> HashSet<Vector2D<i32, i32>> {
        let mut current_pos = start_pos;
        let mut floor_pos: HashSet<Vector2D<i32, i32>> = HashSet::new();
        for _ in 0..iterations {
            let path = ProcedualGenerationAlgorithms::simple_random_walk(current_pos, walk_length);

            floor_pos = floor_pos.union(&path).copied().collect();
            if start_random {
                let index = thread_rng().gen_range(0..floor_pos.len());
                current_pos = *floor_pos.iter().nth(index).unwrap();
            }
        }

        floor_pos
    }
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

    fn add_floor(&mut self, floor: &HashSet<Vector2D<i32, i32>>) {
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
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{esc}[2J{esc}[1;1H", esc = 27 as char)?;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.tiles[y][x] == 1 {
                    write!(f, "\x1b[0;45m \x1b[0m")?;
                } else {
                    write!(f, "\x1b[0;40m \x1b[0m")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}
