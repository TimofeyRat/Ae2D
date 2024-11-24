pub struct VBO
{
	id: u32
}

impl VBO
{
	pub fn new() -> Self
	{
		let mut id = 0;
		unsafe { gl::GenBuffers(1, &mut id); }
		Self { id }
	}

	pub fn bind(&mut self)
	{
		unsafe
		{
			gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
		}
	}

	pub fn unbind(&mut self)
	{
		unsafe
		{
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		}
	}

	pub fn delete(&mut self)
	{
		self.unbind();
		unsafe
		{
			gl::DeleteBuffers(1, &self.id);
		}
	}

	pub fn set(&mut self, vertices: &Vec<f32>)
	{
		self.bind();
		unsafe
		{
			gl::BufferData(
				gl::ARRAY_BUFFER,
				(vertices.len() * size_of::<f32>()) as isize,
				vertices.as_ptr() as *const _,
				gl::DYNAMIC_DRAW
			);
		}
	}
}

impl Drop for VBO
{
	fn drop(&mut self)
	{
		self.delete();
	}
}

pub struct IBO
{
	id: u32
}

impl IBO
{
	pub fn new() -> Self
	{
		let mut id = 0;
		unsafe { gl::GenBuffers(1, &mut id); }
		Self { id }
	}

	pub fn bind(&mut self)
	{
		unsafe
		{
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
		}
	}

	pub fn unbind(&mut self)
	{
		unsafe 
		{
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
		}
	}

	pub fn delete(&mut self)
	{
		self.unbind();
		unsafe
		{
			gl::DeleteBuffers(1, &self.id);
		}
	}

	pub fn set(&mut self, indices: &Vec<u32>)
	{
		self.bind();
		unsafe
		{
			gl::BufferData(
				gl::ELEMENT_ARRAY_BUFFER,
				(indices.len() * size_of::<u32>()) as isize,
				indices.as_ptr() as *const _,
				gl::DYNAMIC_DRAW
			);
		}
	}
}

impl Drop for IBO
{
	fn drop(&mut self)
	{
		self.delete();
	}
}

pub struct VAO
{
	id: u32
}

impl VAO
{
	pub fn new() -> Self
	{
		let mut id = 0;
		unsafe { gl::GenVertexArrays(1, &mut id); }
		Self { id }
	}

	pub fn bind(&mut self)
	{
		unsafe
		{
			gl::BindVertexArray(self.id);
		}
	}

	pub fn unbind(&mut self)
	{
		unsafe
		{
			gl::BindVertexArray(0);
		}
	}

	pub fn delete(&mut self)
	{
		self.unbind();
		unsafe
		{
			gl::DeleteVertexArrays(1, &self.id);
		}
	}

	pub fn set(&mut self)
	{
		self.bind();
		unsafe
		{
			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(
				0,
				3,
				gl::FLOAT,
				gl::FALSE,
				(5 * size_of::<f32>()) as i32,
				std::ptr::null()
			);

			gl::EnableVertexAttribArray(1);
			gl::VertexAttribPointer(
				1,
				2,
				gl::FLOAT,
				gl::FALSE,
				(5 * size_of::<f32>()) as i32,
				(3 * size_of::<f32>()) as *const _
			);
		}
	}
}

impl Drop for VAO
{
	fn drop(&mut self)
	{
		self.delete();
	}
}

pub struct Mesh
{
	vertices: Vec<f32>,
	indices: Vec<u32>,
	vbo: VBO,
	ibo: IBO,
	vao: VAO,
	pos: glm::Vec3,
	scale: glm::Vec3,
	rotation: glm::Vec3,
	texture: u32,
	matrix: [f32; 16],
	reloadMatrix: bool
}

impl Mesh
{
	pub fn new() -> Self
	{
		Self
		{
			vertices: vec![],
			indices: vec![],
			vbo: VBO::new(),
			ibo: IBO::new(),
			vao: VAO::new(),
			pos: glm::Vec3::new(0.0, 0.0, 0.0),
			scale: glm::Vec3::new(1.0, 1.0, 1.0),
			rotation: glm::Vec3::new(0.0, 0.0, 0.0),
			texture: 0,
			matrix: [0.0; 16],
			reloadMatrix: true
		}
	}

	pub fn loadTexture(&mut self, p: String, mode: gl::types::GLenum)
	{
		let res = stb_image::image::load(crate::ae3d::Assets::getCurrentDir() + &p);
		match res
		{
			stb_image::image::LoadResult::Error(err) => { println!("Failed to open texture: {err}"); },
			stb_image::image::LoadResult::ImageU8(data) =>
			{
				unsafe
				{
					gl::GenTextures(1, &mut self.texture);
					gl::BindTexture(gl::TEXTURE_2D, self.texture);
					gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, mode as i32);
					gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, mode as i32);
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
	}

	pub fn gen(&mut self, vertices: &Vec<f32>, indices: &Vec<u32>)
	{
		self.vertices = vertices.to_vec();
		self.indices = indices.to_vec();
		self.vbo.set(&self.vertices);
		self.vao.set();
		self.ibo.set(&self.indices);
	}

	pub fn draw(&mut self, shader: &mut super::Shader::Shader)
	{
		if self.reloadMatrix { self.updateMatrix(); }
		shader.setMat4("model".to_string(), &self.matrix);
		unsafe
		{
			gl::BindTexture(gl::TEXTURE_2D, self.texture);
			self.vao.bind();
			gl::DrawElements(
				gl::TRIANGLES,
				self.indices.len() as i32,
				gl::UNSIGNED_INT,
				std::ptr::null()
			);
			let err = gl::GetError();
			if err != 0 { println!("{err}"); }
		}
	}

	pub fn updateMatrix(&mut self)
	{
		let mut model = crate::ae3d::math::GL::mat4_identity();
		model = glm::ext::translate(&model, -self.pos);
		model = glm::ext::scale(&model, self.scale);
		model = glm::ext::rotate(&model, glm::radians(self.rotation.x), glm::vec3(1.0, 0.0, 0.0));
		model = glm::ext::rotate(&model, glm::radians(self.rotation.y), glm::vec3(0.0, 1.0, 0.0));
		model = glm::ext::rotate(&model, glm::radians(self.rotation.z), glm::vec3(0.0, 0.0, 1.0));
		self.matrix = crate::ae3d::math::GL::mat4_toGL(&model);
		self.reloadMatrix = false;
	}
	
	pub fn translate(&mut self, factor: glm::Vec3)
	{
		self.pos = self.pos + factor;
		self.reloadMatrix = true;
	}

	pub fn setPosition(&mut self, x: glm::Vec3)
	{
		self.pos = x;
		self.reloadMatrix = true;
	}

	pub fn scale(&mut self, factor: glm::Vec3)
	{
		self.scale = self.scale * factor;
		self.reloadMatrix = true;
	}

	pub fn setScale(&mut self, x: glm::Vec3)
	{
		self.scale = x;
		self.reloadMatrix = true;
	}

	pub fn rotateX(&mut self, factor: f32)
	{
		self.rotation.x += factor;
		self.reloadMatrix = true;
	}

	pub fn rotateY(&mut self, factor: f32)
	{
		self.rotation.y += factor;
		self.reloadMatrix = true;
	}

	pub fn rotateZ(&mut self, factor: f32)
	{
		self.rotation.z += factor;
		self.reloadMatrix = true;
	}

	pub fn setRotation(&mut self, x: glm::Vec3)
	{
		self.rotation = x;
		self.reloadMatrix = true;
	}
}