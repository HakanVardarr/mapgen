use vec::Vector2D;

fn main() {
    let mut v1 = Vector2D::new(1.0, 2.0);
    v1 += Vector2D::new(1.0, 2.0);
    println!("{:?}", v1);
}
