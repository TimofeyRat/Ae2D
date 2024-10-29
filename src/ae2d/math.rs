use core::f64;

pub struct Point { pub x: f64, pub y: f64 }

pub struct Vector { length: f64, angle: f64 }

pub fn new_point() {}

impl Point
{
	pub fn default() -> Point { Point { x: 0.0, y: 0.0 } }
	pub fn new(x: f64, y: f64) -> Point { Point { x: x, y: y } }
	pub fn from_num(x: f64) -> Point { Point { x: x, y: x} }
	pub fn add(&mut self, x: Point) -> &mut Point { self.x += x.x; self.y += x.y; self }
	pub fn subtract(&mut self, x: Point) -> &mut Point { self.x -= x.x; self.y -= x.y; self }
	pub fn multiply(&mut self, x: Point) -> &mut Point { self.x *= x.x; self.y *= x.y; self }
	pub fn divide(&mut self, x: Point) -> &mut Point { self.x /= x.x; self.y /= x.y; self }
}

impl Vector
{
	pub fn default() -> Vector { Vector { length: 0.0, angle: 0.0} }
	pub fn new(length: f64, angle: f64) -> Vector { Vector { length: length, angle: angle} }
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