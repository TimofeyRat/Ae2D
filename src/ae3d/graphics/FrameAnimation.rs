pub struct Frame
{
	id: u8,
	duration: f32
}

pub struct Animation
{
	name: String,
	repeat: i32,
	repeated: i32,
	frames: Vec<Frame>,
	currentTime: f32,
	currentFrame: usize
}

impl Animation
{
	pub fn new() -> Self
	{
		Self
		{
			name: String::new(),
			repeat: 0,
			repeated: 0,
			frames: vec![],
			currentFrame: 0,
			currentTime: 0.0
		}
	}

	pub fn parse(base: &json::JsonValue) -> Self
	{
		let mut anim = Animation::new();

		for node in base.entries()
		{
			if node.0 == "name" { anim.name = node.1.as_str().unwrap_or("").to_string(); }
			if node.0 == "repeat" { anim.repeat = node.1.as_i32().unwrap_or(0); }
			if node.0 == "frames"
			{
				for f in node.1.members()
				{
					let mut frame = Frame { id: 0, duration: 0.0 };
					for data in f.entries()
					{
						if data.0 == "frame" { frame.id = data.1.as_u8().unwrap_or(0); }
						if data.0 == "duration" { frame.duration = data.1.as_f32().unwrap_or(0.0); }
					}
					anim.frames.push(frame);
				}
			}
		}
		
		anim
	}

	pub fn update(&mut self)
	{
		if self.repeated >= self.repeat && self.repeat != 0 { return; }

		self.currentTime += crate::ae3d::Window::Window::getDeltaTime();
		if self.currentTime >= self.frames[self.currentFrame].duration
		{
			self.currentTime -= self.frames[self.currentFrame].duration;
			self.currentFrame += 1;
		}
		if self.currentFrame > self.frames.len() - 1
		{
			if self.repeat == 0 { self.currentFrame = 0; self.currentTime = 0.0; }
			else
			{
				self.repeated += 1;
			}
		}
		self.currentFrame = self.currentFrame.clamp(0, self.frames.len() - 1);
	}

	pub fn getCurrentFrame(&mut self) -> u8 { self.frames[self.currentFrame].id }
}

pub struct Animator
{
	texture: u32,
	size: glam::IVec2,
	frame: glam::IVec2,
	animations: Vec<Animation>,
	currentAnimation: usize,
	frames: Vec<sdl2::rect::FRect>
}

impl Animator
{
	pub fn new() -> Self
	{
		Self
		{
			texture: 0,
			size: glam::IVec2::splat(0),
			frame: glam::IVec2::splat(0),
			animations: vec![],
			currentAnimation: 0,
			frames: vec![]
		}
	}

	pub fn load(&mut self, path: String)
	{
		let src = crate::ae3d::Assets::readJSON(path);
		if src.is_none() { return; }

		
		for data in src.unwrap().entries()
		{
			if data.0 == "texture"
			{
				self.texture = crate::ae3d::Assets::getTexture(
					data.1
					.as_str()
					.unwrap_or("")
					.to_string()
				);
			}
			if data.0 == "size"
			{
				for dim in data.1.entries()
				{
					if dim.0 == "x" { self.size.x = dim.1.as_i32().unwrap_or(0); }
					if dim.0 == "y" { self.size.y = dim.1.as_i32().unwrap_or(0); }
				}
			}
			if data.0 == "frame"
			{
				for dim in data.1.entries()
				{
					if dim.0 == "x" { self.frame.x = dim.1.as_i32().unwrap_or(0); }
					if dim.0 == "y" { self.frame.y = dim.1.as_i32().unwrap_or(0); }
				}

				self.calculateFrames();
			}
			if data.0 == "animations"
			{
				for anim in data.1.members()
				{
					self.animations.push(Animation::parse(anim));
				}
			}
		}
	}

	pub fn fromFile(path: String) -> Self
	{
		let mut anim = Animator::new();
		anim.load(path);
		anim
	}

	pub fn bindTexture(&mut self)
	{
		unsafe
		{
			gl::BindTexture(gl::TEXTURE_2D, self.texture);
		}
	}

	pub fn getSize(&mut self) -> glam::IVec2 { self.size }

	pub fn getFrameSize(&mut self) -> glam::IVec2 { self.frame }

	pub fn update(&mut self)
	{
		self.animations[self.currentAnimation].update();
	}

	fn calculateFrames(&mut self)
	{
		self.frames.clear();
		let mut x = 0;
		let mut y = 0;
		while y < self.size.y
		{
			while x < self.size.x
			{
				self.frames.push(sdl2::rect::FRect::new(
					x as f32 / self.size.x as f32,
					y as f32 / self.size.y as f32,
					self.frame.x as f32 / self.size.x as f32,
					self.frame.y as f32 / self.size.y as f32
				));
				x += self.frame.x;
			}
			y += self.frame.y;
			x = 0;
		}
	}

	pub fn getCurrentFrame(&mut self) -> sdl2::rect::FRect
	{
		if self.frames.len() == 0 { return sdl2::rect::FRect::new(0.0 ,0.0, 0.0, 0.0); }
		if self.animations.len() == 0 { return sdl2::rect::FRect::new(0.0, 0.0, 0.0, 0.0); }
		self.frames[self.animations[self.currentAnimation].getCurrentFrame() as usize]
	}

	pub fn setCurrentAnimation(&mut self, name: String)
	{
		for i in 0..self.animations.len()
		{
			if self.animations[i].name == name && self.currentAnimation != i
			{
				self.currentAnimation = i;
				self.restart();
				return;
			}
		}
	}

	fn restart(&mut self)
	{
		for i in 0..self.animations.len()
		{
			self.animations[i].currentTime = 0.0;
			self.animations[i].currentFrame = 0;
		}
	}
}