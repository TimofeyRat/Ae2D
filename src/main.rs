#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_must_use, static_mut_refs)]
mod ae3d;

use ae3d::Window::Window;

fn main()
{
	Window::init();
	Window::setClearColor(sdl2::pixels::Color::RGB(0, 0, 0));

	let mut cam = ae3d::Camera::Camera::perspective(ae3d::Camera::CameraMode::FirstPerson, 45.0);
	cam.translate(glam::vec3(-25.0, 40.0, 35.0));
	cam.rotate(glam::vec2(-55.0, 35.0));

	let moveSpeed = 15.0;

	let mut ui = ae3d::graphics::Shader::Shader::new();
	ui.load("res/shaders/ui.vert".to_string(), "res/shaders/ui.frag".to_string());
	let proj = glam::Mat4::orthographic_rh_gl(0.0, Window::getSize().x, Window::getSize().y, 0.0, 0.1, 100.0);

	let mut dbg = ae3d::graphics::Text::Text::new();
	dbg.loadFont("res/fonts/b52.fnt".to_string());
	dbg.setSize(24);

	let mut newMesh = ae3d::graphics::Mesh::Mesh::new("res/models/triangles.obj".to_string()).expect("Failed to load mesh");
	let mut newSkybox = ae3d::graphics::Mesh::Mesh::new("res/models/skybox.obj".to_string()).expect("Failed to load skybox");
	newSkybox.setScale(glam::Vec3::splat(100.0));
	newSkybox.setLighting(false);
	
	println!("{}", Window::getGL());

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

		cam.rotate(Window::getMouseDelta() / 5.0);

		newSkybox.setPosition(cam.getPosition());

		Window::clear();
		cam.draw(&mut newSkybox);
		cam.draw(&mut newMesh);

		ui.activate();
		ui.setMat4("projection".to_string(), &proj.to_cols_array());
		
		dbg.setString(format!("{:?}", Window::getMouseDelta()));
		dbg.draw(&mut ui);
		
		Window::display();
	}
}
