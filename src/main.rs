#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_must_use)]
mod ae2d;

use ae2d::{Window::Window, math::Point::Point};

fn main()
{
	Window::create(Point { x: 512.0, y: 288.0 }, "Ae2D".to_string());

	let mut x = ae2d::Animation::Animation::new();
	x.loadFromFile(String::from("res/anims/test.json"));

	while Window::isOpen()
	{
		Window::update();

		Window::clear();
		Window::display();
	}
}
