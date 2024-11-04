#[derive(Clone, Copy)]
pub struct Point { 
	pub x: f64, 
	pub y: f64 
}

impl Point
{
	pub fn zero() -> Point {
		Point { x: 0.0, y: 0.0 }
	}
	pub fn num(x: f64) -> Point { 
		Point { x, y: x}
	 }
	pub fn add(&mut self, x: Point) -> &mut Point { 
		self.x += x.x; self.y += x.y; self 
	}
	pub fn subtract(&mut self, x: Point) -> &mut Point { 
		self.x -= x.x; self.y -= x.y; self 
	}
	pub fn multiply(&mut self, x: Point) -> &mut Point { 
		self.x *= x.x; self.y *= x.y; self 
	}
	pub fn divide(&mut self, x: Point) -> &mut Point { 
		self.x /= x.x; self.y /= x.y; self
	}
}