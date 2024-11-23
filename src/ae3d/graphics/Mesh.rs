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
				(3 * size_of::<f32>()) as i32,
				std::ptr::null()
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