#![allow(dead_code, non_snake_case, non_upper_case_globals)]
mod ae2d;

use ae2d::{window::Window, math};
use sdl2::image::LoadTexture;


fn main()
{
	Window::create(math::Point { x: 512.0, y: 288.0 }, "Ae2D".to_string());

	let c = Window::getCanvas();
	let tc = c.texture_creator();

	let s = tc.load_texture(sdl2::filesystem::base_path().unwrap() + "res/tex/menuBG.png").unwrap();

	while Window::isOpen()
	{
		Window::update();

		Window::clear();

		let _ = c.copy(&s, None, sdl2::rect::Rect::new(0, 0, 100, 100));

		Window::display();
	}
}
