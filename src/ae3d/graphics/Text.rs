use crate::ae3d::Window::Window;

use super::Transformable::Transformable2D;

#[derive(Clone, Debug)]
pub struct Glyph
{
	pub rect: sdl2::rect::FRect,
	pub offset: sdl2::rect::Point,
	pub advance: u8,
}

#[derive(Debug)]
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
		let src = crate::ae3d::Assets::readXML(path.clone());
		if src.is_none() { return font; }
		
		for node in src.unwrap().elements()
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

#[derive(Debug)]
pub struct Text
{
	font: Font,
	text: Vec<StyledText>,
	vertices: i32,
	reload: bool,
	vbo: u32,
	vao: u32,
	fontSize: u8,
	dimensions: glam::Vec2,
	ts: Transformable2D
}

impl Drop for Text
{
	fn drop(&mut self)
	{
		if self.vao == 0 && self.vbo == 0 { return; }
		unsafe
		{
			gl::DeleteVertexArrays(1, &mut self.vao);
			gl::DeleteBuffers(1, &mut self.vbo);
		}
	}
}

impl Text
{
	pub fn new() -> Self
	{
		Self
		{
			font: Font::new(),
			text: vec![],
			vbo: 0,
			vao: 0,
			vertices: 0,
			reload: true,
			fontSize: 48,
			dimensions: glam::Vec2::ZERO,
			ts: Transformable2D::new()
		}
	}

	pub fn loadFont(&mut self, path: String)
	{
		if self.vao == 0 && self.vbo == 0
		{
			unsafe
			{
				gl::GenVertexArrays(1, &mut self.vao);
				gl::GenBuffers(1, &mut self.vbo);
			}
		}
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

		self.text.clear();

		let chars: Vec<char> = str.as_str().chars().collect();
		let mut index = 0;
		while index < chars.len()
		{
			let c = *chars.get(index).unwrap();

			if c == '^' && *chars.get(index + 1).unwrap_or(&' ') == '('
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
				while *chars.get(index).unwrap_or(&')') != ')'
				{
					raw.push(*chars.get(index).unwrap_or(&' '));
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
			else if c == '\n' { part.newline = true; }
			else { part.text.push(c); }
			
			index += 1;
		}
		self.text.push(part);

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
			shader.setMat4("model".to_string(), &self.ts.getMatrix().to_cols_array());
			gl::DrawArrays(
				gl::QUADS,
				0,
				self.vertices
			);

			gl::BindTexture(gl::TEXTURE_2D, 0);
			gl::BindVertexArray(0);
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		}
	}

	pub fn update(&mut self)
	{
		self.dimensions = glam::Vec2::ZERO;
		let mut vertices: Vec<f32> = vec![];

		let mut pos = glam::Vec2::ZERO;

		let scale = self.fontSize as f32 / self.font.height as f32;
		let italic = self.font.height as f32 * 10.0_f32.to_radians().sin();

		for part in self.text.iter()
		{
			let clr = glam::vec4(
				part.color.r as f32 / 255.0,
				part.color.g as f32 / 255.0,
				part.color.b as f32 / 255.0,
				part.color.a as f32 / 255.0
			);
			for ch in part.text.chars()
			{
				let glyph = self.font.getGlyph(ch);
				vertices.append(&mut vec![
					pos.x + (glyph.offset.x as f32 + if part.italic { italic } else { 0.0 }) * scale,
					pos.y + (glyph.offset.y as f32 - self.font.base as f32) * scale,
					glyph.rect.left() / self.font.bitmapSize.x,
					glyph.rect.top() / self.font.bitmapSize.y,
					clr.x, clr.y, clr.z, clr.w,
	
					pos.x + (glyph.offset.x as f32 + glyph.rect.width() + if part.italic { italic } else { 0.0 }) * scale,
					pos.y + (glyph.offset.y as f32 - self.font.base as f32) * scale,
					glyph.rect.right() / self.font.bitmapSize.x,
					glyph.rect.top() / self.font.bitmapSize.y,
					clr.x, clr.y, clr.z, clr.w,
	
					pos.x + (glyph.offset.x as f32 + glyph.rect.width()) * scale,
					pos.y + (glyph.offset.y as f32 - self.font.base as f32 * scale + glyph.rect.height()) * scale,
					glyph.rect.right() / self.font.bitmapSize.x,
					glyph.rect.bottom() / self.font.bitmapSize.y,
					clr.x, clr.y, clr.z, clr.w,
	
					pos.x + (glyph.offset.x as f32) * scale,
					pos.y + (glyph.offset.y as f32 - self.font.base as f32 * scale + glyph.rect.height()) * scale,
					glyph.rect.left() / self.font.bitmapSize.x,
					glyph.rect.bottom() / self.font.bitmapSize.y,
					clr.x, clr.y, clr.z, clr.w
				]);

				self.dimensions.x = self.dimensions.x.max(
					pos.x + (glyph.offset.x as f32 + glyph.rect.width() + if part.italic { italic } else { 0.0 }) * scale
				);
				self.dimensions.y = self.dimensions.y.max(
					pos.y + (glyph.offset.y as f32 - self.font.base as f32 * scale + glyph.rect.height()) * scale
				);

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
				gl::DYNAMIC_DRAW
			);

			gl::BindVertexArray(self.vao);
			gl::EnableVertexAttribArray(0);
			gl::EnableVertexAttribArray(1);

			gl::VertexAttribPointer(
				0,
				4,
				gl::FLOAT,
				gl::FALSE,
				(8 * size_of::<f32>()) as i32,
				std::ptr::null()
			);
			gl::VertexAttribPointer(
				1,
				4,
				gl::FLOAT,
				gl::FALSE,
				(8 * size_of::<f32>()) as i32,
				(4 * size_of::<f32>()) as *const _
			);
		}

		self.vertices = vertices.len() as i32 / 4;

		self.reload = false;
	}

	pub fn setSize(&mut self, size: u8)
	{
		self.fontSize = size;
		self.reload = true;
	}

	pub fn getBounds(&mut self) -> sdl2::rect::FRect
	{
		if self.reload { self.update(); }

		let p1 = self.ts.getMatrix() * glam::vec4(0.0, 0.0, 0.0, 1.0);
		let p2 = self.ts.getMatrix() * glam::vec4(self.dimensions.x, 0.0, 0.0, 1.0);
		let p3 = self.ts.getMatrix() * glam::vec4(self.dimensions.x, self.dimensions.y, 0.0, 1.0);
		let p4 = self.ts.getMatrix() * glam::vec4(0.0, self.dimensions.y, 0.0, 1.0);

		let min = p1.min(p2).min(p3).min(p4);
		let max = p1.max(p2).max(p3).max(p4);

		sdl2::rect::FRect::new(min.x, min.y, max.x - min.x, max.y - min.y)
	}

	pub fn getString(&mut self) -> String
	{
		let mut out = String::new();

		for part in &self.text
		{
			out += &part.text;
		}
		
		out
	}

	pub fn getDimensions(&mut self) -> glam::Vec2
	{
		if self.reload { self.update(); }
		self.dimensions
	}

	unsafe extern "C" fn setPosFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let x = obj.getScript().to_number(-2) as f32;
		let y = obj.getScript().to_number(-1) as f32;
		obj.getText().ts.setPosition(glam::vec2(x, y));
		0
	}

	unsafe extern "C" fn translateFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let x = obj.getScript().to_number(-2) as f32;
		let y = obj.getScript().to_number(-1) as f32;
		obj.getText().ts.translate(glam::vec2(x, y));
		0
	}

	unsafe extern "C" fn getPosFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let pos = obj.getText().ts.getPosition();
		obj.getScript().push_number(pos.x as f64);
		obj.getScript().push_number(pos.y as f64);
		2
	}

	unsafe extern "C" fn setRotFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let angle = obj.getScript().to_number(-1) as f32;
		obj.getText().ts.setRotation(angle);
		0
	}

	unsafe extern "C" fn rotateFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let angle = obj.getScript().to_number(-1) as f32;
		obj.getText().ts.rotate(angle);
		0
	}

	unsafe extern "C" fn getRotFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let angle = obj.getText().ts.getRotation() as f64;
		obj.getScript().push_number(angle);
		1
	}

	unsafe extern "C" fn setScaleFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let x = obj.getScript().to_number(-2) as f32;
		let y = obj.getScript().to_number(-1) as f32;
		obj.getText().ts.setScale(glam::vec2(x, y));
		0
	}

	unsafe extern "C" fn scaleFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let x = obj.getScript().to_number(-2) as f32;
		let y = obj.getScript().to_number(-1) as f32;
		obj.getText().ts.scale(glam::vec2(x, y));
		0
	}

	unsafe extern "C" fn getScaleFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let scale = obj.getText().ts.getScale();
		obj.getScript().push_number(scale.x as f64);
		obj.getScript().push_number(scale.y as f64);
		2
	}

	unsafe extern "C" fn setOriginFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let x = obj.getScript().to_number(-2) as f32;
		let y = obj.getScript().to_number(-1) as f32;
		obj.getText().ts.setOrigin(glam::vec2(x, y));
		0
	}

	unsafe extern "C" fn getOriginFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let origin = obj.getText().ts.getOrigin();
		obj.getScript().push_number(origin.x as f64);
		obj.getScript().push_number(origin.y as f64);
		2
	}

	unsafe extern "C" fn boundsFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let bounds = obj.getText().getBounds();
		obj.getScript().push_number(bounds.left() as f64);
		obj.getScript().push_number(bounds.top() as f64);
		obj.getScript().push_number(bounds.width() as f64);
		obj.getScript().push_number(bounds.height() as f64);
		4
	}

	unsafe extern "C" fn sizeFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let size = obj.getText().getDimensions();
		obj.getScript().push_number(size.x as f64);
		obj.getScript().push_number(size.y as f64);
		2
	}

	unsafe extern "C" fn setStringFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let text = obj.getScript().to_str(-1).unwrap_or("").to_string();
		obj.getText().setString(text);
		0
	}

	unsafe extern "C" fn getStringFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let text = obj.getText().getString();
		obj.getScript().push_string(&text);
		1
	}

	pub fn initLua(&mut self, script: &mut lua::State)
	{
		script.create_table(0, 15);

		script.push_string("setPosition"); script.push_fn(Some(Text::setPosFN)); script.set_table(-3);
		script.push_string("translate"); script.push_fn(Some(Text::translateFN)); script.set_table(-3);
		script.push_string("getPosition"); script.push_fn(Some(Text::getPosFN)); script.set_table(-3);
		
		script.push_string("setRotation"); script.push_fn(Some(Text::setRotFN)); script.set_table(-3);
		script.push_string("rotate"); script.push_fn(Some(Text::rotateFN)); script.set_table(-3);
		script.push_string("getRotation"); script.push_fn(Some(Text::getRotFN)); script.set_table(-3);

		script.push_string("setScale"); script.push_fn(Some(Text::setScaleFN)); script.set_table(-3);
		script.push_string("scale"); script.push_fn(Some(Text::scaleFN)); script.set_table(-3);
		script.push_string("getScale"); script.push_fn(Some(Text::getScaleFN)); script.set_table(-3);

		script.push_string("setOrigin"); script.push_fn(Some(Text::setOriginFN)); script.set_table(-3);
		script.push_string("getOrigin"); script.push_fn(Some(Text::getOriginFN)); script.set_table(-3);

		script.push_string("bounds"); script.push_fn(Some(Text::boundsFN)); script.set_table(-3);
		script.push_string("size"); script.push_fn(Some(Text::sizeFN)); script.set_table(-3);

		script.push_string("setString"); script.push_fn(Some(Text::setStringFN)); script.set_table(-3);
		script.push_string("getString"); script.push_fn(Some(Text::getStringFN)); script.set_table(-3);

		script.set_global("text");
	}
}