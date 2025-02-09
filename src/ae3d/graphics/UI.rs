use crate::ae3d::{Programmable::{Programmable, Variable}, Window::Window};

use super::{FrameAnimation::Animator, Text::Text, Transformable::Transformable2D};

pub struct Image
{
	animation: Animator,
	vao: u32,
	vbo: u32,
	vertices: [f32; 32],
	ts: Transformable2D,
	color: glam::Vec4
}

impl Image
{
	pub fn new() -> Self
	{
		Self
		{
			animation: Animator::new(),
			vao: 0,
			vbo: 0,
			vertices: [0.0; 32],
			ts: Transformable2D::new(),
			color: glam::Vec4::ONE
		}
	}

	pub fn parse(node: &spex::xml::Element) -> Self
	{
		let mut img = Image::new();
		if node.name().local_part() != "image" { return img; }
		
		unsafe
		{
			gl::GenVertexArrays(1, &mut img.vao);
			gl::GenBuffers(1, &mut img.vbo);

			gl::BindBuffer(gl::ARRAY_BUFFER, img.vbo);
			gl::BindVertexArray(img.vao);
			gl::EnableVertexAttribArray(0);
			gl::EnableVertexAttribArray(1);
			gl::VertexAttribPointer(
				0, 4, gl::FLOAT,
				gl::FALSE,
				(8 * size_of::<f32>()) as i32,
				std::ptr::null()
			);
			gl::VertexAttribPointer(
				1, 4, gl::FLOAT,
				gl::FALSE,
				(8 * size_of::<f32>()) as i32,
				(4 * size_of::<f32>()) as *const _
			);
		}

		img.animation.load(node.att_opt("anim")
			.unwrap_or_else(|| { println!("No animation provided"); "" })
			.to_string()
		);

		img
	}

	pub fn draw(&mut self, shader: &mut super::Shader::Shader)
	{
		self.animation.update();
		let frame = self.animation.getCurrentFrame();
		let size = self.animation.getFrameSize();
		self.vertices = [
			0.0, 0.0,						frame.left(), frame.top(),
			self.color.x, self.color.y, self.color.z, self.color.w,

			size.x as f32, 0.0,				frame.right(), frame.top(),
			self.color.x, self.color.y, self.color.z, self.color.w,
			
			size.x as f32, size.y as f32,	frame.right(), frame.bottom(),
			self.color.x, self.color.y, self.color.z, self.color.w,
			
			0.0, size.y as f32,				frame.left(), frame.bottom(),
			self.color.x, self.color.y, self.color.z, self.color.w
		];

		shader.setInt("tex".to_string(), 0);
		shader.setMat4("model".to_string(), &self.ts.getMatrix().to_cols_array());
		unsafe
		{
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
			gl::BufferData(
				gl::ARRAY_BUFFER,
				(self.vertices.len() * size_of::<f32>()) as isize,
				self.vertices.as_ptr() as *const _,
				gl::DYNAMIC_DRAW
			);

			gl::BindVertexArray(self.vao);
			gl::ActiveTexture(gl::TEXTURE0);
			self.animation.bindTexture();
			gl::DrawArrays(gl::QUADS, 0, 4);
		}
	}

	pub fn getBounds(&mut self) -> sdl2::rect::FRect
	{
		let size = self.animation.getFrameSize();
		let p1 = self.ts.getMatrix() * glam::vec4(0.0, 0.0, 0.0, 1.0);
		let p2 = self.ts.getMatrix() * glam::vec4(size.x as f32, 0.0, 0.0, 1.0);
		let p3 = self.ts.getMatrix() * glam::vec4(size.x as f32, size.y as f32, 0.0, 1.0);
		let p4 = self.ts.getMatrix() * glam::vec4(0.0, size.y as f32, 0.0, 1.0);

		let min = p1.min(p2).min(p3).min(p4);
		let max = p1.max(p2).max(p3).max(p4);

		sdl2::rect::FRect::new(min.x, min.y, max.x - min.x, max.y - min.y)
	}

	unsafe extern "C" fn setPosFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let x = obj.script.to_number(-2) as f32;
		let y = obj.script.to_number(-1) as f32;
		obj.image.ts.setPosition(glam::vec2(x, y));
		0
	}

	unsafe extern "C" fn translateFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let x = obj.script.to_number(-2) as f32;
		let y = obj.script.to_number(-1) as f32;
		obj.image.ts.translate(glam::vec2(x, y));
		0
	}

	unsafe extern "C" fn getPosFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		obj.script.push_number(obj.image.ts.getPosition().x as f64);
		obj.script.push_number(obj.image.ts.getPosition().y as f64);
		2
	}

	unsafe extern "C" fn setRotFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		obj.image.ts.setRotation(obj.script.to_number(-1) as f32);
		0
	}

	unsafe extern "C" fn rotateFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		obj.image.ts.rotate(obj.script.to_number(-1) as f32);
		0
	}

	unsafe extern "C" fn getRotFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		obj.script.push_number(obj.image.ts.getRotation() as f64);
		1
	}

	unsafe extern "C" fn setScaleFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let x = obj.script.to_number(-2) as f32;
		let y = obj.script.to_number(-1) as f32;
		obj.image.ts.setScale(glam::vec2(x, y));
		0
	}

	unsafe extern "C" fn scaleFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let x = obj.script.to_number(-2) as f32;
		let y = obj.script.to_number(-1) as f32;
		obj.image.ts.scale(glam::vec2(x, y));
		0
	}

	unsafe extern "C" fn getScaleFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		obj.script.push_number(obj.image.ts.getScale().x as f64);
		obj.script.push_number(obj.image.ts.getScale().y as f64);
		2
	}

	unsafe extern "C" fn setOriginFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let x = obj.script.to_number(-2) as f32;
		let y = obj.script.to_number(-1) as f32;
		obj.image.ts.setOrigin(glam::vec2(x, y));
		0
	}

	unsafe extern "C" fn getOriginFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		obj.script.push_number(obj.image.ts.getOrigin().x as f64);
		obj.script.push_number(obj.image.ts.getOrigin().y as f64);
		2
	}

	unsafe extern "C" fn boundsFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let bounds = obj.image.getBounds();
		obj.script.push_number(bounds.left() as f64);
		obj.script.push_number(bounds.top() as f64);
		obj.script.push_number(bounds.width() as f64);
		obj.script.push_number(bounds.height() as f64);
		4
	}

	unsafe extern "C" fn sizeFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let size = obj.image.animation.getFrameSize();
		obj.script.push_number(size.x as f64);
		obj.script.push_number(size.y as f64);
		2
	}

	unsafe extern "C" fn setAnimFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let name = obj.script.to_str(-1).unwrap_or("");
		obj.image.animation.setCurrentAnimation(name.to_string());
		0
	}

	pub fn initLua(&mut self, script: &mut lua::State)
	{
		script.create_table(0, 14);

		script.push_string("setPosition"); script.push_fn(Some(Image::setPosFN)); script.set_table(-3);
		script.push_string("translate"); script.push_fn(Some(Image::translateFN)); script.set_table(-3);
		script.push_string("getPosition"); script.push_fn(Some(Image::getPosFN)); script.set_table(-3);
		
		script.push_string("setRotation"); script.push_fn(Some(Image::setRotFN)); script.set_table(-3);
		script.push_string("rotate"); script.push_fn(Some(Image::rotateFN)); script.set_table(-3);
		script.push_string("getRotation"); script.push_fn(Some(Image::getRotFN)); script.set_table(-3);

		script.push_string("setScale"); script.push_fn(Some(Image::setScaleFN)); script.set_table(-3);
		script.push_string("scale"); script.push_fn(Some(Image::scaleFN)); script.set_table(-3);
		script.push_string("getScale"); script.push_fn(Some(Image::getScaleFN)); script.set_table(-3);

		script.push_string("setOrigin"); script.push_fn(Some(Image::setOriginFN)); script.set_table(-3);
		script.push_string("getOrigin"); script.push_fn(Some(Image::getOriginFN)); script.set_table(-3);

		script.push_string("bounds"); script.push_fn(Some(Image::boundsFN)); script.set_table(-3);
		script.push_string("size"); script.push_fn(Some(Image::sizeFN)); script.set_table(-3);
		script.push_string("setAnimation"); script.push_fn(Some(Image::setAnimFN)); script.set_table(-3);

		script.set_global("image");
	}
}

impl Drop for Image
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

pub struct Object
{
	name: String,
	image: Image,
	text: Text,
	children: Vec<Object>,
	script: lua::State,
	init: bool,
	order: [char; 3],
	hasScript: bool,
	vars: Programmable
}

impl Object
{
	pub fn new() -> Self
	{
		Self
		{
			name: String::new(),
			image: Image::new(),
			text: Text::new(),
			children: vec![],
			script: lua::State::new(),
			init: false,
			order: ['i', 't', 'c'],
			hasScript: false,
			vars: std::collections::HashMap::new()
		}
	}

	unsafe extern "C" fn setNumFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let name = obj.script.to_str(-2).unwrap_or("").to_string();
		let num = obj.script.to_number(-1) as f32;
		obj.vars.insert(name, Variable { num, string: String::new() });
		1
	}

	unsafe extern "C" fn getNumFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let name = obj.script.to_str(-1).unwrap_or("");
		let num = obj.vars.get(&name.to_string()).unwrap_or(&Variable::new()).num;
		obj.script.push_number(num as f64);
		1
	}

	unsafe extern "C" fn setStrFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let name = obj.script.to_str(-2).unwrap_or("").to_string();
		let string = obj.script.to_str(-1).unwrap_or("").to_string();
		obj.vars.insert(name, Variable { num: 0.0, string });
		1
	}

	unsafe extern "C" fn getStrFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		let name = obj.script.to_str(-1).unwrap_or("");
		let def = Variable::new();
		let string = &obj.vars.get(&name.to_string()).unwrap_or(&def).string;
		obj.script.push_string(&string);
		1
	}

	unsafe extern "C" fn nameFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = Window::getUI().scriptExecutor.as_mut().unwrap();
		obj.script.push_string(&obj.name);
		1
	}

	pub fn parse(base: &spex::xml::Element) -> Self
	{
		let mut obj = Object::new();
		if base.name().local_part() != "object" { return obj; }

		obj.name = base.att_opt("name").unwrap_or_else(|| { println!("Object name not found"); "" }).to_string();

		let script = base.att_opt("script").unwrap_or("").to_string();
		if !script.is_empty()
		{
			obj.script.open_libs();
	
			obj.image.initLua(&mut obj.script);
			obj.text.initLua(&mut obj.script);
			Window::initLua(&mut obj.script);
			Window::getUI().initLua(&mut obj.script);
	
			obj.script.create_table(0, 4);
			obj.script.push_string("setStr"); obj.script.push_fn(Some(Object::setStrFN)); obj.script.set_table(-3);
			obj.script.push_string("getStr"); obj.script.push_fn(Some(Object::getStrFN)); obj.script.set_table(-3);
			obj.script.push_string("setNum"); obj.script.push_fn(Some(Object::setNumFN)); obj.script.set_table(-3);
			obj.script.push_string("getNum"); obj.script.push_fn(Some(Object::getNumFN)); obj.script.set_table(-3);
			obj.script.push_string("name"); obj.script.push_fn(Some(Object::nameFN)); obj.script.set_table(-3);
			obj.script.set_global("object");
			
			obj.script.do_file(&script);
			obj.hasScript = true;
		}

		let order = base.att_opt("order").unwrap_or("itc");
		obj.order = [
			order.chars().nth(0).unwrap_or('i'),
			order.chars().nth(1).unwrap_or('t'),
			order.chars().nth(2).unwrap_or('c')
		];
		
		for node in base.elements()
		{
			let name = node.name().local_part();
			if name == "image" { obj.image = Image::parse(node); }
			if name == "text"
			{
				obj.text.loadFont(
					node.att_opt("font")
						.unwrap_or("")
						.to_string()
				);
				obj.text.setSize(
					node.att_opt("size")
						.unwrap_or("")
						.parse::<u8>()
						.unwrap_or(0)
				);
				obj.text.setString(
					node.text()
						.unwrap_or("")
						.to_string()
				);
			}
			if name == "object" { obj.children.push(Object::parse(node)); }
			if name == "var"
			{
				let name = node.att_opt("name").unwrap_or("").to_string();
				obj.vars.insert(name, Variable
				{
					num: node.att_opt("num").unwrap_or("0").parse::<f32>().unwrap(),
					string: node.att_opt("str").unwrap_or("").to_string()
				});
			}
		}
		
		obj
	}

	fn luaError(&mut self, error: lua::ThreadStatus)
	{
		if error == lua::ThreadStatus::Ok { return; }
		println!("Object: {}\n{}\n", self.name, self.script.to_str(-1).unwrap_or(""));
	}

	pub fn draw(&mut self, shader: &mut super::Shader::Shader)
	{
		crate::ae3d::Window::Window::getUI().scriptExecutor = self;
		if self.hasScript
		{
			if !self.init
			{
				self.init = true;
				self.script.get_global("Init");
				let status = self.script.pcall(0, 0, 0);
				self.luaError(status);
			}
			else
			{
				self.script.get_global("Update");
				let status = self.script.pcall(0, 0, 0);
				self.luaError(status);
			}
		}

		let count = self.children.len();
		match self.order[2]
		{
			'i' => self.image.draw(shader),
			't' => self.text.draw(shader),
			'c' => for i in 0..self.children.len()
			{
				self.children[count - 1 - i].draw(shader);
			}
			_ => {}
		}
		match self.order[1]
		{
			'i' => self.image.draw(shader),
			't' => self.text.draw(shader),
			'c' => for i in 0..self.children.len()
			{
				self.children[count - 1 - i].draw(shader);
			}
			_ => {}
		}
		match self.order[0]
		{
			'i' => self.image.draw(shader),
			't' => self.text.draw(shader),
			'c' => for i in 0..self.children.len()
			{
				self.children[count - 1 - i].draw(shader);
			}
			_ => {}
		}
	}

	pub fn getScript(&mut self) -> &mut lua::State { &mut self.script }
	pub fn getText(&mut self) -> &mut Text { &mut self.text }
}
pub struct UI
{
	root: Object,
	shader: super::Shader::Shader,
	projection: [f32; 16],
	pub scriptExecutor: *mut Object,
	view: Transformable2D,
	loadPath: String
}

impl UI
{
	pub fn new() -> Self
	{
		Self
		{
			root: Object::new(),
			shader: super::Shader::Shader::new(),
			projection: glam::Mat4::IDENTITY.to_cols_array(),
			scriptExecutor: std::ptr::null::<Object>() as *mut Object,
			view: Transformable2D::new(),
			loadPath: String::new()
		}
	}

	pub fn fromFile(path: String) -> Self
	{
		let mut ui = UI::new();
		ui.load(path);
		ui
	}

	pub fn resize(&mut self)
	{
		let winSize = crate::ae3d::Window::Window::getSize();
		self.projection = glam::Mat4::orthographic_rh_gl(
			0.0,
			winSize.x,
			winSize.y,
			0.0,
			0.1,
			100.0
		).to_cols_array();
	}

	pub fn load(&mut self, path: String)
	{
		let src = crate::ae3d::Assets::readXML(path);
		if src.is_none() { return; }

		self.root = Object::parse(&src.unwrap());
		self.view = Transformable2D::new();
		self.scriptExecutor = std::ptr::null::<Object>() as *mut Object;

		if !self.shader.isLoaded()
		{
			self.shader.load("res/shaders/ui.vert".to_string(), "res/shaders/ui.frag".to_string());
		}
	}

	pub fn draw(&mut self)
	{
		if !self.loadPath.is_empty()
		{
			self.load(self.loadPath.clone());
			self.loadPath.clear();
		}
		self.shader.activate();
		self.shader.setMat4("view".to_string(), &self.view.getMatrix().to_cols_array());
		self.shader.setMat4("projection".to_string(), &self.projection);
		self.root.draw(&mut self.shader);
	}

	fn requestReload(&mut self, path: String)
	{
		self.loadPath = path;
	}

	unsafe extern "C" fn loadFileFN(_: *mut std::ffi::c_void) -> i32
	{
		let ui = Window::getUI();
		let path = ui.scriptExecutor.as_mut().unwrap().getScript().to_str(-1).unwrap_or("");
		ui.requestReload(path.to_string());
		0
	}

	pub fn initLua(&mut self, script: &mut lua::State)
	{
		script.create_table(0, 1);

		script.push_string("loadFile"); script.push_fn(Some(UI::loadFileFN)); script.set_table(-3);

		script.set_global("ui");
	}
}
