use sdl2::image::LoadTexture;

use crate::ae2d::{Assets, Window::Window, math::Point::Point};

pub struct Animation<'a>
{
	pub texture: Option<sdl2::render::Texture<'a>>,
	pub frames: Vec<sdl2::rect::Rect>
}

impl<'a> Animation<'a>
{
	pub fn new() -> Self
	{
		Self
		{
			texture: None,
			frames: vec![]
		}
	}

	pub fn loadFromFile(&mut self, path: String)
	{
		let code = Assets::readFile(path.clone());
		if code.is_none() { println!("Failed to open file {}", path.clone()); return }
		
		let parsedRes = json::parse(code.unwrap().as_str());
		if parsedRes.is_err() { println!("Failed to parse json from {}: {}", path, parsedRes.err().unwrap()); return }

		let parsed = parsedRes.unwrap();
		for element in parsed.entries()
		{
			if element.0 == "texture"
			{
				let path = element.1.as_str().unwrap();
				let full = Assets::getCurrentDir() + path;
				self.texture = Some(
					Window::getTC()
					.load_texture(full)
					.expect((String::from("Failed  to load texture ") + path).as_str())
				);
			}
			else if element.0 == "size"
			{
				let mut size = Point::zero();
				for dim in element.1.entries()
				{
					if dim.0 == "x" { size.x = dim.1.as_f64().unwrap(); }
					if dim.1 == "y" { size.y = dim.1.as_f64().unwrap(); }
				}
				self.calculateFrames(size);
			}
			else { println!("{}: {}", element.0, element.1); }
		}
	}

	fn calculateFrames(&mut self, size: Point)
	{
		self.frames.clear();
		// TODO
	}
}