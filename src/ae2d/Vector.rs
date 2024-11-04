use super::Point;

#[derive(Clone, Copy)]

pub struct Vector { length: f64, angle: f64 }

impl Vector
{
	pub fn zero() -> Vector { 
        Self { length: 0.0, angle: 0.0 } 
    }
	pub fn to_point(&mut self) -> Point::Point
	{
		let a = f64::to_radians(self.angle);
		Point::Point
		{
			x: f64::cos(a) * self.length,
			y: f64::sin(a) * self.length
		}
	}
}