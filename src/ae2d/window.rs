use super::math;

pub struct Window
{
	context: sdl2::Sdl,
	video: Option<sdl2::VideoSubsystem>,
	window: Option<sdl2::video::Window>,
	canvas: Option<sdl2::render::WindowCanvas>,
	events: Option<sdl2::EventPump>,
	running: bool,
	clearColor: sdl2::pixels::Color
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
			clearColor: sdl2::pixels::Color::BLACK
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

	pub fn create(size: math::Point, title: String)
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
	}

	pub fn update()
	{
		let i = Window::getInstance();
		for event in i.events.as_mut().unwrap().poll_iter()
		{
			match event
			{
				sdl2::event::Event::Quit {..} => { i.running = false; }
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

	pub fn display() { Window::getInstance().canvas.as_mut().unwrap().present(); }
	pub fn isOpen() -> bool { Window::getInstance().running }
	pub fn close() { Window::getInstance().running = false; }

	pub fn getCanvas() -> &'static mut sdl2::render::WindowCanvas { Window::getInstance().canvas.as_mut().unwrap() }
}