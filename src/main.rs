#![allow(dead_code, non_snake_case, non_upper_case_globals)]
mod ae2d;

use ae2d::{window::Window, math, render};

fn main()
{
	Window::create(math::Point { x: 512.0, y: 288.0 }, "Ae2D".to_string());

	let mut spr = render::Sprite::new();
	spr.loadFromFile("res/tex/menuBG.png".to_string());
	spr.setTextureRect(sdl2::rect::Rect::new(0, 0, 144, 64));

	let size = spr.getTextureRect().size();

	spr.setScale(math::Point
	{
		x: Window::getSize().x / size.0 as f64,
		y: Window::getSize().y / size.1 as f64
	});

	while Window::isOpen()
	{
		Window::update();

		Window::clear();
		Window::draw(&mut spr);
		Window::display();
	}
}
