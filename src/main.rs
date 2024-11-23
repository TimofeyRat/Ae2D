#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_must_use)]
mod ae3d;

use ae3d::{graphics::Mesh::*, Window::Window};

fn main()
{
	Window::init();
	Window::setClearColor(sdl2::pixels::Color::RGB(127, 127,127));

	let mut shader = ae3d::graphics::Shader::Shader::new();
	shader.load("res/shaders/render.vert".to_string(), "res/shaders/render.frag".to_string());
	shader.activate();

	let vertices: Vec<f32>= vec![
		-0.5, -0.5, 1.0,
		0.5, -0.5, 0.0,
		0.0, 0.5, 0.0
	];

	let indices: Vec<u32> = vec![
		0, 1, 2
	];

	let mut vbo = VBO::new(); vbo.set(&vertices);
	let mut vao = VAO::new(); vao.set();
	let mut ibo = IBO::new(); ibo.set(&indices);

	while Window::isOpen()
	{
		Window::update();

		Window::clear();
		unsafe
		{
			gl::DrawElements(
				gl::TRIANGLES,
				indices.len() as i32,
				gl::UNSIGNED_INT,
				std::ptr::null()
			);
			let err = gl::GetError();
			if err != 0 { println!("{err}"); }
		}
		Window::display();
	}
}
