use euclid::Vector2D;
use mapgen::Map;
use mapgen::SimpleRandomWalkDungeonGenerator;

fn main() {
    let srwdg = SimpleRandomWalkDungeonGenerator::new();
    let mut map = Map::new(43, 160);
    srwdg.run_procedural_genartion(&mut map, Vector2D::new(75, 20), 100, 20, false);
    srwdg.run_procedural_genartion(&mut map, Vector2D::new(20, 25), 100, 10, false);
    
}
