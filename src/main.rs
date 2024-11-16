#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_must_use)]
mod ae2d;

use ae2d::Window::Window;

fn main()
{
	Window::init();
	
	let mut spr = ae2d::Sprite::Sprite::new();

	spr.loadAnimator("res/anims/test.json".to_string());
	spr.scaleToSize(Window::getSize());

	while Window::isOpen()
	{
		Window::update();

		Window::clear();
		Window::draw(&mut spr);
		Window::display();
	}
}
