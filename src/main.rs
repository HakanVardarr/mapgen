use mapgen::Map;
use vec::Vector2D;

fn main() {
    let mut map = Map::new(43, 160);
    map.run_procedural_genartion(Vector2D::new(75, 20), 100, 20, false);
    map.run_procedural_genartion(Vector2D::new(20, 25), 100, 10, false);
    println!("{:?}", map.tiles);
}
