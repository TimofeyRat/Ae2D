#[derive(Clone, Debug)]
struct TextPart
{
	pub txt: String,
	pub style: sdl2::ttf::FontStyle,
	pub newline: bool
}

impl TextPart
{
	pub fn new() -> Self
	{
		Self
		{
			newline: false,
			style: sdl2::ttf::FontStyle::NORMAL,
			txt: String::new()
		}
	}
}

pub struct Text<'a>
{
	font: Option<sdl2::ttf::Font<'a, 'a>>,
	txt: Vec<TextPart>
}

impl<'a> Text<'a>
{
	pub fn new() -> Self
	{
		Self
		{
			font: None,
			txt: Vec::new()
		}
	}

	pub fn loadFont(&mut self, p: String, size: u16) -> bool
	{
		let res = super::Window::Window::getTTF().load_font(p.clone(), size);
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
					self.txt.push(part.clone());
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
	}

	pub fn draw(&mut self)
	{
		// 
	}
}