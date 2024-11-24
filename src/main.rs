#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_must_use)]
mod ae3d;

use ae3d::{graphics::Mesh::*, Window::Window};

fn main()
{
	Window::init();
	Window::setClearColor(sdl2::pixels::Color::RGB(127, 127,127));

	let vertices: Vec<f32>= vec![
		-10.0, 0.0, -10.0,	0.0, 0.0,
		10.0, 0.0, -10.0,	5.0, 0.0,
		10.0, 0.0, 10.0,	5.0, 5.0,
		-10.0, 0.0, 10.0,	0.0, 5.0
	];
		
	let indices: Vec<u32> = vec![
		0, 1, 2,
		2, 3, 0
	];

	let mut m = Mesh::new();
	m.loadTexture("res/tex/floor.png".to_string(), gl::REPEAT);
	m.gen(&vertices, &indices);
	// m.translate(glm::vec3(0.0,0.0, -3.0));

	let mut cam = ae3d::Camera::Camera::perspective(ae3d::Camera::CameraMode::FirstPerson, 45.0);
	cam.translate(glm::vec3(0.0, 1.0, -3.0));
	cam.rotate(glm::vec2(90.0, 0.0));

	let moveSpeed = 3.0;
	let rotateSpeed = 75.0;

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
		Window::display();
	}
}
