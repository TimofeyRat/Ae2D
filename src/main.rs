#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_must_use, static_mut_refs)]
mod ae3d;

use ae3d::{graphics::Skeleton::Skeleton, Camera::Camera, Window::Window};

fn main()
{
	Window::init();
	Window::setClearColor(sdl2::pixels::Color::RGB(0, 0, 0));

	println!("{}", Window::getGL());
	
	let mut model = Skeleton::new("res/models/triangles.obj".to_string());

	let mut cam = Camera::perspective(ae3d::Camera::CameraMode::FirstPerson, 90.0);

	Window::resetDT();
	while Window::isOpen()
	{
		Window::update();

		Window::clear();
		if Window::getVariable("ingame".to_string()).num == 1.0
		{
			if Window::isKeyPressed(sdl2::keyboard::Scancode::A) { cam.fly(glam::Vec3::NEG_X * 5.0 * Window::getDeltaTime()); }
			if Window::isKeyPressed(sdl2::keyboard::Scancode::D) { cam.fly(glam::Vec3::X * 5.0 * Window::getDeltaTime()); }
			if Window::isKeyPressed(sdl2::keyboard::Scancode::W) { cam.fly(glam::Vec3::Z * 5.0 * Window::getDeltaTime()); }
			if Window::isKeyPressed(sdl2::keyboard::Scancode::S) { cam.fly(glam::Vec3::NEG_Z * 5.0 * Window::getDeltaTime()); }
			if Window::isKeyPressed(sdl2::keyboard::Scancode::E) { cam.fly(glam::Vec3::Y * 5.0 * Window::getDeltaTime()); }
			if Window::isKeyPressed(sdl2::keyboard::Scancode::Q) { cam.fly(glam::Vec3::NEG_Y * 5.0 * Window::getDeltaTime()); }
			Window::setLockCursor(true);
			cam.rotate(Window::getMouseDelta() * 0.25);
			cam.draw(model.getMesh());
		}
		Window::getUI().draw();
		Window::display();
	}
}
