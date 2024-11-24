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
		-0.5, -0.5, 0.0,	0.0, 0.0,
		0.5, -0.5, 0.0,		1.0, 0.0,
		0.5, 0.5, 0.0,		1.0, 1.0,
		-0.5, 0.5, 0.0,		0.0, 1.0
	];

	let indices: Vec<u32> = vec![
		0, 1, 2,
		2, 3, 0
	];

	let view = ae3d::math::GL::mat4_toGL(&glm::ext::translate(
		&ae3d::math::GL::mat4_identity(),
		glm::Vec3::new(0.0, 0.0, -3.0)
	));
	let proj = ae3d::math::GL::mat4_toGL(&glm::ext::perspective(
		glm::radians(45.0),
		Window::getSize().x / Window::getSize().y,
		0.1, 100.0
	));

	shader.setMat4("view".to_string(), &view);
	shader.setMat4("projection".to_string(), &proj);

	let mut m = Mesh::new();
	m.loadTexture("res/tex/test.png".to_string());
	m.gen(&vertices, &indices);

	while Window::isOpen()
	{
		Window::update();

		m.rotateY(45.0 * Window::getDeltaTime());
		
		Window::clear();
		m.draw(&mut shader);
		Window::display();
	}
}
