#[derive(Clone, Copy)]
pub struct Color
{
	r: u8,
	g: u8,
	b: u8,
	a: u8,
}

impl Color
{
	pub fn toGL(&mut self) -> (f32, f32, f32, f32)
	{
		(
			(self.r as f32) / 255.0,
			(self.g as f32) / 255.0,
			(self.b as f32) / 255.0,
			(self.a as f32) / 255.0
		)
	}
	pub fn rgb(r: u8, g: u8, b: u8) -> Color { Color { r, g, b, a: 255 } }
	pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color { Color { r, g, b, a } }
	pub const Black: Color = Color { r: 0, g: 0, b: 0, a: 255 };
	pub const White: Color = Color { r: 255, g: 255, b: 255, a: 255 };
	pub const Red: Color = Color { r: 255, g: 0, b: 0, a: 255 };
	pub const Green: Color = Color { r: 0, g: 255, b: 0, a: 255 };
	pub const Blue: Color = Color { r: 0, g: 0, b: 255, a: 255 };
	pub const Transparent: Color = Color { r: 0, g: 0, b: 0, a: 0 };
}