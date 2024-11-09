use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Point { 
	pub x: f64, 
	pub y: f64 
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

impl Point
{
	pub fn zero() -> Self
	{
		Self { x: 0.0, y: 0.0 }
	}
	pub fn num(x: f64) -> Self
	{
		Self { x, y: x }
	}
}