pub struct Glyph
{
	code: char,
	rect: sdl2::rect::Rect
}

pub struct Font
{
	texture: u32,
	glyphs: Vec<Glyph>
}

impl Font
{
	pub fn new() -> Self
	{
		Self
		{
			texture: 0,
			glyphs: vec![]
		}
	}

	pub fn load(&mut self, texture: String, info: String)
	{
		self.texture = crate::ae3d::Assets::loadTexture(texture);
	}
}