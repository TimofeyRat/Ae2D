use crate::ae2d::Assets;

pub struct Animation<'a>
{
	pub frames: Vec<sdl2::render::Texture<'a>>
}

impl<'a> Animation<'a>
{
	pub fn new() -> Self
	{
		Self
		{
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
		
		println!("{:?}", parsed);
	}
}