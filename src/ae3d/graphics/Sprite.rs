use sdl2::image::LoadTexture;

use crate::ae3d::{Assets, math::Point::Point, Animation::*, Window::Window};

pub struct Sprite<'a>
{
	tex: Option<sdl2::render::Texture<'a>>,
	texRect: sdl2::rect::Rect,
	scale: Point,
	position: Point,
	rotation: f32,
	absolute_origin: Option<Point>,
	relative_origin: Option<Point>,
	texSize: Point,
	anim: Animator<'a>,
	animated: bool
}

impl<'a> Sprite<'a>
{
	pub fn new() -> Sprite<'a>
	{
		Sprite
		{
			tex: None,
			texRect: sdl2::rect::Rect::new(0, 0, 0, 0),
			scale: Point::num(1.0),
			position: Point::zero(),
			rotation: 0.0,
			absolute_origin: Some(Point::zero()),
			relative_origin: None,
			texSize: Point::zero(),
			animated: false,
			anim: Animator::new()
		}
	}
	pub fn loadTexture(&mut self, path: String)
	{
		let res = Window::getTC().load_texture(Assets::getCurrentDir() + &path.clone());
		self.tex = if res.is_ok() { Some(res.unwrap()) } else { println!("Failed to load texture {}", path.clone().as_str()); None };
		if self.tex.is_none() { return; }
		let query = self.tex.as_mut().unwrap().query();
		self.texSize = Point { x: query.clone().width as f32, y: query.clone().height as f32 };
		self.texRect = sdl2::rect::Rect::new(0, 0, query.clone().width, query.height);
		self.animated = false;
		self.anim = Animator::new();
	}

	pub fn loadAnimator(&mut self, path: String)
	{
		self.texRect = sdl2::rect::Rect::new(0, 0, 0, 0);
		self.anim.loadFromFile(path);
		self.tex = None;
		self.animated = true;
	}

	pub fn draw(&mut self, canvas: &mut sdl2::render::WindowCanvas)
	{
		// canvas.set_scale(self.scale.x.abs() as f32, self.scale.y.abs() as f32);
		let origin = self.getOrigin() * self.scale;

		if self.animated
		{
			let mut anim = self.anim.getCurrentAnimation();
			if anim.is_none() { return; }
			anim.as_mut().unwrap().update();
			let id = anim.unwrap().getCurrentFrame().id;
			let frame = self.anim.getFrame(id);
			// print!("{:?}\r", frame);
			// std::io::stdout().flush();
			canvas.copy_ex(
				self.anim.texture.as_ref().unwrap(),
				frame,
				sdl2::rect::Rect::new(
					self.position.x as i32 - origin.x as i32,
					self.position.y as i32 - origin.y as i32,
					(frame.width() as f32 * self.scale.x) as u32,
					(frame.height() as f32 * self.scale.y) as u32
				),
				self.rotation as f64,
				sdl2::rect::Point::new(origin.x as i32, origin.y as i32),
				self.scale.x < 0.0,
				self.scale.y < 0.0
			);
		}
		else
		{
			if self.tex.is_none() { return; }

			canvas.copy_ex(
				self.tex.as_ref().unwrap(),
				self.texRect,
				sdl2::rect::Rect::new(
					self.position.x as i32 - origin.x as i32,
					self.position.y as i32 - origin.y as i32,
					self.texRect.width(),
					self.texRect.height()
				),
				self.rotation as f64,
				sdl2::rect::Point::new(origin.x as i32, origin.y as i32),
				self.scale.x < 0.0,
				self.scale.y < 0.0
			);
		}
	}

	pub fn getTextureSize(&mut self) -> Point
	{
		let query = self.tex.as_mut().unwrap().query();
		Point
		{
			x: query.clone().width as f32,
			y: query.height as f32
		}
	}

	pub fn scaleToSize(&mut self, size: Point)
	{
		let bounds = if self.animated
		{
			Point
			{
				x: self.anim.getCurrentFrame().clone().width() as f32,
				y: self.anim.getCurrentFrame().clone().height() as f32
			}
		}
		else
		{
			Point
			{
				x: self.texRect.clone().width() as f32,
				y: self.texRect.clone().height() as f32
			}
		};

		self.scale = Point
		{
			x: size.x / bounds.x,
			y: size.y / bounds.y
		};
	}
	
	pub fn getOrigin(&mut self) -> Point
	{
		if self.absolute_origin.is_some() { self.absolute_origin.unwrap() }
		else
		{
			let bounds = if self.animated
			{
				Point
				{
					x: self.anim.getCurrentFrame().clone().width() as f32,
					y: self.anim.getCurrentFrame().clone().height() as f32
				}
			}
			else
			{
				Point
				{
					x: self.texRect.clone().width() as f32,
					y: self.texRect.clone().height() as f32
				}
			};
			bounds * self.relative_origin.unwrap()
		}
	}

	pub fn moveBy(&mut self, p: Point) { self.position += p; }
	pub fn rotate(&mut self, angle: f32) { self.rotation += angle; }
	pub fn scale(&mut self, scale: Point) { self.scale += scale; }
	pub fn setTextureRect(&mut self, r: sdl2::rect::Rect) { self.texRect = r; }
	pub fn getTextureRect(&mut self) -> sdl2::rect::Rect { self.texRect }
	pub fn setPosition(&mut self, pos: Point) { self.position = pos; }
	pub fn getPosiiton(&mut self) -> Point { self.position }
	pub fn setScale(&mut self, factor: Point) { self.scale = factor; }
	pub fn getScale(&mut self) -> Point { self.scale }
	pub fn setRotation(&mut self, angle: f32) { self.rotation = angle; }
	pub fn getRotation(&mut self) -> f32 { self.rotation }
	pub fn setAbsoluteOrigin(&mut self, p: Point) { self.relative_origin = None; self.absolute_origin = Some(p); }
	pub fn setRelativeOrigin(&mut self, p: Point) { self.absolute_origin = None; self.relative_origin = Some(p); }
}