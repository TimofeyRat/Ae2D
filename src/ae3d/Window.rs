use sdl2::surface::Surface;

use super::{math::Point::Point, Assets};

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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
	pub pos: Point
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
	deltaTime: f32,
	currentTime: f32,
	lastTime: f32,
	timer: sdl2::TimerSubsystem,
	keyEvent: Option<KeyEvent>,
	mouseEvent: Option<MouseEvent>,
	ttfContext: sdl2::ttf::Sdl2TtfContext,
	palette: Vec<Color>,
    gl: Option<sdl2::video::GLContext>,
	textureCreator: Option<sdl2::render::TextureCreator<sdl2::surface::SurfaceContext<'static>>>,
	tcCanvas: sdl2::render::SurfaceCanvas<'static>
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
			currentTime: 0.0,
			lastTime: 0.0,
			timer: c.timer().unwrap(),
			keyEvent: None,
			mouseEvent: None,
			ttfContext: sdl2::ttf::init().expect("Failed to initialize TTF"),
			palette: Vec::new(),
            gl: None,
			textureCreator: None,
			tcCanvas: Surface::new(1, 1, sdl2::pixels::PixelFormatEnum::RGBA8888).unwrap().into_canvas().unwrap()
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
		let f = Assets::readJSON("res/global/config.json".to_string());
		if f.is_none() { return }

		let mut title = String::from("");
		let mut size = Point::zero();
		let mut style = String::from("");
		let mut pos = Point::num(-127.0);

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
				}
			}
			if section.0 == "custom" {}
		}

		let i = Window::getInstance();

		let attr = i.video.gl_attr();
		attr.set_context_profile(sdl2::video::GLProfile::Core);
		attr.set_context_version(3, 3);

		let mut builder = i.video.window(title.as_str(), size.x as u32, size.y as u32);

		if pos != Point::num(-127.0) { builder.position(pos.x as i32, pos.y as i32); }
		else { builder.position_centered(); }
		if style.as_str() == "resizable" { builder.resizable(); }
		if style.as_str() == "borderless" { builder.borderless(); }
		if style.as_str() == "fullscreen" { builder.fullscreen_desktop(); }

		i.window = Some(builder.opengl().build().unwrap());

		i.lastTime = i.timer.performance_counter() as f32;
		i.currentTime = i.lastTime + 1.0;

		i.gl = Some(i.window.as_mut().unwrap().gl_create_context().unwrap());
		gl::load_with(|name| i.video.gl_get_proc_address(name) as *const _);
		i.video.gl_set_swap_interval(sdl2::video::SwapInterval::VSync);
		unsafe
		{
			// gl::Enable(gl::DEPTH_TEST);
			// gl::DepthFunc(gl::LESS);
			let size = i.window.as_mut().unwrap().size();
			gl::Viewport(0, 0, size.0 as i32, size.1 as i32);
		}

		i.textureCreator = Some(i.tcCanvas.texture_creator());

		Window::loadColors();
	}

	pub fn loadColors()
	{
		let palette = &mut Window::getInstance().palette;

		let f = Assets::readJSON("res/global/colors.json".to_string());
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
				if v.0 == "r" { c.value.r = v.1.as_u8().unwrap(); }
				if v.0 == "g" { c.value.g = v.1.as_u8().unwrap(); }
				if v.0 == "b" { c.value.b = v.1.as_u8().unwrap(); }
				if v.0 == "a" { c.value.a = v.1.as_u8().unwrap(); }
			}
			palette.push(c);
		}
	}

	pub fn update()
	{
		let i = Window::getInstance();
		i.keyEvent = None;
		i.mouseEvent = None;

		i.lastTime = i.currentTime;
		i.currentTime = i.timer.performance_counter() as f32;
		i.deltaTime = (i.currentTime - i.lastTime) / i.timer.performance_frequency() as f32;
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
						pos: Point{ x: x as f32, y: y as f32 }
					});
				},
				sdl2::event::Event::MouseButtonUp { mouse_btn, x, y, .. } =>
				{
					i.mouseEvent = Some(MouseEvent
					{
						btn: mouse_btn,
						clicks: 0,
						pos: Point{ x: x as f32, y: y as f32 }
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
				},
				_ => {}
			}
		}
	}

	pub fn clear()
	{
		let i = Window::getInstance();
		
        unsafe
        {
            let c = Window::toGLcolor(i.clearColor);
            gl::ClearColor(c.0, c.1, c.2, c.3);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
	
	pub fn getTC() -> &'static mut sdl2::render::TextureCreator<sdl2::surface::SurfaceContext<'static>>
	{
		Window::getInstance().textureCreator.as_mut().unwrap()
	}

	pub fn setSize(size: Point)
	{
		Window::getInstance().window.as_mut().unwrap().set_size(
			size.x as u32,
			size.y as u32
		);
	}

	pub fn getSize() -> Point
	{
		let size = Window::getInstance().window.as_mut().unwrap().size();
		Point
		{
			x: size.0 as f32,
			y: size.1 as f32
		}
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
}
