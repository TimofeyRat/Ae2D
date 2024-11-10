#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_must_use)]
mod ae2d;

use ae2d::{math::Point::Point, Window::{KeyAction, Window}};

fn main()
{
	Window::create(Point { x: 512.0, y: 288.0 }, "Ae2D".to_string());

	let mut spr = ae2d::Sprite::Sprite::new();

	spr.loadAnimator("res/anims/test.json".to_string());
	spr.scaleToSize(Window::getSize());
	// spr.setRelativeOrigin(Point::num(0.5));
	// spr.setPosition(Window::getSize() * Point::num(0.5));

	while Window::isOpen()
	{
		Window::update();

		let escape = Window::getKeyEvent();
		if escape.is_some()
		{
			if escape.unwrap().key == sdl2::keyboard::Scancode::Escape &&
			   escape.unwrap().action == KeyAction::PressedRepeat
			{
				Window::close();
			}
		}

		// if Window::isKeyPressed(sdl2::keyboard::Scancode::A) { spr.moveBy(Point { x: -100.0 * Window::getDeltaTime(), y: 0.0 }); }
		// if Window::isKeyPressed(sdl2::keyboard::Scancode::D) { spr.moveBy(Point { x: 100.0 * Window::getDeltaTime(), y: 0.0 }); }
		// if Window::isKeyPressed(sdl2::keyboard::Scancode::W) { spr.moveBy(Point { x: 0.0, y: -100.0 * Window::getDeltaTime() }); }
		// if Window::isKeyPressed(sdl2::keyboard::Scancode::S) { spr.moveBy(Point { x: 0.0, y: 100.0 * Window::getDeltaTime() }); }

		// if Window::isKeyPressed(sdl2::keyboard::Scancode::Q) { spr.rotate(-90.0 * Window::getDeltaTime()); }
		// if Window::isKeyPressed(sdl2::keyboard::Scancode::E) { spr.rotate(90.0 * Window::getDeltaTime()); }
		// if Window::isKeyPressed(sdl2::keyboard::Scancode::R) { spr.setRotation(0.0); }

		// if Window::isKeyPressed(sdl2::keyboard::Scancode::Z) { spr.scale(Point::num(2.0 * Window::getDeltaTime())); }
		// if Window::isKeyPressed(sdl2::keyboard::Scancode::X) { spr.scale(Point::num(-2.0 * Window::getDeltaTime())); }
		// if Window::isKeyPressed(sdl2::keyboard::Scancode::C) { spr.setScale(Point::num(1.0)); }

		Window::clear();
		Window::draw(&mut spr);
		Window::display();
	}
}
