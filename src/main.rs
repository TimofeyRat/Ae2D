#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_must_use)]
mod ae3d;

use ae3d::{graphics::Mesh::*, Window::Window};

fn main()
{
	Window::init();
	Window::setClearColor(sdl2::pixels::Color::RGB(0, 0, 0));

	let mut m = Mesh::new();
	m.loadFromFile("res/models/quads.obj".to_string());

	let mut light = Mesh::new();
	light.loadFromFile("res/models/cube.obj".to_string());
	light.setPosition(glm::vec3(-25.0, 25.0, 25.0));

	let mut cam = ae3d::Camera::Camera::perspective(ae3d::Camera::CameraMode::FirstPerson, 45.0);
	cam.translate(glm::vec3(0.0, 1.0, -3.0));
	cam.rotate(glm::vec2(90.0, 0.0));

	// let moveSpeed = 3.0;
	let moveSpeed = 10.0;
	let rotateSpeed = 75.0;

	unsafe
	{
		let v = gl::GetString(gl::VERSION);
		let mut size: isize = 0;
		let mut vector: Vec<u8> = vec![];
		while v.offset(size).read() != 0
		{
			vector.push(v.offset(size).read());
			size += 1;
		}
		let str = String::from_utf8(vector).unwrap();
		println!("{str}");
	}

	while Window::isOpen()
	{
		Window::update();

		if Window::isKeyPressed(sdl2::keyboard::Scancode::W)
		{
			cam.fly(glm::vec3(0.0, 0.0, moveSpeed * Window::getDeltaTime()));
		}
		if Window::isKeyPressed(sdl2::keyboard::Scancode::S)
		{
			cam.fly(glm::vec3(0.0, 0.0, -moveSpeed * Window::getDeltaTime()));
		}
		if Window::isKeyPressed(sdl2::keyboard::Scancode::A)
		{
			cam.fly(glm::vec3(-moveSpeed * Window::getDeltaTime(), 0.0, 0.0));
		}
		if Window::isKeyPressed(sdl2::keyboard::Scancode::D)
		{
			cam.fly(glm::vec3(moveSpeed * Window::getDeltaTime(), 0.0, 0.0));
		}
		if Window::isKeyPressed(sdl2::keyboard::Scancode::Space)
		{
			cam.fly(glm::vec3(0.0, moveSpeed * Window::getDeltaTime(), 0.0));
		}
		if Window::isKeyPressed(sdl2::keyboard::Scancode::LShift)
		{
			cam.fly(glm::vec3(0.0, -moveSpeed * Window::getDeltaTime(), 0.0));
		}

		if Window::isKeyPressed(sdl2::keyboard::Scancode::Up)
		{
			cam.rotate(glm::vec2(0.0, -rotateSpeed * Window::getDeltaTime()));
		}
		if Window::isKeyPressed(sdl2::keyboard::Scancode::Down)
		{
			cam.rotate(glm::vec2(0.0, rotateSpeed * Window::getDeltaTime()));
		}
		if Window::isKeyPressed(sdl2::keyboard::Scancode::Left)
		{
			cam.rotate(glm::vec2(-rotateSpeed * Window::getDeltaTime(), 0.0));
		}
		if Window::isKeyPressed(sdl2::keyboard::Scancode::Right)
		{
			cam.rotate(glm::vec2(rotateSpeed * Window::getDeltaTime(), 0.0));
		}

		Window::clear();
		cam.draw(&mut m);
		cam.draw(&mut light);
		Window::display();
	}
}
