struct Point {
    x: f64,
    y: f64,
}

impl Point {
    const fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    // Было бы удобно засунуть эти вычисления в compiletime, но стабильная версия
    // ещё не поддерживает операции над дробными числами в данном контексте :(
    // Ждём https://github.com/rust-lang/rust/issues/57241
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    let point1 = Point::new(3.0, 4.0);
    let point2 = Point::new(1.0, 2.0);

    let distance = point1.distance(&point2);

    println!("Расстояние между точками: {}", distance);
}