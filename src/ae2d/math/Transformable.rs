use super::{Point::Point, Vector::Vector};

pub struct Transformable
{
	pos: Point,
	scale: Point,
	origin: Point,
	angle: f64
}

impl Transformable
{
	pub fn new() -> Self
	{
		Self
		{
			angle: 0.0,
			pos: Point::zero(),
			scale: Point::num(1.0),
			origin: Point::zero()
		}
	}

	pub fn setPosition(&mut self, pos: Point)
	{
		self.pos = pos;
	}

	pub fn movePoint(&mut self, delta: Point)
	{
		self.pos += delta;
	}

	pub fn moveVector(&mut self, delta: Vector)
	{
		self.pos += delta.toPoint();
	}

	pub fn getPosition(&mut self) -> Point
	{
		self.pos
	}

	pub fn setScale(&mut self, factor: Point)
	{
		self.scale = factor;
	}

	pub fn scale(&mut self, factor: Point)
	{
		self.scale += factor;
	}

	pub fn getScale(&mut self) -> Point
	{
		self.scale
	}

	pub fn setOrigin(&mut self, origin: Point)
	{
		self.origin = origin;
	}

	pub fn getOrigin(&mut self) -> Point
	{
		self.origin
	}

	pub fn setRotation(&mut self, angle: f64)
	{
		self.angle = angle;
	}

	pub fn rotate(&mut self, angle: f64)
	{
		self.angle += angle;
	}

	pub fn getRotation(&mut self) -> f64
	{
		self.angle
	}
}