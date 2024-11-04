use super::Window::Window;

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
		let mut decode= gif::DecodeOptions::new();
		decode.set_color_output(gif::ColorOutput::RGBA);
		let fileResult = std::fs::File::open(sdl2::filesystem::base_path().unwrap() + path.as_str());
		if fileResult.is_err() { println!("Failed to load animation {}", path.as_str()); return; }
		let file = fileResult.unwrap();
		let mut decoder = decode.read_info(file).unwrap();
		while let Some(frame) = decoder.read_next_frame().unwrap()
		{
			let mut result = sdl2::surface::Surface::new(
				frame.width as u32,
				frame.height as u32,
				sdl2::pixels::PixelFormatEnum::RGBA8888
			);
			if result.is_err() { return; }
			let surface = result.as_mut().unwrap();
			for x in 0..frame.width
			{
				for y in 0..frame.height
				{
					let index = (y as f64 * frame.width as f64 + x as f64) as usize;
					surface.fill_rect(
						sdl2::rect::Rect::new(x as i32, y as i32, 1, 1),
						sdl2::pixels::Color::RGBA(
							frame.buffer[index],
							frame.buffer[index + 1],
							frame.buffer[index + 2],
							frame.buffer[index + 3]
						)
					);
				}
			}
			self.frames.push(surface.as_texture(Window::getTC()).unwrap());
		}
	}
}