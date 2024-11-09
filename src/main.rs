#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_must_use)]
mod ae2d;

use ae2d::{Window::Window, math::Point::Point};

fn main()
{
	Window::create(Point { x: 512.0, y: 288.0 }, "Ae2D".to_string());

	let mut spr = ae2d::Sprite::Sprite::new();
	// spr.loadTexture("res/tex/menuBG.png".to_string());
	// spr.setTextureRect(sdl2::rect::Rect::new(0, 0, 144, 64));
	spr.loadAnimator("res/anims/test.json".to_string());
	spr.scaleToSize(Window::getSize());
	spr.setPosition(Window::getSize() / Point::num(2.0));
	spr.setRelativeOrigin(Point::num(0.5));

	while Window::isOpen()
	{
		Window::update();

		// let angle = spr.getRotation();
		// spr.setRotation(angle + Window::getDeltaTime() * 180.0);

		Window::clear();
		Window::draw(&mut spr);
		Window::display();
	}
}
