#![allow(dead_code)]
mod ae2d;

use ae2d::{window::Window, math};
use sdl2::keyboard::Scancode;

fn main()
{
	Window::init(math::Point { x: 1024.0, y: 576.0 }, "Ae2D");

	while Window::is_open()
	{
		Window::update();
		if Window::is_key_pressed(Scancode::Escape) { Window::close(); }
		Window::clear();
		Window::display();
	}
}
