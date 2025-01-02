use crate::ae3d::Window::Window;

pub struct Image
{
	animation: super::FrameAnimation::Animator,
	vao: u32,
	vbo: u32,
	vertices: [f32; 16],
}

impl Image
{
	pub fn new() -> Self
	{
		Self
		{
			animation: super::FrameAnimation::Animator::new(),
			vao: 0,
			vbo: 0,
			vertices: [0.0; 16],
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
			gl::VertexAttribPointer(
				0, 4, gl::FLOAT,
				gl::FALSE,
				(4 * size_of::<f32>()) as i32,
				std::ptr::null()
			);
		}

		img.animation.load(node.att_opt("anim")
			.unwrap_or_else(|| { println!("No animation provided"); "" })
			.to_string()
		);

		img
	}

	pub fn draw(&mut self, shader: &mut super::Shader::Shader, base: &glam::Mat4)
	{
		self.animation.update();
		let frame = self.animation.getCurrentFrame();
		let size = self.animation.getFrameSize();
		self.vertices = [
			0.0, 0.0,						frame.left(), frame.top(),
			size.x as f32, 0.0,				frame.right(), frame.top(),
			size.x as f32, size.y as f32,	frame.right(), frame.bottom(),
			0.0, size.y as f32,				frame.left(), frame.bottom()
		];

		shader.setInt("tex".to_string(), 0);
		shader.setMat4("model".to_string(), &base.to_cols_array());
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

	pub fn getBounds(&mut self, model: &glam::Mat4) -> sdl2::rect::FRect
	{
		let size = self.animation.getFrameSize();
		let p1 = *model * glam::vec4(0.0, 0.0, 0.0, 1.0);
		let p2 = *model * glam::vec4(size.x as f32, 0.0, 0.0, 1.0);
		let p3 = *model * glam::vec4(size.x as f32, size.y as f32, 0.0, 1.0);
		let p4 = *model * glam::vec4(0.0, size.y as f32, 0.0, 1.0);

		let min = p1.min(p2).min(p3).min(p4);
		let max = p1.max(p2).max(p3).max(p4);

		sdl2::rect::FRect::new(min.x, min.y, max.x - min.x, max.y - min.y)
	}
}

pub struct Object
{
	name: String,
	image: Image,
	text: super::Text::Text,
	position: glam::Vec2,
	angle: f32,
	scale: glam::Vec2,
	origin: glam::Vec2,
	children: Vec<Object>,
	script: lua::State,
	init: bool,
	model: glam::Mat4,
	reloadModel: bool,
	order: [char; 3]
}

impl Object
{
	pub fn new() -> Self
	{
		Self
		{
			name: String::new(),
			image: Image::new(),
			text: super::Text::Text::new(),
			position: glam::Vec2::ZERO,
			angle: 0.0,
			scale: glam::Vec2::ONE,
			origin: glam::Vec2::ZERO,
			children: vec![],
			script: lua::State::new(),
			init: false,
			model: glam::Mat4::IDENTITY,
			reloadModel: true,
			order: ['i', 't', 'c']
		}
	}

	unsafe extern "C" fn setPositionFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI().scriptExecutor.as_mut().unwrap();
		let x = obj.script.to_number(-2) as f32;
		let y = obj.script.to_number(-1) as f32;
		obj.position = glam::vec2(x, y);
		obj.reloadModel = true;
		0
	}

	unsafe extern "C" fn setScaleFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI().scriptExecutor.as_mut().unwrap();
		let x = obj.script.to_number(-2) as f32;
		let y = obj.script.to_number(-1) as f32;
		obj.scale = glam::vec2(x, y);
		obj.reloadModel = true;
		0
	}

	unsafe extern "C" fn setOriginFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI().scriptExecutor.as_mut().unwrap();
		let x = obj.script.to_number(-2) as f32;
		let y = obj.script.to_number(-1) as f32;
		obj.origin = glam::vec2(x, y);
		obj.reloadModel = true;
		0
	}

	unsafe extern "C" fn setAngleFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI().scriptExecutor.as_mut().unwrap();
		obj.angle = obj.script.to_number(-1) as f32;
		obj.reloadModel = true;
		0
	}

	unsafe extern "C" fn translateFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI().scriptExecutor.as_mut().unwrap();
		let x = obj.script.to_number(-2) as f32;
		let y = obj.script.to_number(-1) as f32;
		obj.position += glam::vec2(x, y);
		obj.reloadModel = true;
		0
	}

	unsafe extern "C" fn rotateFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI().scriptExecutor.as_mut().unwrap();
		obj.angle += obj.script.to_number(-1) as f32;
		obj.reloadModel = true;
		0
	}

	unsafe extern "C" fn scaleFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI().scriptExecutor.as_mut().unwrap();
		let x = obj.script.to_number(-2) as f32;
		let y = obj.script.to_number(-1) as f32;
		obj.scale *= glam::vec2(x, y);
		obj.reloadModel = true;
		0
	}

	unsafe extern "C" fn getPositionFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI().scriptExecutor.as_mut().unwrap();
		obj.script.push_number(obj.position.x as f64);
		obj.script.push_number(obj.position.y as f64);
		2
	}

	unsafe extern "C" fn getScaleFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI().scriptExecutor.as_mut().unwrap();
		obj.script.push_number(obj.scale.x as f64);
		obj.script.push_number(obj.scale.y as f64);
		2
	}

	unsafe extern "C" fn getOriginFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI().scriptExecutor.as_mut().unwrap();
		obj.script.push_number(obj.origin.x as f64);
		obj.script.push_number(obj.origin.y as f64);
		2
	}

	unsafe extern "C" fn getAngleFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI().scriptExecutor.as_mut().unwrap();
		obj.script.push_number(obj.angle as f64);
		2
	}
	
	unsafe extern "C" fn imageBoundsFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI().scriptExecutor.as_mut().unwrap();
		let b = obj.image.getBounds(&obj.model);
		obj.script.push_number(b.x() as f64);
		obj.script.push_number(b.y() as f64);
		obj.script.push_number(b.width() as f64);
		obj.script.push_number(b.height() as f64);
		4
	}
	
	unsafe extern "C" fn textBoundsFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI().scriptExecutor.as_mut().unwrap();
		let b = obj.text.getBounds(&obj.model);
		obj.script.push_number(b.x() as f64);
		obj.script.push_number(b.y() as f64);
		obj.script.push_number(b.width() as f64);
		obj.script.push_number(b.height() as f64);
		4
	}

	unsafe extern "C" fn deltaTimeFN(_: *mut std::ffi::c_void) -> i32
	{
		crate::ae3d::Window::Window::getUI()
			.scriptExecutor
			.as_mut()
			.unwrap()
			.script
			.push_number(Window::getDeltaTime() as f64);
		1
	}

	unsafe extern "C" fn winSizeFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI()
			.scriptExecutor
			.as_mut()
			.unwrap();
		obj.script.push_number(Window::getSize().x as f64);
		obj.script.push_number(Window::getSize().y as f64);
		2
	}

	unsafe extern "C" fn imageSizeFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI()
			.scriptExecutor
			.as_mut()
			.unwrap();
		obj.script.push_number(obj.image.animation.getFrameSize().x as f64);
		obj.script.push_number(obj.image.animation.getFrameSize().y as f64);
		2
	}

	unsafe extern "C" fn mousePosFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI()
			.scriptExecutor
			.as_mut()
			.unwrap();
		obj.script.push_number(Window::getMousePos().x as f64);
		obj.script.push_number(Window::getMousePos().y as f64);
		2
	}

	unsafe extern "C" fn mousePressedFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI()
			.scriptExecutor
			.as_mut()
			.unwrap();
		let btn: sdl2::mouse::MouseButton;
		{
			btn = match obj.script.to_str(-1).unwrap_or("")
			{
				"Left" => { sdl2::mouse::MouseButton::Left },
				"Right" => { sdl2::mouse::MouseButton::Right },
				"Middle" => { sdl2::mouse::MouseButton::Middle },
				"X1" => { sdl2::mouse::MouseButton::X1 },
				"X2" => { sdl2::mouse::MouseButton::X2 }
				_ => { sdl2::mouse::MouseButton::Unknown },
			}
		}
		obj.script.push_bool(crate::ae3d::Window::Window::isMousePressed(btn));
		1
	}

	unsafe extern "C" fn mouseJustPressedFN(_: *mut std::ffi::c_void) -> i32
	{
		let event = crate::ae3d::Window::Window::getMouseEvent();
		let obj = crate::ae3d::Window::Window::getUI()
			.scriptExecutor
			.as_mut()
			.unwrap();
		if event.is_none() { obj.script.push_bool(false); return 1; }
		let btn: sdl2::mouse::MouseButton;
		{
			btn = match obj.script.to_str(-1).unwrap_or("")
			{
				"Left" => { sdl2::mouse::MouseButton::Left },
				"Right" => { sdl2::mouse::MouseButton::Right },
				"Middle" => { sdl2::mouse::MouseButton::Middle },
				"X1" => { sdl2::mouse::MouseButton::X1 },
				"X2" => { sdl2::mouse::MouseButton::X2 }
				_ => { sdl2::mouse::MouseButton::Unknown },
			}
		}
		obj.script.push_bool(event.unwrap().btn == btn);
		1
	}

	unsafe extern "C" fn keyPressedFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI()
			.scriptExecutor
			.as_mut()
			.unwrap();
		let name: String;
		{
			name = obj.script.to_str(-1).unwrap().to_string();
		}
		obj.script.push_bool(crate::ae3d::Window::Window::isKeyPressed(
			sdl2::keyboard::Scancode::from_name(&name).unwrap()
		));
		1
	}

	unsafe extern "C" fn keyJustPressedFN(_: *mut std::ffi::c_void) -> i32
	{
		let event = crate::ae3d::Window::Window::getKeyEvent();
		let obj = crate::ae3d::Window::Window::getUI()
			.scriptExecutor
			.as_mut()
			.unwrap();
		if event.is_none() { obj.script.push_bool(false); return 1; }
		let name: String;
		{
			name = obj.script.to_str(-1).unwrap().to_string();
		}
		obj.script.push_bool(
			event.unwrap().key == sdl2::keyboard::Scancode::from_name(&name).unwrap() &&
			(
				event.unwrap().action == crate::ae3d::Window::KeyAction::Pressed ||
				event.unwrap().action == crate::ae3d::Window::KeyAction::PressedRepeat
			)
		);
		1
	}

	unsafe extern "C" fn getWindowNumFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI()
			.scriptExecutor
			.as_mut()
			.unwrap();
		let name: String;
		{
			name = obj.script.to_str(-1).unwrap().to_string();
		}
		obj.script.push_number(crate::ae3d::Window::Window::getVariable(name).num as f64);
		1
	}

	unsafe extern "C" fn getWindowStrFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI()
			.scriptExecutor
			.as_mut()
			.unwrap();
		let name: String;
		{
			name = obj.script.to_str(-1).unwrap().to_string();
		}
		obj.script.push_string(&crate::ae3d::Window::Window::getVariable(name).string);
		1
	}

	unsafe extern "C" fn textSetStringFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI()
			.scriptExecutor
			.as_mut()
			.unwrap();
		let text = obj.script.to_str(-1).unwrap().to_string();
		obj.text.setString(text);
		0
	}

	unsafe extern "C" fn textGetStringFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI()
			.scriptExecutor
			.as_mut()
			.unwrap();
		obj.script.push_string(obj.text.getString().as_str());
		1
	}

	unsafe extern "C" fn textSizeFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI()
			.scriptExecutor
			.as_mut()
			.unwrap();
		let size = obj.text.getDimensions();
		obj.script.push_number(size.x as f64);
		obj.script.push_number(size.y as f64);
		2
	}

	unsafe extern "C" fn imageSetFN(_: *mut std::ffi::c_void) -> i32
	{
		let obj = crate::ae3d::Window::Window::getUI()
			.scriptExecutor
			.as_mut()
			.unwrap();
		let anim = obj.script.to_str(-1).unwrap_or("");
		obj.image.animation.setCurrentAnimation(anim.to_string());
		0
	}

	pub fn parse(base: &spex::xml::Element) -> Self
	{
		let mut obj = Object::new();
		if base.name().local_part() != "object" { return obj; }

		obj.name = base.att_opt("name").unwrap_or_else(|| { println!("Object name not found"); "" }).to_string();

		obj.script.open_libs();
		obj.script.create_table(0, 12);
		obj.script.push_string("setPosition");
		obj.script.push_fn(Some(Object::setPositionFN));
		obj.script.set_table(-3);
		obj.script.push_string("setAngle");
		obj.script.push_fn(Some(Object::setAngleFN));
		obj.script.set_table(-3);
		obj.script.push_string("setScale");
		obj.script.push_fn(Some(Object::setScaleFN));
		obj.script.set_table(-3);
		obj.script.push_string("setOrigin");
		obj.script.push_fn(Some(Object::setOriginFN));
		obj.script.set_table(-3);
		obj.script.push_string("translate");
		obj.script.push_fn(Some(Object::translateFN));
		obj.script.set_table(-3);
		obj.script.push_string("scale");
		obj.script.push_fn(Some(Object::scaleFN));
		obj.script.set_table(-3);
		obj.script.push_string("rotate");
		obj.script.push_fn(Some(Object::rotateFN));
		obj.script.set_table(-3);
		obj.script.push_string("getPosition");
		obj.script.push_fn(Some(Object::getPositionFN));
		obj.script.set_table(-3);
		obj.script.push_string("getScale");
		obj.script.push_fn(Some(Object::getScaleFN));
		obj.script.set_table(-3);
		obj.script.push_string("getAngle");
		obj.script.push_fn(Some(Object::getAngleFN));
		obj.script.set_table(-3);
		obj.script.push_string("getOrigin");
		obj.script.push_fn(Some(Object::getOriginFN));
		obj.script.set_table(-3);
		obj.script.set_global("transform");

		obj.script.create_table(0, 9);
		obj.script.push_string("dt");
		obj.script.push_fn(Some(Object::deltaTimeFN));
		obj.script.set_table(-3);
		obj.script.push_string("size");
		obj.script.push_fn(Some(Object::winSizeFN));
		obj.script.set_table(-3);
		obj.script.push_string("mousePos");
		obj.script.push_fn(Some(Object::mousePosFN));
		obj.script.set_table(-3);
		obj.script.push_string("mousePressed");
		obj.script.push_fn(Some(Object::mousePressedFN));
		obj.script.set_table(-3);
		obj.script.push_string("mouseJustPressed");
		obj.script.push_fn(Some(Object::mouseJustPressedFN));
		obj.script.set_table(-3);
		obj.script.push_string("keyPressed");
		obj.script.push_fn(Some(Object::keyPressedFN));
		obj.script.set_table(-3);
		obj.script.push_string("keyJustPressed");
		obj.script.push_fn(Some(Object::keyJustPressedFN));
		obj.script.set_table(-3);
		obj.script.push_string("getNum");
		obj.script.push_fn(Some(Object::getWindowNumFN));
		obj.script.set_table(-3);
		obj.script.push_string("getStr");
		obj.script.push_fn(Some(Object::getWindowStrFN));
		obj.script.set_table(-3);
		obj.script.set_global("window");

		// Implement all input functions

		obj.script.create_table(0, 2);
		obj.script.push_string("size");
		obj.script.push_fn(Some(Object::imageSizeFN));
		obj.script.set_table(-3);
		obj.script.push_string("bounds");
		obj.script.push_fn(Some(Object::imageBoundsFN));
		obj.script.set_table(-3);
		obj.script.set_global("image");

		obj.script.create_table(0, 4);
		obj.script.push_string("bounds");
		obj.script.push_fn(Some(Object::textBoundsFN));
		obj.script.set_table(-3);
		obj.script.push_string("setString");
		obj.script.push_fn(Some(Object::textSetStringFN));
		obj.script.set_table(-3);
		obj.script.push_string("getString");
		obj.script.push_fn(Some(Object::textGetStringFN));
		obj.script.set_table(-3);
		obj.script.push_string("size");
		obj.script.push_fn(Some(Object::textSizeFN));
		obj.script.set_table(-3);
		obj.script.set_global("text");
		
		obj.script.do_file(base.att_opt("script").unwrap_or(""));

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
		}
		
		obj
	}

	pub fn draw(&mut self, shader: &mut super::Shader::Shader, base: &glam::Mat4)
	{
		crate::ae3d::Window::Window::getUI().scriptExecutor = self;
		if !self.init
		{
			self.init = true;
			self.script.get_global("Init");
			self.script.pcall(0, 0, 0);
		}
		else
		{
			self.script.get_global("Update");
			self.script.pcall(0, 0, 0);
		}

		if self.reloadModel { self.updateModel(base); }

		let count = self.children.len();
		match self.order[2]
		{
			'i' => self.image.draw(shader, &self.model),
			't' => self.text.draw(shader, &self.model),
			'c' => for i in 0..self.children.len()
			{
				self.children[count - 1 - i].draw(shader, &self.model);
			}
			_ => {}
		}
		match self.order[1]
		{
			'i' => self.image.draw(shader, &self.model),
			't' => self.text.draw(shader, &self.model),
			'c' => for i in 0..self.children.len()
			{
				self.children[count - 1 - i].draw(shader, &self.model);
			}
			_ => {}
		}
		match self.order[0]
		{
			'i' => self.image.draw(shader, &self.model),
			't' => self.text.draw(shader, &self.model),
			'c' => for i in 0..self.children.len()
			{
				self.children[count - 1 - i].draw(shader, &self.model);
			}
			_ => {}
		}
	}

	pub fn getScript(&mut self) -> &mut lua::State { &mut self.script }

	pub fn updateModel(&mut self, base: &glam::Mat4)
	{
		self.reloadModel = false;
		self.model = glam::Mat4::mul_mat4(
			&base,
			&glam::Mat4::from_translation(glam::vec3(self.position.x, self.position.y, 0.0))
		);
		self.model = glam::Mat4::mul_mat4(&self.model, &glam::Mat4::from_scale(glam::vec3(self.scale.x, self.scale.y, 1.0)));
		self.model = glam::Mat4::mul_mat4(&self.model, &glam::Mat4::from_rotation_z(self.angle.to_radians()));
		self.model = glam::Mat4::mul_mat4(&self.model, &glam::Mat4::from_translation(-glam::vec3(self.origin.x, self.origin.y, 0.0)));
	}
}

pub struct UI
{
	root: Object,
	shader: super::Shader::Shader,
	projection: [f32; 16],
	view: [f32; 16],
	position: glam::Vec2,
	angle: f32,
	scale: glam::Vec2,
	reloadView: bool,
	pub scriptExecutor: *mut Object
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
			view: glam::Mat4::IDENTITY.to_cols_array(),
			position: glam::Vec2::ZERO,
			angle: 0.0,
			scale: glam::Vec2::ONE,
			reloadView: true
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

		self.shader.load("res/shaders/ui.vert".to_string(), "res/shaders/ui.frag".to_string());
	}

	fn updateView(&mut self)
	{
		let mut matrix = glam::Mat4::from_translation(glam::vec3(self.position.x, self.position.y, 0.0));
		matrix = glam::Mat4::mul_mat4(&matrix, &glam::Mat4::from_scale(glam::vec3(self.scale.x, self.scale.y, 1.0)));
		matrix = glam::Mat4::mul_mat4(&matrix, &glam::Mat4::from_rotation_z(self.angle.to_radians()));
		self.view = matrix.to_cols_array();
		self.reloadView = false;
	}

	pub fn draw(&mut self)
	{
		self.shader.activate();
		self.shader.setMat4("view".to_string(), &self.view);
		self.shader.setMat4("projection".to_string(), &self.projection);
		self.root.draw(&mut self.shader, &glam::Mat4::IDENTITY);
	}
}
