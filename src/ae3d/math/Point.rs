use std::ops::{Add, Div, Mul, Sub, AddAssign, DivAssign, MulAssign, SubAssign};

#[derive(Clone, Copy)]
pub struct Point { 
	pub x: f32, 
	pub y: f32 
}

impl Add for Point
{
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output
	{
		Point { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}

impl Sub for Point
{
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output
	{
		Point { x: self.x - rhs.x, y: self.y - rhs.y }
	}
}

impl Mul for Point
{
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output
	{
		Point { x: self.x * rhs.x, y: self.y * rhs.y }
	}
}

impl Div for Point
{
	type Output = Self;
	fn div(self, rhs: Self) -> Self::Output
	{
		Point { x: self.x / rhs.x, y: self.y / rhs.y }
	}
}

impl AddAssign for Point
{
	fn add_assign(&mut self, rhs: Self)
	{
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl SubAssign for Point
{
	fn sub_assign(&mut self, rhs: Self)
	{
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl MulAssign for Point
{
	fn mul_assign(&mut self, rhs: Self)
	{
		self.x *= rhs.x;
		self.y *= rhs.y;
	}
}

impl DivAssign for Point
{
	fn div_assign(&mut self, rhs: Self)
	{
		self.x /= rhs.x;
		self.y /= rhs.y;
	}
}

impl PartialEq for Point
{
	fn eq(&self, other: &Self) -> bool
	{
		self.x == other.x && self.y == other.y
	}

	fn ne(&self, other: &Self) -> bool
	{
		self.x != other.x || self.y != other.y
	}
}

impl Point
{
	pub fn zero() -> Self
	{
		Self { x: 0.0, y: 0.0 }
	}
	pub fn num(x: f32) -> Self
	{
		Self { x, y: x }
	}
}