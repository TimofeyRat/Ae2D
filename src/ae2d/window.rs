use glfw::Context;
use crate::ae2d::math;

use crate::ae2d::render;

struct CharEvent
{
	input: char,
	mods: Option<glfw::Modifiers>
}

struct KeyEvent
{
	key: glfw::Key,
	scancode: glfw::Scancode,
	action: glfw::Action,
	mods: glfw::Modifiers
}

struct MouseButtonEvent
{
	btn: glfw::MouseButton,
	action: glfw::Action,
	mods: glfw::Modifiers
}

struct MouseMoveEvent
{
	inWindow: bool,
	pos: math::Point,
	scroll: math::Point
}

pub struct Window
{
	//State
	context: glfw::Glfw,
	window: Option<glfw::PWindow>,
	events: Option<glfw::GlfwReceiver<(f64, glfw::WindowEvent)>>,
	focus: bool,
	iconified: bool,
	maximized: bool,
	clear_color: render::Color,
	//Events
	last_mm: Option<MouseMoveEvent>,
	last_input: Option<CharEvent>,
	last_key: Option<KeyEvent>,
	last_mb: Option<MouseButtonEvent>,
	resized: bool,
	moved: bool,
	dragAndDrop: Option<Vec<std::path::PathBuf>>
}

impl Window
{
	pub fn default() -> Window
	{
		Window
		{
			context: glfw::init(glfw::fail_on_errors).unwrap(),
			window: None,
			events: None,
			last_input: None,
			last_mm: None,
			focus: true,
			iconified: false,
			last_key: None,
			maximized: false,
			last_mb: None,
			resized: false,
			moved: false,
			dragAndDrop: None,
			clear_color: render::Color::Black
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

	fn init(size: math::Point, title: &str, fullscreen: bool)
	{
		let i = Window::getInstance();
		let win = i.context.with_primary_monitor(|c, m| {
			let mode: glfw::WindowMode;
			let w: u32;
			let h: u32;
			if fullscreen
			{
				mode = glfw::WindowMode::FullScreen(m.as_ref().unwrap());
				w = m.as_ref().unwrap().get_video_mode().unwrap().width;
				h = m.as_ref().unwrap().get_video_mode().unwrap().height;
			}
			else { mode = glfw::WindowMode::Windowed; w = size.x as u32; h = size.y as u32; }
			c.create_window(w, h, title, mode)
		}).unwrap();
		i.window = Some(win.0);
		i.events = Some(win.1);
		i.window.as_mut().unwrap().make_current();
		i.window.as_mut().unwrap().swap_buffers();
		i.window.as_mut().unwrap().set_all_polling(true);
		gl::load_with(|s| i.window.as_mut().unwrap().get_proc_address(s));
	}

	pub fn create(size: math::Point, title: &str)
	{
		Window::init(size, title, false);
		println!("Initialized window of size {}x{} called \"{}\"", size.x, size.y, title);
	}

	pub fn createFullscreen(title: &str)
	{
		Window::init(math::Point::zero(), title, true);
		println!("Intiailized fullscreen window called \"{title}\"");
	}

	pub fn update()
	{
		let i = Window::getInstance();
		i.last_input = None;
		i.last_key = None;
		i.last_mm = None;
		i.last_mb = None;
		i.resized = false;
		i.moved = false;
		i.dragAndDrop = None;
		i.context.poll_events();
		for (_, event) in glfw::flush_messages(i.events.as_ref().unwrap())
		{
			match event
			{
				glfw::WindowEvent::Close => { Window::close(); }
				glfw::WindowEvent::Char(c) => { i.last_input = Some(CharEvent { input: c, mods: None }); }
				glfw::WindowEvent::CharModifiers(c, m) => { i.last_input = Some(CharEvent { input: c, mods: Some(m) }); }
				glfw::WindowEvent::ContentScale(x, y) => { println!("Content scaled by {x}x{y}"); }
				glfw::WindowEvent::CursorEnter(enter) =>
					{ i.last_mm = Some(MouseMoveEvent { inWindow: enter, pos: math::Point::zero(), scroll: math::Point::zero() }); }
				glfw::WindowEvent::CursorPos(x, y) =>
					{ i.last_mm = Some(MouseMoveEvent { inWindow: true, pos: math::Point {x, y}, scroll: math::Point::zero() }); }
				glfw::WindowEvent::FileDrop(file) => { i.dragAndDrop = Some(file); }
				glfw::WindowEvent::Focus(focus) => { i.focus = focus; }
				glfw::WindowEvent::FramebufferSize(_, _) | glfw::WindowEvent::Size(_, _) => { i.resized = true; }
				glfw::WindowEvent::Iconify(iconified) => { i.iconified = iconified; }
				glfw::WindowEvent::Key(key, scancode, action, mods) =>
					{ i.last_key = Some(KeyEvent { key, scancode, action, mods }); }
				glfw::WindowEvent::Maximize(maximized) => { i.maximized = maximized; }
				glfw::WindowEvent::MouseButton(btn, action, mods) =>
					{ i.last_mb = Some(MouseButtonEvent { btn, action, mods });}
				glfw::WindowEvent::Refresh => {}
				glfw::WindowEvent::Scroll(x, y) =>
					{ i.last_mm = Some(MouseMoveEvent { inWindow: true, pos: math::Point::zero(), scroll: math::Point { x, y } }); }
				glfw::WindowEvent::Pos(_, _) => { i.moved = true; }
			}
		}
	}

	pub fn isOpen() -> bool { !Window::getInstance().window.as_mut().unwrap().should_close() }
	pub fn isActive() -> bool { Window::getInstance().window.as_mut().unwrap().is_focused() }
	pub fn clear()
	{
		let i = Window::getInstance();
		i.window.as_mut().unwrap().make_current();
		unsafe
		{
			let clr = i.clear_color.toGL();
			gl::ClearColor(clr.0, clr.1, clr.2, clr.3);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}
	}
	pub fn display() { Window::getInstance().window.as_mut().unwrap().swap_buffers(); }
	pub fn close() { Window::getInstance().window.as_mut().unwrap().set_should_close(true); }
	pub fn setClearColor(clr: render::Color) { Window::getInstance().clear_color = clr; }
}