#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_must_use, static_mut_refs)]
mod ae3d;

use ae3d::Window::Window;

fn main()
{
	Window::init();
	Window::setClearColor(sdl2::pixels::Color::RGB(0, 0, 0));

	println!("{}", Window::getGL());

	while Window::isOpen()
	{
		Window::update();

		Window::clear();
		Window::getUI().draw();
		Window::display();
	}
}
