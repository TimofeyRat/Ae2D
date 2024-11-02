#![allow(dead_code, non_snake_case, non_upper_case_globals)]
mod ae2d;

use ae2d::{window::Window, math, render};

fn main()
{
	Window::create(math::Point { x: 512.0, y: 288.0 }, "Ae2D");

	Window::setClearColor(render::Color::White);

	while Window::isOpen()
	{
		Window::update();

		Window::clear();
		Window::display();
	}
}
