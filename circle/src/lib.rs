#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center : Point,
	pub radius : f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point (x, y),
            radius: radius,
        }
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    pub fn intersect(&self, circle: &Circle) -> bool {
        let dis = self.center.distance(circle.center);
        dis <= self.radius + circle.radius
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point (pub f64, pub f64);

impl Point {
    pub fn distance(&self, point_b: Point) -> f64 {
        let dx = self.0 - point_b.0;
        let dy = self.1 - point_b.1;
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let circle = Circle::new(500.0, 500.0, 150.0);
        let circle1 = Circle {
            center: Point(80.0, 115.0),
            radius: 30.0,
        };

        let point_a = Point(1.0, 1.0);
        let point_b = Point(0.0, 0.0);
        
        println!("circle = {:?} area = {}", circle, circle.area());
        println!("circle = {:?} diameter = {}", circle, circle.diameter());
        println!("circle1 = {:?} diameter = {}", circle1, circle1.diameter());
        
        println!(
            "circle and circle1 intersect = {}",
            circle.intersect(&circle1)
        );

        println!(
            "distance between {:?} and {:?} is {}",
            point_a,
            point_b,
            point_a.distance(point_b)
        );
    }
}
