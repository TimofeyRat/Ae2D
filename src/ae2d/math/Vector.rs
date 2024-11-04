use super::Point::Point;

#[derive(Clone, Copy)]

pub struct Vector { length: f64, angle: f64 }

impl Vector
{
	pub fn zero() -> Self
	{
        Self { length: 0.0, angle: 0.0 }
    }
	pub fn to_point(&mut self) -> Point
	{
		let a = f64::to_radians(self.angle);
		Point
		{
			x: f64::cos(a) * self.length,
			y: f64::sin(a) * self.length
		}
	}
}