#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_must_use)]
mod ae3d;

use ae3d::{graphics::Mesh::*, Window::Window};

fn main()
{
	Window::init();
	Window::setClearColor(sdl2::pixels::Color::RGB(0, 0, 0));

	// let mut skybox = Mesh::new();
	// skybox.loadFromFile("res/models/skybox.obj".to_string());
	// skybox.setScale(glam::vec3(100.0, 100.0, 100.0));
	// skybox.setApplyLighting(false);

	// let mut m = Mesh::new();
	// m.loadFromFile("res/models/quads.obj".to_string());

	let mut utah = NewMesh::new("res/models/utah.obj".to_string()).expect("Failed to load model");

	let mut cam = ae3d::Camera::Camera::perspective(ae3d::Camera::CameraMode::FirstPerson, 45.0);
	cam.translate(glam::vec3(-25.0, 40.0, 35.0));
	cam.rotate(glam::vec2(-55.0, 35.0));

	// let moveSpeed = 3.0;
	let moveSpeed = 10.0;
	let rotateSpeed = 75.0;

	// let mut ui = ae3d::graphics::Shader::Shader::new();
	// ui.load("res/shaders/ui.vert".to_string(), "res/shaders/ui.frag".to_string());
	// let proj = glam::Mat4::orthographic_rh_gl(0.0, Window::getSize().x, Window::getSize().y, 0.0, 0.1, 100.0);

	// let mut txt = ae3d::graphics::Text::Text::new();
	// txt.loadFont("res/fonts/b52.fnt".to_string());
	// txt.setString("^(*)Hello ^(/)world".to_string());

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
			cam.fly(glam::vec3(0.0, 0.0, moveSpeed * Window::getDeltaTime()));
		}
		if Window::isKeyPressed(sdl2::keyboard::Scancode::S)
		{
			cam.fly(glam::vec3(0.0, 0.0, -moveSpeed * Window::getDeltaTime()));
		}
		if Window::isKeyPressed(sdl2::keyboard::Scancode::A)
		{
			cam.fly(glam::vec3(-moveSpeed * Window::getDeltaTime(), 0.0, 0.0));
		}
		if Window::isKeyPressed(sdl2::keyboard::Scancode::D)
		{
			cam.fly(glam::vec3(moveSpeed * Window::getDeltaTime(), 0.0, 0.0));
		}
		if Window::isKeyPressed(sdl2::keyboard::Scancode::Space)
		{
			cam.fly(glam::vec3(0.0, moveSpeed * Window::getDeltaTime(), 0.0));
		}
		if Window::isKeyPressed(sdl2::keyboard::Scancode::LShift)
		{
			cam.fly(glam::vec3(0.0, -moveSpeed * Window::getDeltaTime(), 0.0));
		}

		if Window::isKeyPressed(sdl2::keyboard::Scancode::Up)
		{
			cam.rotate(glam::vec2(0.0, -rotateSpeed * Window::getDeltaTime()));
		}
		if Window::isKeyPressed(sdl2::keyboard::Scancode::Down)
		{
			cam.rotate(glam::vec2(0.0, rotateSpeed * Window::getDeltaTime()));
		}
		if Window::isKeyPressed(sdl2::keyboard::Scancode::Left)
		{
			cam.rotate(glam::vec2(-rotateSpeed * Window::getDeltaTime(), 0.0));
		}
		if Window::isKeyPressed(sdl2::keyboard::Scancode::Right)
		{
			cam.rotate(glam::vec2(rotateSpeed * Window::getDeltaTime(), 0.0));
		}

		// skybox.setPosition(cam.getPosition());

		Window::clear();
		// cam.draw(&mut skybox);
		// cam.draw(&mut m);
		// cam.draw(&mut utah);

		// ui.activate();
		// ui.setMat4("projection".to_string(), &proj.to_cols_array());
		// txt.draw(&mut ui);
		
		Window::display();
	}
}
