#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_must_use)]
mod ae3d;

use ae3d::Window::Window;

fn main()
{
	Window::init();

	let mut shader = ae3d::graphics::Shader::Shader::new();
	shader.load("res/shaders/render.vert".to_string(), "res/shaders/render.frag".to_string());

	let mut vertices= vec![
		-0.5, -0.5, 0.0,
		0.5, -0.5, 0.0,
		0.0, 0.5, 0.0
	];

	let mut vbo: u32 = 0;
	let mut vao: u32 = 0;
	unsafe
	{
		gl::GenBuffers(1, &mut vbo);
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::GenVertexArrays(1, &mut vao);
		gl::BindVertexArray(vao);

		gl::BufferData(gl::ARRAY_BUFFER, 9 * 8, vertices.as_mut_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
		gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * 8, std::ptr::null());
		gl::EnableVertexAttribArray(0);
	}

	while Window::isOpen()
	{
		Window::update();


		Window::clear();
		// shader.activate();
		unsafe
		{
			gl::BindVertexArray(vao);
			gl::DrawArrays(gl::TRIANGLES,  0, 3);
		}
		Window::display();
	}
}
