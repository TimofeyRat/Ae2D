#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_must_use)]
mod ae3d;

use ae3d::{graphics::Mesh::*, Assets, Window::Window};

fn main()
{
	Window::init();
	Window::setClearColor(sdl2::pixels::Color::RGB(127, 127,127));

	let mut shader = ae3d::graphics::Shader::Shader::new();
	shader.load("res/shaders/render.vert".to_string(), "res/shaders/render.frag".to_string());
	shader.activate();

	let vertices: Vec<f32>= vec![
		-0.5, -0.5, 0.0,	0.0, 0.0,
		0.5, -0.5, 0.0,		1.0, 0.0,
		0.5, 0.5, 0.0,		1.0, 1.0,
		-0.5, 0.5, 0.0,		0.0, 1.0
	];

	let indices: Vec<u32> = vec![
		0, 1, 2,
		2, 3, 0
	];
	
	let mut tex: u32 = 0;
	
	let res = stb_image::image::load(Assets::getCurrentDir() + "res/tex/nefor.png");
	match res
	{
		stb_image::image::LoadResult::Error(err) => { println!("Failed to open texture: {err}"); },
		stb_image::image::LoadResult::ImageU8(data) =>
		{
			unsafe
			{
				gl::GenTextures(1, &mut tex);
				gl::BindTexture(gl::TEXTURE_2D, tex);
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
				gl::TexImage2D(
					gl::TEXTURE_2D,
					0,
					gl::RGBA as i32,
					data.width as i32,
					data.height as i32,
					0,
					gl::RGBA,
					gl::UNSIGNED_BYTE,
					data.data.as_ptr() as *const _
				);
				gl::GenerateMipmap(gl::TEXTURE_2D);
			}
		},
		_ => {}
	}

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
