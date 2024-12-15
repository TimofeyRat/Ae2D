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
	base: u8
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
			base: 0
		}
	}

	pub fn load(path: String) -> Self
	{
		let mut font = Font::new();
		let src = crate::ae3d::Assets::openFile(path.clone());
		if src.is_none() { println!("Failed to load font from {path}"); return font; }
		let doc = spex::parsing::XmlReader::parse_auto(src.unwrap());
		if doc.is_err() { println!("Failed to parse font from {path}: {}", doc.err().unwrap()); return font; }

		let mut w = 0.0;
		let mut h = 0.0;

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
				w = node.att_req("scaleW")
					.unwrap_or("0")
					.parse::<f32>()
					.unwrap();
				h = node.att_req("scaleH")
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
									.unwrap() / w,
								ch.att_req("y")
									.unwrap_or("0")
									.parse::<f32>()
									.unwrap() / h,
								ch.att_req("width")
									.unwrap_or("0")
									.parse::<f32>()
									.unwrap() / w,
								ch.att_req("height")
									.unwrap_or("0")
									.parse::<f32>()
									.unwrap() / h
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