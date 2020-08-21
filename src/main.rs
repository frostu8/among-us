use among_us::collide::{Circle, Polygon, Geometry as _};
use among_us::math::*;

fn main() {
    let polygon: Polygon = [Vector2::new(0.0, 2.0), Vector2::new(0.0, -2.0), Vector2::new(-4.0, -2.0), Vector2::new(-4.0, 2.0)].iter().map(|v| v + Vector2::new(10.0, 10.0)).collect::<Vec<Vector2>>().into();

    if polygon.collide(&Circle::new(Vector2::new(15.0, 10.0), 5.000001)) {
        println!("collision");
    } else {
        println!("no collision");
    }
}
