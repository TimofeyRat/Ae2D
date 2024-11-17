use super::Point::Point;

#[derive(Clone, Copy)]

pub struct Vector 
{ 
	length: f64, 
	angle: f64 
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

	pub fn to_point(&mut self) -> Point
	{
		let a = f64::to_radians(self.angle);
		Point
		{
			x: f64::cos(a) * self.length,
			y: f64::sin(a) * self.length
		}
	}

	pub fn normalize(&mut self) -> Vector 
	{
		Vector {angle: self.angle, length: 1.0}
	}

	pub fn set_angle(&mut self, angle: f64) 
	{
		self.angle = angle;
	}

	pub fn set_length(&mut self, length: f64) 
	{
		self.length = length;
	}
}