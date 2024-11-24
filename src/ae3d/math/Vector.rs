use super::Point::Point;

#[derive(Clone, Copy)]

pub struct Vector 
{ 
	pub length: f32, 
	pub angle: f32
}

impl Vector
{
	pub fn new(&mut self, length: f32, angle: f32) -> Vector 
	{
		Vector {angle, length}
	}

	pub fn zero() -> Self
	{
        Self { length: 0.0, angle: 0.0 }
    }

	pub fn toPoint(self) -> Point
	{
		let a = f32::to_radians(self.angle);
		Point
		{
			x: f32::cos(a) * self.length,
			y: f32::sin(a) * self.length
		}
	}

	pub fn normalize(self) -> Vector 
	{
		Vector {angle: self.angle, length: 1.0}
	}
}