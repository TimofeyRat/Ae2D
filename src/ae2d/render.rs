use sdl2::image::LoadTexture;

use super::window::Window;
use super::math;

pub struct Sprite<'a>
{
	tex: Option<sdl2::render::Texture<'a>>,
	texRect: sdl2::rect::Rect,
	scale: math::Point,
	position: math::Point,
	texSize: math::Point
}

impl<'a> Sprite<'a>
{
	pub fn new() -> Sprite<'a>
	{
		Sprite
		{
			tex: None,
			texRect: sdl2::rect::Rect::new(0, 0, 0, 0),
			scale: math::Point::num(1.0),
			position: math::Point::zero(),
			texSize: math::Point::zero()
		}
	}
	pub fn loadFromFile(&mut self, path: String)
	{
		let res = Window::getTC().load_texture(sdl2::filesystem::base_path().unwrap() + &path.clone());
		self.tex = if res.is_ok() { Some(res.unwrap()) } else { println!("Failed to load texture {}", path.clone().as_str()); None };
		if self.tex.is_none() { return; }
		let query = self.tex.as_mut().unwrap().query();
		self.texSize = math::Point { x: query.clone().width as f64, y: query.clone().height as f64 };
		self.texRect = sdl2::rect::Rect::new(0, 0, query.clone().width, query.height);
	}

	pub fn draw(&mut self, canvas: &mut sdl2::render::WindowCanvas)
	{
		if self.tex.is_none() { return; }
		let _ = canvas.copy(
			self.tex.as_ref().unwrap(),
			self.texRect,
			sdl2::rect::Rect::new(
				self.position.x as i32,
				self.position.y as i32,
				(self.texRect.width() as f64 * self.scale.x) as u32,
				(self.texRect.height() as f64 * self.scale.y) as u32
			)
		);
	}

	pub fn getTextureSize(&mut self) -> math::Point
	{
		let query = self.tex.as_mut().unwrap().query();
		math::Point
		{
			x: query.clone().width as f64,
			y: query.height as f64
		}
	}

	pub fn scaleToSize(&mut self, size: math::Point)
	{
		self.scale = math::Point
		{
			x: size.x / self.texRect.clone().width() as f64,
			y: size.y / self.texRect.clone().height() as f64
		};
	}

	pub fn setTextureRect(&mut self, r: sdl2::rect::Rect) { self.texRect = r; }
	pub fn getTextureRect(&mut self) -> sdl2::rect::Rect { self.texRect }
	pub fn setPosition(&mut self, pos: math::Point) { self.position = pos; }
	pub fn getPosiiton(&mut self) -> math::Point { self.position }
	pub fn setScale(&mut self, factor: math::Point) { self.scale = factor; }
	pub fn getScale(&mut self) -> math::Point { self.scale }
}