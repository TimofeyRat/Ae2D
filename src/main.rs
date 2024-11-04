#![allow(dead_code, non_snake_case, non_upper_case_globals)]
mod ae2d;

use ae2d::{Window::Window, math::Point::Point, Sprite};

fn main()
{
	Window::create(Point { x: 512.0, y: 288.0 }, "Ae2D".to_string());

	let mut spr = Sprite::Sprite::new();
	spr.loadFromFile("res/tex/menuBG.png".to_string());
	spr.setTextureRect(sdl2::rect::Rect::new(0, 0, 144, 64));


	spr.scaleToSize(Window::getSize());

	while Window::isOpen()
	{
		Window::update();

		Window::clear();
		Window::draw(&mut spr);
		Window::display();
	}
}
