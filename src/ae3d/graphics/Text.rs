#[derive(Clone, Debug)]
pub struct Glyph
{
	pub rect: sdl2::rect::FRect,
	pub offset: sdl2::rect::Point,
	pub advance: u8,
}

pub struct Font
{
	page: u32,
	glyphs: std::collections::HashMap<u16, Glyph>,
	height: u8,
	name: String,
	base: u8,
	pub bitmapSize: glam::Vec2
}

impl Font
{
	pub fn new() -> Self
	{
		Self
		{
			page: 0,
			glyphs: std::collections::HashMap::new(),
			height: 0,
			name: "".to_string(),
			base: 0,
			bitmapSize: glam::vec2(0.0, 0.0)
		}
	}

	pub fn load(path: String) -> Self
	{
		let mut font = Font::new();
		let src = crate::ae3d::Assets::openFile(path.clone());
		if src.is_none() { println!("Failed to load font from {path}"); return font; }
		let doc = spex::parsing::XmlReader::parse_auto(src.unwrap());
		if doc.is_err() { println!("Failed to parse font from {path}: {}", doc.err().unwrap()); return font; }

		for node in doc.unwrap().root().elements()
		{
			let name = node.name().local_part();
			if name == "info"
			{
				font.name =
					node.att_req("face")
					.unwrap_or("")
					.to_string();
			}
			if name == "common"
			{
				font.height =
					node.att_req("lineHeight")
					.unwrap_or("0")
					.parse::<u8>()
					.unwrap();
				font.bitmapSize.x = node.att_req("scaleW")
					.unwrap_or("0")
					.parse::<f32>()
					.unwrap();
				font.bitmapSize.y = node.att_req("scaleH")
					.unwrap_or("0")
					.parse::<f32>()
					.unwrap();
			}
			if name == "pages"
			{
				let mut p = path.clone();
				while p.chars().last().unwrap() != '/' { p.pop(); }
				font.page = crate::ae3d::Assets::getTexture(
					p + node.elements().nth(0).unwrap()
					.att_req("file")
					.unwrap_or("")
				);
			}
			if name == "chars"
			{
				for ch in node.elements()
				{
					font.glyphs.insert(
						ch.att_req("id")
							.unwrap_or("0")
							.parse::<u16>()
							.unwrap(),
						Glyph
						{
							rect: sdl2::rect::FRect::new(
								ch.att_req("x")
									.unwrap_or("0")
									.parse::<f32>()
									.unwrap(),
								ch.att_req("y")
									.unwrap_or("0")
									.parse::<f32>()
									.unwrap(),
								ch.att_req("width")
									.unwrap_or("0")
									.parse::<f32>()
									.unwrap(),
								ch.att_req("height")
									.unwrap_or("0")
									.parse::<f32>()
									.unwrap()
							),
							offset: sdl2::rect::Point::new(
								ch.att_req("xoffset")
									.unwrap_or("0")
									.parse::<i32>()
									.unwrap(),
								ch.att_req("yoffset")
									.unwrap_or("0")
									.parse::<i32>()
									.unwrap()
							),
							advance: ch.att_req("xadvance")
								.unwrap_or("0")
								.parse::<u8>()
								.unwrap()
						}
					);
				}
			}
		}

		println!("Loaded {} chars from {path}", font.glyphs.len());

		font
	}

	pub fn getGlyph(&mut self, c: char) -> Glyph
	{
		self.glyphs.get(&(c as u16)).unwrap().clone()
	}

	pub fn bindTexture(&mut self)
	{
		unsafe
		{
			gl::BindTexture(gl::TEXTURE_2D, self.page);
		}
	}

	pub fn unbindTexture()
	{
		unsafe
		{
			gl::BindTexture(gl::TEXTURE_2D, 0);
		}
	}
}

impl Drop for Font
{
	fn drop(&mut self)
	{
		unsafe
		{
			gl::DeleteTextures(1, &self.page);
		}
	}
}

#[derive(PartialEq)]
pub enum Anchor { Left, Center, Right, Bottom, Top }

#[derive(Debug)]
struct StyledText
{
	pub text: String,
	pub bold: bool,
	pub italic: bool,
	pub underlined: bool,
	pub strikethrough: bool,
	pub newline: bool,
	pub color: sdl2::pixels::Color
}

pub struct Text
{
	font: Font,
	position: glam::Vec2,
	text: Vec<StyledText>,
	vertices: i32,
	reload: bool,
	vbo: u32,
	vao: u32,
	fontSize: u8
}

impl Text
{
	pub fn new() -> Self
	{
		let mut vao = 0;
		let mut vbo = 0;

		unsafe
		{
			gl::GenBuffers(1, &mut vbo);
			gl::GenVertexArrays(1, &mut vao);
		}

		println!("{vao}, {vbo}");

		Self
		{
			font: Font::new(),
			position: glam::Vec2::ZERO,
			text: vec![],
			vbo,
			vao,
			vertices: 0,
			reload: true,
			fontSize: 48
		}
	}

	pub fn loadFont(&mut self, path: String)
	{
		self.font = Font::load(path);
	}

	pub fn setString(&mut self, str: String)
	{
		let mut part = StyledText
		{
			text: String::new(),
			bold: false,
			italic: false,
			underlined: false,
			strikethrough: false,
			newline: false,
			color: sdl2::pixels::Color::WHITE
		};

		let chars = str.as_str();
		let mut index = 0;
		while index < chars.len()
		{
			let c = &chars[index..index+1];

			if c == "^" && chars.get(index+1..index+2).unwrap_or("") == "("
			{
				if !part.text.is_empty()
				{
					self.text.push(part);
					part = StyledText
					{
						text: String::new(),
						bold: false,
						italic: false,
						underlined: false,
						strikethrough: false,
						newline: false,
						color: sdl2::pixels::Color::WHITE
					};
				}
				let mut raw = String::new();

				index += 2;
				while chars.get(index..index+1).unwrap_or(")") != ")"
				{
					raw.push_str(chars.get(index..index+1).unwrap_or(""));
					index += 1;
				}

				let style: Vec<&str> = raw.split(" ").collect();
				for el in style
				{
					if el == "*" { part.bold = true; }
					if el == "/" { part.italic = true; }
					if el == "_" { part.underlined = true; }
					if el == "-" { part.strikethrough = true; }
					if el.contains("clr")
					{
						part.color = crate::ae3d::Window::Window::getColor(el.split("=").nth(1).unwrap().to_string());
					}
				}
			}
			else if c == "\n" { part.newline = true; }
			else { part.text.push_str(c); }
			
			index += 1;
		}
		self.text.push(part);

		println!("{:?}", self.text);
		
		self.reload = true;
	}

	pub fn draw(&mut self, shader: &mut super::Shader::Shader)
	{
		if self.reload { self.update(); }

		unsafe
		{
			gl::BindVertexArray(self.vao);

			gl::ActiveTexture(gl::TEXTURE0);
			gl::BindTexture(gl::TEXTURE_2D, self.font.page);
			shader.setInt("tex".to_string(), 0);


			gl::DrawArrays(
				gl::QUADS,
				0,
				self.vertices
			);
		}
	}

	pub fn update(&mut self)
	{
		let mut vertices: Vec<f32> = vec![];

		let mut pos = glam::Vec2::splat(100.0);

		let scale = self.fontSize as f32 / self.font.height as f32;
		let italic = self.font.height as f32 * 10.0_f32.to_radians().sin();

		for part in self.text.iter()
		{
			for ch in part.text.chars()
			{
				let glyph = self.font.getGlyph(ch);
				vertices.append(&mut vec![
					pos.x + (glyph.offset.x as f32 + if part.italic { italic } else { 0.0 }) * scale,
					pos.y + (glyph.offset.y as f32 - self.font.base as f32) * scale,
					glyph.rect.left() / self.font.bitmapSize.x,
					glyph.rect.top() / self.font.bitmapSize.y,
	
					pos.x + (glyph.offset.x as f32 + glyph.rect.width() + if part.italic { italic } else { 0.0 }) * scale,
					pos.y + (glyph.offset.y as f32 - self.font.base as f32) * scale,
					glyph.rect.right() / self.font.bitmapSize.x,
					glyph.rect.top() / self.font.bitmapSize.y,
	
					pos.x + (glyph.offset.x as f32 + glyph.rect.width()) * scale,
					pos.y + (glyph.offset.y as f32 - self.font.base as f32 * scale + glyph.rect.height()) * scale,
					glyph.rect.right() / self.font.bitmapSize.x,
					glyph.rect.bottom() / self.font.bitmapSize.y,
	
					pos.x + (glyph.offset.x as f32) * scale,
					pos.y + (glyph.offset.y as f32 - self.font.base as f32 * scale + glyph.rect.height()) * scale,
					glyph.rect.left() / self.font.bitmapSize.x,
					glyph.rect.bottom() / self.font.bitmapSize.y
				]);
				pos.x += glyph.advance as f32 * scale;
			}
			if part.newline
			{
				pos.x = 0.0;
				pos.y += self.font.height as f32;
			}
		}

		unsafe
		{
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
			gl::BufferData(
				gl::ARRAY_BUFFER,
				(vertices.len() * size_of::<f32>()) as isize,
				vertices.as_ptr() as *const _,
				gl::STATIC_DRAW
			);

			gl::BindVertexArray(self.vao);
			gl::EnableVertexAttribArray(0);

			gl::VertexAttribPointer(
				0,
				4,
				gl::FLOAT,
				gl::FALSE,
				(4 * size_of::<f32>()) as i32,
				std::ptr::null()
			);
		}

		self.vertices = vertices.len() as i32 / 2;

		self.reload = false;
	}

	pub fn setSize(&mut self, size: u8)
	{
		self.fontSize = size;
		self.reload = true;
	}
}

impl Drop for Text
{
	fn drop(&mut self)
	{
		unsafe
		{
			gl::DeleteBuffers(1, &self.vbo);
			gl::DeleteVertexArrays(1, &self.vao);
		}
	}
}