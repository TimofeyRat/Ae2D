use crate::ae3d::Window::Window;

#[derive(PartialEq)]
pub enum Anchor  { Left, Center, Right, Top, Bottom }

struct TextPart
{
	pub txt: String,
	pub style: sdl2::ttf::FontStyle,
	pub newline: bool,
	pub clr: sdl2::pixels::Color,
}

impl TextPart
{
	pub fn new() -> Self
	{
		Self
		{
			newline: false,
			style: sdl2::ttf::FontStyle::NORMAL,
			txt: String::new(),
			clr: sdl2::pixels::Color::RGBA(255, 255, 255, 255)
		}
	}
}

pub struct Text<'a>
{
	font: Option<sdl2::ttf::Font<'a, 'a>>,
	txt: Vec<TextPart>,
	prerendered: Option<sdl2::surface::Surface<'a>>,
	anchorX: Anchor,
	anchorY: Anchor,
	rendered: Vec<Vec<sdl2::render::Texture<'a>>>,
	lineWidth: Vec<f32>,
	pub transform: crate::ae3d::math::Transformable::Transformable
}

impl<'a> Text<'a>
{
	pub fn new() -> Self
	{
		Self
		{
			font: None,
			txt: Vec::new(),
			prerendered: None,
			anchorX: Anchor::Left,
			anchorY: Anchor::Top,
			rendered: Vec::new(),
			lineWidth: Vec::new(),
			transform: crate::ae3d::math::Transformable::Transformable::new()
		}
	}

	pub fn loadFont(&mut self, p: String, size: u16) -> bool
	{
		let res = crate::ae3d::Window::Window::getTTF().load_font(p.clone(), size);
		if res.is_ok()
		{
			self.font = Some(res.unwrap());
			true
		}
		else
		{
			self.font = None;
			println!("Failed to load font {p}: {}", res.err().unwrap());
			false
		}
	}

	pub fn setString(&mut self, txt: String)
	{
		self.txt.clear();
		let mut part = TextPart::new();
		let chars = txt.as_str();
		let mut index: usize = 0;
		while index < txt.len()
		{
			let mut c = &chars[index..index + 1];
			if c == "^"
			{
				index += 1;
				if &chars[index..index + 1] != "(" { continue; }

				if !part.txt.is_empty()
				{
					self.txt.push(part);
					part = TextPart::new();
				}

				let mut raw = String::new();
				index += 1;

				c = &chars[index..index + 1];

				while c != ")"
				{
					raw.push_str(c);
					index += 1;
					c = &chars[index..index + 1];
				}
				
				let style: Vec<&str> = raw.split(" ").collect();
				for el in style
				{
					if el == "*" { part.style = part.style | sdl2::ttf::FontStyle::BOLD; }
					if el == "/" { part.style = part.style | sdl2::ttf::FontStyle::ITALIC; }
					if el == "_" { part.style = part.style | sdl2::ttf::FontStyle::UNDERLINE; }
					if el == "-" { part.style = part.style | sdl2::ttf::FontStyle::STRIKETHROUGH; }
					if el.contains("clr")
					{
						part.clr = Window::getColor(
							el.split("=")
							.collect::<Vec<&str>>()
							.get(1)
							.unwrap()
							.to_string()
						);
					}
				}
			}
			else if c == "\n"
			{
				part.newline = true;
			}
			else
			{
				part.txt.push_str(c);
			}
			index += 1;
		}
		self.txt.push(part);

		self.render();
	}

	pub fn render(&mut self)
	{
		if self.font.is_none() { return }
		self.rendered.clear();
		self.lineWidth.clear();

		let mut line: Vec<sdl2::render::Texture<'a>> = Vec::new();
		let mut lw = 0.0;

		for part in self.txt.iter()
		{
			self.font.as_mut().unwrap().set_style(part.style);

			let res = self.font.as_mut().unwrap().render(&part.txt).blended(part.clr);
			if res.is_err()
			{
				println!("Failed to render text part \"{}\": {}", part.txt, res.err().unwrap());
				return;
			}
			
			lw += res.as_ref().unwrap().size().0 as f32;
			line.push(res.unwrap().as_texture(Window::getTC()).expect("Failed to create texture while rendering text"));
			if part.newline
			{
				self.rendered.push(line);
				self.lineWidth.push(lw);
				line = Vec::new();
				lw = 0.0;
			}
		}
		self.rendered.push(line);
		self.lineWidth.push(lw);
	}

	pub fn draw(&mut self, canvas: &mut sdl2::render::WindowCanvas)
	{
		let mut lineNumber = 0;
		let mut pos = self.transform.getPosition();
		if self.anchorY == Anchor::Center
		{
			pos.y -= self.font.as_mut().unwrap().height() as f32 * (self.lineWidth.len() as f32 / 2.0);
		}
		if self.anchorY == Anchor::Bottom
		{
			pos.y -= self.font.as_mut().unwrap().height() as f32 * self.lineWidth.len() as f32;
		}

		for line in self.rendered.iter()
		{
			let lineWidth = self.lineWidth.get(lineNumber).unwrap();
			if self.anchorX == Anchor::Center { pos.x -= lineWidth / 2.0; }
			if self.anchorX == Anchor::Right { pos.x -= lineWidth; }
			for element in line.iter()
			{
				let w = element.query().width;
				let h = element.query().height;
				canvas.copy(
					element,
					None,
					sdl2::rect::Rect::new(
						pos.x as i32, pos.y as i32,
						w, h
					)
				);
				pos.x += w as f32;
			}
			lineNumber += 1;
			pos.y += self.font.as_mut().unwrap().height() as f32;
			pos.x = Window::getSize().x / 2.0;
		}
	}

	pub fn setAnchor(&mut self, x: Anchor, y: Anchor)
	{
		self.anchorX = x;
		self.anchorY = y;
	}
}