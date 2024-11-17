#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_must_use)]
mod ae2d;

use ae2d::{Assets, Window::Window};

fn main()
{
	Window::init();

	let mut txt = ae2d::Text::Text::new();
	txt.loadFont(Assets::getCurrentDir() + "res/fonts/main.ttf", 24);
	txt.setString(String::from("^(*)bold text ^(/)italic text ^()newline\n"));

	while Window::isOpen()
	{
		Window::update();

		Window::clear();
		Window::display();
	}
}