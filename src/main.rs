#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_must_use)]
mod ae2d;

use ae2d::{Window::Window, math::Point::Point};

fn main()
{
	Window::create(Point { x: 512.0, y: 288.0 }, "Ae2D".to_string());

	// let mut spr = ae2d::Sprite::Sprite::new();
	// spr.loadTexture("res/tex/menuBG.png".to_string());
	// spr.setTextureRect(sdl2::rect::Rect::new(0, 0, 144, 64));
	// spr.setScale(Point { x: 1.0, y: 1.0});
	// spr.setPosition(Point { x: 100.0, y: 100.0 });
	// spr.setRelativeOrigin(Point { x: 0.5, y: 0.5 });
	// spr.setRotation(45.0);

	let mut x = ae2d::Animation::Animation::new();
	x.loadFromFile(String::from("res/anims/test.json"));

	while Window::isOpen()
	{
		Window::update();

		Window::clear();
		// Window::draw(&mut spr);
		Window::display();
	}
}
