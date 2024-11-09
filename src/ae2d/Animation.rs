use sdl2::image::LoadTexture;

use crate::ae2d::{Assets, Window::Window, math::Point::Point};

pub struct Frame
{
	pub id: usize,
	pub duration: f64
}

pub struct Animation
{
	pub name: String,
	pub frames: Vec<Frame>,
	pub currentTime: f64,
	pub currentFrame: usize,
	pub repeat: i32,
	pub repeated: i32
}

impl Animation
{
	pub fn update(&mut self)
	{
		if self.currentFrame >= self.frames.len()
		{
			if self.repeat == 0 || self.repeated < self.repeat - 1
			{
				self.currentFrame = 0;
				self.currentTime = 0.0;
				if self.repeat != 0 { self.repeated += 1; }
			}
			else
			{
				self.currentFrame = self.frames.len() - 1;
				return
			}
		}
		self.currentTime += Window::getDeltaTime();
		if self.currentTime >= self.frames[self.currentFrame].duration
		{
			self.currentFrame += 1;
			self.currentTime = 0.0;
		}
	}

	pub fn getCurrentFrame(&mut self) -> &Frame { &self.frames[self.currentFrame.clamp(0, self.frames.len() - 1)] }
}

pub struct Animator<'a>
{
	pub texture: Option<sdl2::render::Texture<'a>>,
	frames: Vec<sdl2::rect::Rect>,
	anims: Vec<Animation>,
	currentAnimation: usize,
}

impl<'a> Animator<'a>
{
	pub fn new() -> Self
	{
		Self
		{
			texture: None,
			frames: vec![],
			anims: vec![],
			currentAnimation: 0,
		}
	}

	pub fn loadFromFile(&mut self, path: String)
	{
		let code = Assets::readFile(path.clone());
		if code.is_none()
		{
			println!("Failed to open file {}", path.clone());
			return;
		}
		
		let parsedRes = json::parse(code.unwrap().as_str());
		if parsedRes.is_err()
		{
			println!("Failed to parse json from {}: {}", path, parsedRes.err().unwrap());
			return;
		}

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
					if dim.0 == "y" { size.y = dim.1.as_f64().unwrap(); }
				}
				self.calculateFrames(size);
			}
			else if element.0 == "animations"
			{
				for anim in element.1.entries()
				{
					let name = String::from(anim.0);
					let mut frames: Vec<Frame> = vec![];
					let mut repeat = 0;

					for attr in anim.1.entries()
					{
						if attr.0 == "repeat" { repeat = attr.1.as_i32().unwrap(); }
						if attr.0 == "frames"
						{
							for frame in attr.1.members()
							{
								let mut f = Frame { id: 0, duration: 0.0 };
								for args in frame.entries()
								{
									if args.0 == "frame" { f.id = args.1.as_usize().unwrap(); }
									if args.0 == "duration" { f.duration = args.1.as_f64().unwrap(); }
								}
								frames.push(f);
							}
						}
					}
					
					self.anims.push(Animation { name, frames, currentFrame: 0, currentTime: 0.0, repeat, repeated: 0 });
				}
			}
			else { println!("{}: {}", element.0, element.1); }
		}
	}

	fn calculateFrames(&mut self, size: Point)
	{
		if size.x == 0.0 || size.y == 0.0
		{
			println!("Invalid frame size! {}x{}", size.x, size.y);
			return;
		}

		self.frames.clear();
		let texSize = Point {
			x: self.texture.as_mut().unwrap().query().width as f64,
			y: self.texture.as_mut().unwrap().query().height as f64
		};

		if (texSize.x as i32 % size.x as i32 != 0) ||
			texSize.y as i32 % size.y as i32 != 0
		{
			println!("Size of frame isn't compatible with the size of the texture.");
			return;
		}

		let mut y = 0;
		while y < texSize.y as i32
		{
			let mut x = 0;
			while x < texSize.x as i32
			{
				self.frames.push(sdl2::rect::Rect::new(
					x, y,
					size.x as u32,
					size.y as u32
				));
				x += size.x as i32;
			}
			y += size.y as i32;
		}
	}
	
	pub fn getCurrentAnimation(&mut self) -> &mut Animation { &mut self.anims[self.currentAnimation] }
	pub fn getFrame(&mut self, id: usize) -> sdl2::rect::Rect { self.frames[id] }
	pub fn getCurrentFrame(&mut self) -> sdl2::rect::Rect
	{
		let id = self.getCurrentAnimation().getCurrentFrame().id;
		self.frames[id]
	}
}