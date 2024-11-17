#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_must_use)]
mod ae2d;

use std::io::Write;

use ae2d::{math::Point::Point, Assets, Window::Window};

fn main()
{
	Window::init();

	let mut txt = ae2d::Text::Text::new();
	txt.loadFont(Assets::getCurrentDir() + "res/fonts/main.ttf", 24);
	txt.setString(String::from("^(* clr=red)bold text ^(/ clr=green)italic text ^(clr=blue)newline\n^(-)strikethrough text^() ^(_)underlined text"));
	txt.setAnchor(ae2d::Text::Anchor::Center, ae2d::Text::Anchor::Center);
	txt.transform.setPosition(Window::getSize() / Point::num(2.0));

	let mut fpsTimer: f64 = 0.0;
	let mut fpsCounter: f64 = 0.0;
	let mut fpsCount = 0;

	while Window::isOpen()
	{
		Window::update();

		fpsTimer += Window::getDeltaTime();
		fpsCounter += 1.0 / Window::getDeltaTime();
		fpsCount += 1;

		if fpsTimer >= 1.0
		{
			fpsTimer = 0.0;
			print!("{}\r", fpsCounter / fpsCount as f64);
			fpsCounter = 0.0;
			fpsCount = 0;
			std::io::stdout().flush();
		}

		Window::clear();
		txt.draw(&mut Window::getCanvas());
		Window::display();
	}
}