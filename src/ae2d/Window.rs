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
	fn ne(&self, other: &Self) -> bool
	{
		*self as i32 != *other as i32
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

pub struct Window
{
	context: sdl2::Sdl,
	video: Option<sdl2::VideoSubsystem>,
	window: Option<sdl2::video::Window>,
	canvas: Option<sdl2::render::WindowCanvas>,
	events: Option<sdl2::EventPump>,
	running: bool,
	clearColor: sdl2::pixels::Color,
	textureCreator: Option<sdl2::render::TextureCreator<sdl2::video::WindowContext>>,
	deltaTime: f64,
	currentTime: f64,
	lastTime: f64,
	timer: Option<sdl2::TimerSubsystem>,
	keyEvent: Option<KeyEvent>,
	mouseEvent: Option<MouseEvent>,
	ttfContext: sdl2::ttf::Sdl2TtfContext
}

impl Window
{
	pub fn default() -> Window
	{
		Window
		{
			context: sdl2::init().expect("Failed to initialize SDL2"),
			video: None,
			window: None,
			canvas: None,
			events: None,
			running: true,
			clearColor: sdl2::pixels::Color::BLACK,
			textureCreator: None,
			deltaTime: 0.0,
			currentTime: 0.0,
			lastTime: 0.0,
			timer: None,
			keyEvent: None,
			mouseEvent: None,
			ttfContext: sdl2::ttf::init().expect("Failed to initialize TTF")
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

	pub fn create(size: Point, title: String)
	{
		let i = Window::getInstance();
		i.video = Some(i.context.video().unwrap());
		i.window = Some(i.video.as_mut().unwrap()
			.window(title.as_str(), size.x as u32, size.y as u32)
			.position_centered()
			.opengl()
			.build()
			.unwrap());
		i.canvas = Some(i.window.as_mut().unwrap().clone().into_canvas().accelerated().build().unwrap());
		i.events = Some(i.context.event_pump().unwrap());
		i.textureCreator = Some(i.canvas.as_mut().unwrap().texture_creator());
		i.timer = Some(i.context.timer().unwrap());
		i.lastTime = i.timer.as_mut().unwrap().performance_counter() as f64;
		i.currentTime = i.lastTime;
	}

	pub fn init()
	{
		let f = Assets::readJSON("res/global/config.json".to_string());
		if f.is_none() { return }

		let mut title = String::from("");
		let mut size = Point::zero();
		let mut style = String::from("");
		let mut pos = Point::num(-127.0);
		let mut ogl = false;

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
							if dim.0 == "w" { size.x = dim.1.as_f64().unwrap(); }
							if dim.0 == "h" { size.y = dim.1.as_f64().unwrap(); }
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
							if dim.0 == "x" { pos.x = dim.1.as_f64().unwrap(); }
							if dim.0 == "y" { pos.y = dim.1.as_f64().unwrap(); }
						}
					}
					if attr.0 == "OpenGL" { ogl = attr.1.as_bool().unwrap(); }
				}
			}
			if section.0 == "custom" {}
		}

		let i = Window::getInstance();
		i.video = Some(i.context.video().unwrap());
		let mut builder = i.video.as_mut().unwrap()
			.window(title.as_str(), size.x as u32, size.y as u32);

		if pos != Point::num(-127.0) { builder.position(pos.x as i32, pos.y as i32); }
		else { builder.position_centered(); }
		if ogl { builder.opengl(); }
		if style.as_str() == "resizable" { builder.resizable(); }
		if style.as_str() == "borderless" { builder.borderless(); }
		if style.as_str() == "fullscreen" { builder.fullscreen_desktop(); }

		i.window = Some(builder.build().unwrap());
		i.canvas = Some(i.window.as_mut().unwrap().clone().into_canvas().accelerated().build().unwrap());
		i.events = Some(i.context.event_pump().unwrap());
		i.textureCreator = Some(i.canvas.as_mut().unwrap().texture_creator());
		i.timer = Some(i.context.timer().unwrap());
		i.lastTime = i.timer.as_mut().unwrap().performance_counter() as f64;
		i.currentTime = i.lastTime + 1.0;
	}

	pub fn update()
	{
		let i = Window::getInstance();
		i.keyEvent = None;
		i.mouseEvent = None;

		i.lastTime = i.currentTime;
		i.currentTime = i.timer.as_mut().unwrap().performance_counter() as f64;
		i.deltaTime = (i.currentTime - i.lastTime) / i.timer.as_mut().unwrap().performance_frequency() as f64;
		for event in i.events.as_mut().unwrap().poll_iter()
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
						pos: Point{ x: x as f64, y: y as f64 }
					});
				},
				sdl2::event::Event::MouseButtonUp { mouse_btn, x, y, .. } =>
				{
					i.mouseEvent = Some(MouseEvent
					{
						btn: mouse_btn,
						clicks: 0,
						pos: Point{ x: x as f64, y: y as f64 }
					});
				}
				_ => {}
			}
		}
	}

	pub fn clear()
	{
		let i = Window::getInstance();
		i.canvas.as_mut().unwrap().set_draw_color(i.clearColor);
		i.canvas.as_mut().unwrap().clear();
	}
	
	pub fn getTC() -> &'static mut sdl2::render::TextureCreator<sdl2::video::WindowContext>
	{
		Window::getInstance().textureCreator.as_mut().unwrap()
	}

	pub fn draw(spr: &mut super::Sprite::Sprite)
	{
		spr.draw(Window::getInstance().canvas.as_mut().unwrap());
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
			x: size.0 as f64,
			y: size.1 as f64
		}
	}

	pub fn isKeyPressed(key: sdl2::keyboard::Scancode) -> bool
	{
		Window::getInstance().events.as_mut().unwrap().keyboard_state().is_scancode_pressed(key)
	}

	pub fn isMousePressed(btn: sdl2::mouse::MouseButton) -> bool
	{
		Window::getInstance().events.as_mut().unwrap().mouse_state().is_mouse_button_pressed(btn)
	}

	pub fn getTTF() -> &'static mut sdl2::ttf::Sdl2TtfContext { &mut Window::getInstance().ttfContext }
	pub fn getKeyEvent() -> Option<KeyEvent> { Window::getInstance().keyEvent }
	pub fn getMouseEvent() -> Option<MouseEvent> { Window::getInstance().mouseEvent }
	pub fn setClearColor(clr: sdl2::pixels::Color) { Window::getInstance().clearColor = clr; }
	pub fn display() { Window::getInstance().canvas.as_mut().unwrap().present(); }
	pub fn isOpen() -> bool { Window::getInstance().running }
	pub fn close() { Window::getInstance().running = false; }
	pub fn getDeltaTime() -> f64 { Window::getInstance().deltaTime }
}