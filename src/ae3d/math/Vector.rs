use super::Point::Point;

#[derive(Clone, Copy)]

pub struct Vector 
{ 
	pub length: f64, 
	pub angle: f64
}

impl Vector
{
	pub fn new(&mut self, length: f64, angle: f64) -> Vector 
	{
		Vector {angle, length}
	}

	pub fn zero() -> Self
	{
        Self { length: 0.0, angle: 0.0 }
    }

	pub fn toPoint(self) -> Point
	{
		let a = f64::to_radians(self.angle);
		Point
		{
			x: f64::cos(a) * self.length,
			y: f64::sin(a) * self.length
		}
	}

	pub fn normalize(self) -> Vector 
	{
		Vector {angle: self.angle, length: 1.0}
	}
}