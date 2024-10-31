use crate::math;

pub struct Window
{
	context: sdl2::Sdl,
	running: bool,
	video: Option<sdl2::VideoSubsystem>,
	window: Option<sdl2::video::Window>,
	canvas: Option<sdl2::render::WindowCanvas>,
	clear_color: sdl2::pixels::Color,
	mouse: Option<sdl2::mouse::MouseState>,
	event_pump: Option<sdl2::EventPump>,
	keyboard: Option<sdl2::keyboard::KeyboardState<'static>>,
}

impl Window
{
	fn get_instance() -> &'static mut Window
	{
		static mut INSTANCE: Option<Window> = None;
		
		unsafe
		{
			if INSTANCE.is_none() { INSTANCE = Some(Window::default()); }
			INSTANCE.as_mut().expect("Window singleton is not initialized")
		}
	}

	pub fn init(size: math::Point, title: &str)
	{
		let i = Window::get_instance();
		i.video = Some(i.context.video().unwrap());
		i.window = Some(i.video.as_mut().unwrap()
			.window(title, size.x as u32, size.y as u32)
			.position_centered()
			.opengl()
			.resizable()
			.build()
			.unwrap());
		i.canvas = Some(i.window.clone().unwrap().into_canvas().build().unwrap());
		i.event_pump = Some(i.context.event_pump().unwrap());
		i.keyboard = Some(i.event_pump.as_ref().unwrap().keyboard_state());
		println!("Initialized window of size {}x{} called \"{}\"", size.x, size.y, title);
	}

	pub fn update()
	{
		let i = Window::get_instance();
		for event in i.event_pump.as_mut().unwrap().poll_iter()
		{
			match event
			{
				sdl2::event::Event::Quit { .. } => { i.running = false; }
				sdl2::event::Event::MouseMotion { mousestate, ..} => { i.mouse = Some(mousestate); }
				// sdl2::event::Event::KeyDown { .. } => { i.keyboard = Some(sdl2::keyboard::KeyboardState::new(e)) }
				// _ => { println!("Catched event #{event:?}"); }
				_ => {}
			}
		}
	}

	pub fn clear()
	{
		let i = Window::get_instance();
		let canvas = i.canvas.as_mut().unwrap();
		canvas.set_draw_color(i.clear_color);
		canvas.clear();
		canvas.present();
	}

	fn default() -> Window
	{
		Window
		{
			context: sdl2::init().unwrap(),
			running: true,
			video: None,
			window: None,
			canvas: None,
			clear_color: sdl2::pixels::Color::RGB(0, 0, 0),
			mouse: None,
			keyboard: None,
			event_pump: None
		}
	}

	pub fn set_clear_color(clr: sdl2::pixels::Color) { Window::get_instance().clear_color = clr; }
	pub fn is_open() -> bool { Window::get_instance().running }
	pub fn display() { Window::get_instance().canvas.as_mut().unwrap().present(); }
	pub fn close() { Window::get_instance().running = false; }
	pub fn is_key_pressed(key: sdl2::keyboard::Scancode) -> bool
	{
		Window::get_instance().keyboard.as_mut().unwrap().is_scancode_pressed(key)
	}
}