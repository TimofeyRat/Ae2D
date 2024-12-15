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
	light.setPosition(glam::vec3(-25.0, 35.0, 25.0));
	light.rotateY(45.0 - 180.0);
	light.rotateZ(45.0);

	let mut cam = ae3d::Camera::Camera::perspective(ae3d::Camera::CameraMode::FirstPerson, 45.0);
	cam.translate(glam::vec3(-25.0, 40.0, 35.0));
	cam.rotate(glam::vec2(-55.0, 35.0));

	// let moveSpeed = 3.0;
	let moveSpeed = 10.0;
	let rotateSpeed = 75.0;

	let mut f = ae3d::graphics::Text::Font::load("res/fonts/b52.fnt".to_string());

	println!("Character {}: {:?}", 'A' as u16, f.getGlyph('A'));
	
	/*
		TODO:
		Write a function to get the glyphs out of the font; (SUCCESS)
		Try to bind the texture of font and
		build a mesh to draw chars
	*/

	let glyph = f.getGlyph('A').rect;

	// let vertices: [f32; 16] = [
	// 	0.0, 0.0,		glyph.x, glyph.y,
	// 	100.0, 0.0,		glyph.x + glyph.w, glyph.y,
	// 	100.0, 100.0,	glyph.x + glyph.w, glyph.y + glyph.h,
	// 	0.0, 100.0,		glyph.x, glyph.y + glyph.h
	// ];

	let vertices: [f32; 16] = [
		0.0, 0.0,		0.0, 0.0,
		1024.0, 0.0,	1.0, 0.0,
		1024.0, 512.0,	1.0, 1.0,
		0.0, 512.0,		0.0, 1.0
	];

	let mut ui = ae3d::graphics::Shader::Shader::new();
	ui.load("res/shaders/ui.vert".to_string(), "res/shaders/ui.frag".to_string());

	let mut vbo = 0;
	let mut vao = 0;

	unsafe
	{
		gl::GenBuffers(1, &mut vbo);
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(
			gl::ARRAY_BUFFER,
			(vertices.len() * size_of::<f32>()) as isize,
			vertices.as_ptr() as *const _,
			gl::STATIC_DRAW
		);

		gl::GenVertexArrays(1, &mut vao);
		gl::BindVertexArray(vao);
		gl::EnableVertexAttribArray(0);
		gl::EnableVertexAttribArray(1);

		gl::VertexAttribPointer(
			0,
			2,
			gl::FLOAT,
			gl::FALSE,
			(4 * size_of::<f32>()) as i32,
			std::ptr::null()
		);
		gl::VertexAttribPointer(
			1,
			2,
			gl::FLOAT,
			gl::FALSE,
			(4 * size_of::<f32>()) as i32,
			(2 * size_of::<f32>()) as *const _
		);
	}

	let proj = glam::Mat4::orthographic_rh_gl(0.0, Window::getSize().x, Window::getSize().y, 0.0, 0.1, 100.0);

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

		Window::clear();
		cam.draw(&mut m);
		cam.draw(&mut light);

		unsafe
		{
			ui.activate();
			gl::ActiveTexture(gl::TEXTURE0);
			f.bindTexture();
			ui.setInt("tex".to_string(), 0);
			ui.setMat4("projection".to_string(), &proj.to_cols_array());

			gl::BindVertexArray(vao);

			gl::DrawArrays(gl::QUADS, 0, 4);
		}
		Window::display();
	}
}
