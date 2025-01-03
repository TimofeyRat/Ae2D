use std::time::Instant;

#[derive(Clone, Copy, Debug)]
pub enum KeyAction
{
	Pressed = 0,
	Released = 1,
	PressedRepeat = 2,
	ReleasedRepeat = 3
}

impl PartialEq for KeyAction
{
	fn eq(&self, other: &Self) -> bool
	{
		*self as i32 == *other as i32
	}
}

#[derive(Clone, Copy, Debug)]
pub struct KeyEvent
{
	pub key: sdl2::keyboard::Scancode,
	pub mods: sdl2::keyboard::Mod,
	pub action: KeyAction
}

#[derive(Clone, Copy)]
pub struct MouseEvent
{
	pub btn: sdl2::mouse::MouseButton,
	pub clicks: u8,
	pub pos: glam::Vec2
}

pub struct Color
{
	name: String,
	value: sdl2::pixels::Color
}

pub struct Window
{
	context: sdl2::Sdl,
	video: sdl2::VideoSubsystem,
	window: Option<sdl2::video::Window>,
	events: sdl2::EventPump,
	running: bool,
	clearColor: sdl2::pixels::Color,
	timer: sdl2::TimerSubsystem,
	keyEvent: Option<KeyEvent>,
	mouseEvent: Option<MouseEvent>,
	ttfContext: sdl2::ttf::Sdl2TtfContext,
	palette: Vec<Color>,
    gl: Option<sdl2::video::GLContext>,
	textureCreator: Option<sdl2::render::TextureCreator<sdl2::surface::SurfaceContext<'static>>>,
	tcCanvas: sdl2::render::SurfaceCanvas<'static>,
	deltaTime: f32,
	lastTime: std::time::Instant,
	mouse: sdl2::mouse::MouseUtil,
	lockCursor: bool,
	mouseDelta: glam::Vec2,
	ui: super::graphics::UI::UI,
	vars: super::Programmable::Programmable
}

impl Window
{
	pub fn default() -> Window
	{
		let c = sdl2::init().expect("Failed to initialize SDL");
		Window
		{
			context: c.clone(),
			video: c.video().unwrap(),
			window: None,
			events: c.event_pump().unwrap(),
			running: true,
			clearColor: sdl2::pixels::Color::BLACK,
			deltaTime: 0.0,
			lastTime: Instant::now(),
			timer: c.timer().unwrap(),
			keyEvent: None,
			mouseEvent: None,
			ttfContext: sdl2::ttf::init().expect("Failed to initialize TTF"),
			palette: Vec::new(),
            gl: None,
			textureCreator: None,
			tcCanvas: sdl2::surface::Surface::new(1, 1, sdl2::pixels::PixelFormatEnum::RGBA8888).unwrap().into_canvas().unwrap(),
			mouse: c.mouse(),
			lockCursor: false,
			mouseDelta: glam::Vec2::ZERO,
			ui: super::graphics::UI::UI::new(),
			vars: std::collections::HashMap::new()
		}
	}

	fn getInstance() -> &'static mut Window
	{
		static mut INSTANCE: Option<Window> = None;
		
		unsafe
		{
			if INSTANCE.is_none() { INSTANCE = Some(Window::default()); }
			INSTANCE.as_mut().expect("Window singleton is not initialized")
		}
	}
	
	pub fn init()
	{
		let f = super::Assets::readJSON("res/global/config.json".to_string());
		if f.is_none() { return }

		let mut title = String::from("");
		let mut size = glam::Vec2::ZERO;
		let mut style = String::from("");
		let mut pos = glam::Vec2::splat(-127.0);
		let mut hideCursor = false;
		let mut lockCursor = false;
		let mut vsync = true;
		
		let i = Window::getInstance();

		for section in f.unwrap().entries()
		{
			if section.0 == "init"
			{
				for attr in section.1.entries()
				{
					if attr.0 == "title" { title = attr.1.as_str().unwrap().to_string(); }
					if attr.0 == "style" { style = attr.1.as_str().unwrap().to_string(); }
					if attr.0 == "size"
					{
						for dim in attr.1.entries()
						{
							if dim.0 == "w" { size.x = dim.1.as_f32().unwrap(); }
							if dim.0 == "h" { size.y = dim.1.as_f32().unwrap(); }
						}
					}
				}
			}
			if section.0 == "optional"
			{
				for attr in section.1.entries()
				{
					if attr.0 == "position"
					{
						for dim in attr.1.entries()
						{
							if dim.0 == "x" { pos.x = dim.1.as_f32().unwrap(); }
							if dim.0 == "y" { pos.y = dim.1.as_f32().unwrap(); }
						}
					}
					if attr.0 == "hideCursor" { hideCursor = attr.1.as_bool().unwrap_or(false); }
					if attr.0 == "lockCursor" { lockCursor = attr.1.as_bool().unwrap_or(false); }
					if attr.0 == "vsync" { vsync = attr.1.as_bool().unwrap_or(true); }
				}
			}
			if section.0 == "custom"
			{
				for var in section.1.members()
				{
					let mut name = String::new();
					let mut value = super::Programmable::Variable::new();
					for attr in var.entries()
					{
						if attr.0 == "name" { name = attr.1.as_str().unwrap().to_string(); }
						if attr.0 == "num" { value.num = attr.1.as_f32().unwrap(); }
						if attr.0 == "str" { value.string = attr.1.as_str().unwrap().to_string(); }
					}
					i.vars.insert(name, value);
				}
			}
		}


		let attr = i.video.gl_attr();
		attr.set_context_profile(sdl2::video::GLProfile::Core);
		// attr.set_context_version(3, 3);
		attr.set_context_version(2, 0);
		attr.set_depth_size(24);

		let mut builder = i.video.window(title.as_str(), size.x as u32, size.y as u32);

		if pos != glam::Vec2::splat(-127.0) { builder.position(pos.x as i32, pos.y as i32); }
		else { builder.position_centered(); }
		if style.as_str() == "resizable" { builder.resizable(); }
		if style.as_str() == "borderless" { builder.borderless(); }
		if style.as_str() == "fullscreen" { builder.fullscreen_desktop(); }

		i.window = Some(builder.opengl().build().unwrap());

		i.gl = Some(i.window.as_mut().unwrap().gl_create_context().unwrap());
		gl::load_with(|name| i.video.gl_get_proc_address(name) as *const _);
		
		i.video.gl_set_swap_interval(if vsync { sdl2::video::SwapInterval::VSync } else { sdl2::video::SwapInterval::Immediate });
		i.mouse.show_cursor(!hideCursor);
		i.lockCursor = lockCursor;

		unsafe
		{
			gl::Enable(gl::DEPTH_TEST);
			gl::Enable(gl::BLEND);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
			gl::DepthFunc(gl::LESS);
			let size = i.window.as_mut().unwrap().size();
			gl::Viewport(0, 0, size.0 as i32, size.1 as i32);
		}

		i.textureCreator = Some(i.tcCanvas.texture_creator());

		Window::loadColors();
		
		i.ui.load("res/ui/mainMenu.xml".to_string());
	}

	pub fn loadColors()
	{
		let palette = &mut Window::getInstance().palette;

		let f = super::Assets::readJSON("res/global/colors.json".to_string());
		if f.is_none() { return; }

		for color in f.unwrap().entries()
		{
			let mut c = Color
			{
				name: String::from(color.0),
				value: sdl2::pixels::Color::RGBA(0, 0, 0, 255)
			};
			for v in color.1.entries()
			{
				if v.0 == "r" { c.value.r = v.1.as_u8().unwrap_or(0); }
				if v.0 == "g" { c.value.g = v.1.as_u8().unwrap_or(0); }
				if v.0 == "b" { c.value.b = v.1.as_u8().unwrap_or(0); }
				if v.0 == "a" { c.value.a = v.1.as_u8().unwrap_or(255); }
			}
			palette.push(c);
		}
	}

	pub fn update()
	{
		let i = Window::getInstance();
		i.keyEvent = None;
		i.mouseEvent = None;
		i.mouseDelta = glam::Vec2::ZERO;

		i.deltaTime = i.lastTime.elapsed().as_secs_f32();
		i.lastTime = std::time::Instant::now();

		for event in i.events.poll_iter()
		{
			match event
			{
				sdl2::event::Event::Quit {..} => { i.running = false; }
				sdl2::event::Event::KeyDown { scancode, keymod, repeat, .. } =>
				{
					i.keyEvent = Some(KeyEvent
					{
						key: scancode.unwrap(),
						mods: keymod,
						action: if repeat { KeyAction::PressedRepeat } else { KeyAction::Pressed }
					});
				},
				sdl2::event::Event::KeyUp { scancode, keymod, repeat, .. } =>
				{
					i.keyEvent = Some(KeyEvent
					{
						key: scancode.unwrap(),
						mods: keymod,
						action: if repeat { KeyAction::ReleasedRepeat } else { KeyAction::Released }
					});
				},
				sdl2::event::Event::MouseButtonDown { mouse_btn, clicks, x, y, .. } =>
				{
					i.mouseEvent = Some(MouseEvent
					{
						btn: mouse_btn,
						clicks,
						pos: glam::vec2(x as f32, y as f32)
					});
				},
				sdl2::event::Event::MouseButtonUp { mouse_btn, x, y, .. } =>
				{
					i.mouseEvent = Some(MouseEvent
					{
						btn: mouse_btn,
						clicks: 0,
						pos: glam::vec2(x as f32, y as f32)
					});
				},
				sdl2::event::Event::Window { win_event, .. } =>
				{
					match win_event
					{
						sdl2::event::WindowEvent::Resized(x, y) =>
						{
							unsafe { gl::Viewport(0, 0, x, y); }
						},
						sdl2::event::WindowEvent::Maximized =>
						{
							unsafe { gl::Viewport(0, 0, Window::getSize().x as i32, Window::getSize().y as i32); }
						},
						_ => {}
					}
					i.ui.resize();
				},
				sdl2::event::Event::MouseMotion { x, y, xrel, yrel, .. } =>
				{
					if x == Window::getSize().x as i32 / 2 && y == Window::getSize().y as i32 / 2 { continue; }
					i.mouseDelta = glam::vec2(xrel as f32, yrel as f32);
				},
				_ => {}
			}
		}

		if i.lockCursor { i.context.mouse().warp_mouse_in_window(
			i.window.as_ref().unwrap(),
			Window::getSize().x as i32 / 2,
			Window::getSize().y as i32 / 2
		); }
	}

	pub fn clear()
	{
		let i = Window::getInstance();
		
        unsafe
        {
            let c = Window::toGLcolor(i.clearColor);
            gl::ClearColor(c.0, c.1, c.2, c.3);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
	
	pub fn getTC() -> &'static mut sdl2::render::TextureCreator<sdl2::surface::SurfaceContext<'static>>
	{
		Window::getInstance().textureCreator.as_mut().unwrap()
	}

	pub fn setSize(size: glam::Vec2)
	{
		Window::getInstance().window.as_mut().unwrap().set_size(
			size.x as u32,
			size.y as u32
		);
	}

	pub fn getSize() -> glam::Vec2
	{
		let size = Window::getInstance().window.as_mut().unwrap().size();
		glam::vec2(size.0 as f32, size.1 as f32)
	}

	pub fn isKeyPressed(key: sdl2::keyboard::Scancode) -> bool
	{
		Window::getInstance().events.keyboard_state().is_scancode_pressed(key)
	}

	pub fn isMousePressed(btn: sdl2::mouse::MouseButton) -> bool
	{
		Window::getInstance().events.mouse_state().is_mouse_button_pressed(btn)
	}

	pub fn getColor(name: String) -> sdl2::pixels::Color
	{
		for c in Window::getInstance().palette.iter()
		{
			if c.name == name { return c.value }
		}
		sdl2::pixels::Color::RGBA(0, 0, 0,0)
	}

    pub fn toGLcolor(clr: sdl2::pixels::Color) -> (f32, f32, f32, f32)
    {
        (
            clr.r as f32 / 255.0,
            clr.g as f32 / 255.0,
            clr.b as f32 / 255.0,
            clr.a as f32 / 255.0
        )
    }

    pub fn display()
    {
        let i = Window::getInstance();
        i.window.as_mut().unwrap().gl_swap_window();
    }

	pub fn getTTF() -> &'static mut sdl2::ttf::Sdl2TtfContext { &mut Window::getInstance().ttfContext }
	pub fn getKeyEvent() -> Option<KeyEvent> { Window::getInstance().keyEvent }
	pub fn getMouseEvent() -> Option<MouseEvent> { Window::getInstance().mouseEvent }
	pub fn setClearColor(clr: sdl2::pixels::Color) { Window::getInstance().clearColor = clr; }
	pub fn isOpen() -> bool { Window::getInstance().running }
	pub fn close() { Window::getInstance().running = false; }
	pub fn getDeltaTime() -> f32 { Window::getInstance().deltaTime }
    pub fn getContext() -> &'static mut sdl2::video::GLContext { Window::getInstance().gl.as_mut().unwrap() }
	pub fn getMouseDelta() -> glam::Vec2 { Window::getInstance().mouseDelta }
	pub fn getUI() -> &'static mut super::graphics::UI::UI { &mut Window::getInstance().ui }

	pub fn getGL() -> String
	{
		unsafe
		{
			let v = gl::GetString(gl::VERSION);
			let mut size: isize = 0;
			let mut vector: Vec<u8> = vec![];
			while v.offset(size).read() != 0
			{
				vector.push(v.offset(size).read());
				size += 1;
			}
			String::from_utf8(vector).unwrap()
		}
	}

	pub fn getMousePos() -> glam::IVec2
	{
		let s = Window::getInstance().events.mouse_state();
		glam::ivec2(s.x(), s.y())
	}

	pub fn getVariable(name: String) -> super::Programmable::Variable
	{
		Window::getInstance().vars[&name].clone()
	}

	unsafe extern "C" fn sizeFN(_: *mut std::ffi::c_void) -> i32
	{
		let script = Window::getUI().scriptExecutor.as_mut().unwrap().getScript();
		script.push_number(Window::getSize().x as f64);
		script.push_number(Window::getSize().y as f64);
		2
	}

	unsafe extern "C" fn dtFN(_: *mut std::ffi::c_void) -> i32
	{
		Window::getUI().scriptExecutor.as_mut().unwrap().getScript().push_number(Window::getDeltaTime() as f64);
		1
	}

	unsafe extern "C" fn getNumFN(_: *mut std::ffi::c_void) -> i32
	{
		let script = Window::getUI().scriptExecutor.as_mut().unwrap().getScript();
		let var = Window::getVariable(script.to_str(-1).unwrap_or("").to_string());
		script.push_number(var.num as f64);
		1
	}

	unsafe extern "C" fn getStrFN(_: *mut std::ffi::c_void) -> i32
	{
		let script = Window::getUI().scriptExecutor.as_mut().unwrap().getScript();
		let var = Window::getVariable(script.to_str(-1).unwrap_or("").to_string());
		script.push_string(&var.string);
		1
	}

	unsafe extern "C" fn mousePosFN(_: *mut std::ffi::c_void) -> i32
	{
		let script = Window::getUI().scriptExecutor.as_mut().unwrap().getScript();
		script.push_number(Window::getMousePos().x as f64);
		script.push_number(Window::getMousePos().y as f64);
		2
	}

	unsafe extern "C" fn mousePressedFN(_: *mut std::ffi::c_void) -> i32
	{
		let script = Window::getUI().scriptExecutor.as_mut().unwrap().getScript();
		let btn = match script.to_str(-1).unwrap_or("")
		{
			"Left" => sdl2::mouse::MouseButton::Left,
			"Right" => sdl2::mouse::MouseButton::Right,
			"Middle" => sdl2::mouse::MouseButton::Middle,
			"X1" => sdl2::mouse::MouseButton::X1,
			"X2" => sdl2::mouse::MouseButton::X2,
			_ => sdl2::mouse::MouseButton::Unknown
		};
		script.push_bool(Window::isMousePressed(btn));
		1
	}

	unsafe extern "C" fn mouseJustPressedFN(_: *mut std::ffi::c_void) -> i32
	{
		let script = Window::getUI().scriptExecutor.as_mut().unwrap().getScript();
		if Window::getMouseEvent().is_none() { script.push_bool(false); return 1; }
		let btn = match script.to_str(-1).unwrap_or("")
		{
			"Left" => sdl2::mouse::MouseButton::Left,
			"Right" => sdl2::mouse::MouseButton::Right,
			"Middle" => sdl2::mouse::MouseButton::Middle,
			"X1" => sdl2::mouse::MouseButton::X1,
			"X2" => sdl2::mouse::MouseButton::X2,
			_ => sdl2::mouse::MouseButton::Unknown
		};
		let e = Window::getMouseEvent().unwrap();
		script.push_bool(e.btn == btn && e.clicks > 0);
		1
	}

	unsafe extern "C" fn keyPressedFN(_: *mut std::ffi::c_void) -> i32
	{
		let script = Window::getUI().scriptExecutor.as_mut().unwrap().getScript();
		let scancode = sdl2::keyboard::Scancode::from_name(script.to_str(-1)
			.unwrap_or(""))
			.unwrap_or(sdl2::keyboard::Scancode::SysReq);
		script.push_bool(Window::isKeyPressed(scancode));
		1
	}

	unsafe extern "C" fn keyJustPressedFN(_: *mut std::ffi::c_void) -> i32
	{
		let script = Window::getUI().scriptExecutor.as_mut().unwrap().getScript();
		if Window::getKeyEvent().is_none() { script.push_bool(false); return 1; }
		let scancode = sdl2::keyboard::Scancode::from_name(script.to_str(-1)
			.unwrap_or(""))
			.unwrap_or(sdl2::keyboard::Scancode::SysReq);
		let e = Window::getKeyEvent().unwrap();
		script.push_bool(e.key == scancode && (e.action == KeyAction::Pressed || e.action == KeyAction::PressedRepeat));
		1
	}

	unsafe extern "C" fn closeFN(_: *mut std::ffi::c_void) -> i32 { Window::close(); 0 }

	pub fn initLua(script: &mut lua::State)
	{
		script.create_table(0, 10);

		script.push_string("size"); script.push_fn(Some(Window::sizeFN)); script.set_table(-3);
		script.push_string("dt"); script.push_fn(Some(Window::dtFN)); script.set_table(-3);
		script.push_string("getNum"); script.push_fn(Some(Window::getNumFN)); script.set_table(-3);
		script.push_string("getStr"); script.push_fn(Some(Window::getStrFN)); script.set_table(-3);
		script.push_string("mousePos"); script.push_fn(Some(Window::mousePosFN)); script.set_table(-3);
		script.push_string("mousePressed"); script.push_fn(Some(Window::mousePressedFN)); script.set_table(-3);
		script.push_string("mouseJustPressed"); script.push_fn(Some(Window::mouseJustPressedFN)); script.set_table(-3);
		script.push_string("close"); script.push_fn(Some(Window::closeFN)); script.set_table(-3);
		script.push_string("keyPressed"); script.push_fn(Some(Window::keyPressedFN)); script.set_table(-3);
		script.push_string("keyJustPressed"); script.push_fn(Some(Window::keyJustPressedFN)); script.set_table(-3);

		script.set_global("window");
	}
}
