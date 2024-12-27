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

	pub fn set(&mut self, vertexCount: usize, normalsCount: usize)
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

			gl::EnableVertexAttribArray(1);
			gl::VertexAttribPointer(
				1,
				3,
				gl::FLOAT,
				gl::FALSE,
				(3 * size_of::<f32>()) as i32,
				((vertexCount + normalsCount) * size_of::<f32>()) as *const _
			);

			gl::EnableVertexAttribArray(2);
			gl::VertexAttribPointer(
				2,
				2,
				gl::FLOAT,
				gl::FALSE,
				(2 * size_of::<f32>()) as i32,
				(vertexCount * size_of::<f32>()) as *const _
			);
		}
	}

	pub fn gen(&mut self)
	{
		self.bind();
		unsafe
		{
			gl::EnableVertexAttribArray(0);
			gl::EnableVertexAttribArray(1);
			gl::EnableVertexAttribArray(2);
			
			gl::VertexAttribPointer(
				0,
				3,
				gl::FLOAT,
				gl::FALSE,
				(8 * size_of::<f32>()) as i32,
				std::ptr::null()
			);

			gl::VertexAttribPointer(
				1,
				2,
				gl::FLOAT,
				gl::FALSE,
				(8 * size_of::<f32>()) as i32,
				(3 * size_of::<f32>()) as *const std::ffi::c_void
			);

			gl::VertexAttribPointer(
				2,
				3,
				gl::FLOAT,
				gl::FALSE,
				(8 * size_of::<f32>()) as i32,
				(5 * size_of::<f32>()) as *const std::ffi::c_void
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

#[derive(Debug)]
pub struct MaterialUsage
{
	tex: u32,
	ambient: glam::Vec3,
	diffuse: glam::Vec3,
	specular: glam::Vec3,
	start: i32,
	count: i32
}

impl MaterialUsage
{
	pub fn new() -> Self
	{
		Self
		{
			tex: 0,
			ambient: glam::Vec3::ZERO,
			diffuse: glam::Vec3::ZERO,
			specular: glam::Vec3::ZERO,
			count: 0,
			start: 0
		}
	}

	pub fn loadMaterial(&mut self, mtl: std::sync::Arc<obj::Material>)
	{
		if mtl.name.is_empty() { return; }

		self.ambient = glam::Vec3::from_array(mtl.ka.unwrap_or([0.0; 3]));
		self.diffuse = glam::Vec3::from_array(mtl.kd.unwrap_or([0.0; 3]));
		self.specular = glam::Vec3::from_array(mtl.ks.unwrap_or([0.0; 3]));
		self.tex = crate::ae3d::Assets::getTexture(mtl.map_kd.as_ref().unwrap_or(&String::new()).clone());
	}
}

pub struct Mesh
{
	vertices: Vec<f32>,
	vbo: VBO,
	vao: VAO,
	polygons: Vec<MaterialUsage>,
	name: String,
	matrix: [f32; 16],
	pos: glam::Vec3,
	scale: glam::Vec3,
	rotation: glam::Vec3,
	lighting: bool,
	reloadMatrix: bool
}

impl Mesh
{
	pub fn new(path: String) -> Option<Self>
	{
		let result = obj::Obj::load(path.clone());
		if result.is_err() { println!("Failed to load model from {path}: {}", result.err().unwrap()); return None; }

		let mut obj = result.unwrap();
		obj.load_mtls();

		let mut mesh = Mesh
		{
			vertices: vec![],
			vbo: VBO::new(),
			vao: VAO::new(),
			polygons: vec![],
			name: String::new(),
			lighting: true,
			matrix: glam::Mat4::IDENTITY.to_cols_array(),
			pos: glam::Vec3::ZERO,
			reloadMatrix: true,
			rotation: glam::Vec3::ZERO,
			scale: glam::Vec3::ONE
		};

		let mut index = 0;

		for o in &obj.data.objects
		{
			mesh.name = o.name.clone();
			for g in &o.groups
			{
				let mut mu = MaterialUsage::new();
				match g.material.as_ref().unwrap_or(&obj::ObjMaterial::Ref(String::new()))
				{
					obj::ObjMaterial::Mtl(x) => mu.loadMaterial(x.clone()),
					obj::ObjMaterial::Ref(_) => {}
				}

				mu.start = index;

				for p in &g.polys
				{
					let x = &p.0;

					mesh.vertices.append(&mut obj.data.position[x[0].0].to_vec());
					mesh.vertices.append(&mut obj.data.texture.get(x[0].1.unwrap_or(usize::MAX)).unwrap_or(&[0.0; 2]).to_vec());
					mesh.vertices.append(&mut obj.data.normal.get(x[0].2.unwrap_or(usize::MAX)).unwrap_or(&[0.0; 3]).to_vec());
					
					mesh.vertices.append(&mut obj.data.position[x[1].0].to_vec());
					mesh.vertices.append(&mut obj.data.texture.get(x[1].1.unwrap_or(usize::MAX)).unwrap_or(&[0.0; 2]).to_vec());
					mesh.vertices.append(&mut obj.data.normal.get(x[1].2.unwrap_or(usize::MAX)).unwrap_or(&[0.0; 3]).to_vec());

					mesh.vertices.append(&mut obj.data.position[x[2].0].to_vec());
					mesh.vertices.append(&mut obj.data.texture.get(x[2].1.unwrap_or(usize::MAX)).unwrap_or(&[0.0; 2]).to_vec());
					mesh.vertices.append(&mut obj.data.normal.get(x[2].2.unwrap_or(usize::MAX)).unwrap_or(&[0.0; 3]).to_vec());
					
					index += 3;
				}

				mu.count = index - mu.start;

				mesh.polygons.push(mu);
				mesh.vbo.set(&mesh.vertices);
				mesh.vao.gen();
			}
		}

		Some(mesh)
	}

	pub fn updateMatrix(&mut self)
	{
		let mut model = glam::Mat4::IDENTITY;
		model = model.mul_mat4(&glam::Mat4::from_translation(self.pos));
		model = model.mul_mat4(&glam::Mat4::from_rotation_x(self.rotation.x.to_radians()));
		model = model.mul_mat4(&glam::Mat4::from_rotation_y(self.rotation.y.to_radians()));
		model = model.mul_mat4(&glam::Mat4::from_rotation_z(self.rotation.z.to_radians()));
		model = model.mul_mat4(&glam::Mat4::from_scale(self.scale));
		self.matrix = model.to_cols_array();
		self.reloadMatrix = false;
	}

	pub fn draw(&mut self, shader: &mut super::Shader::Shader)
	{
		if self.reloadMatrix { self.updateMatrix(); }
		shader.setMat4("model".to_string(), &self.matrix);
		shader.setBool("enableLight".to_string(), self.lighting);
		self.vao.bind();

		for mu in &self.polygons
		{
			shader.setVec3("ambient".to_string(), mu.ambient.to_array());
			shader.setVec3("diffuse".to_string(), mu.diffuse.to_array());
			shader.setVec3("specular".to_string(), mu.specular.to_array());
			unsafe
			{
				gl::ActiveTexture(gl::TEXTURE0);
				gl::BindTexture(gl::TEXTURE_2D, mu.tex);
				shader.setInt("tex".to_string(), 0);
				shader.setBool("enableTexture".to_string(), mu.tex != 0);
				gl::DrawArrays(gl::TRIANGLES, mu.start, mu.count);
				let err = gl::GetError();
				if err != 0 { println!("{err}"); }
			}
		}
	}
	
	pub fn translate(&mut self, factor: glam::Vec3)
	{
		self.pos = self.pos + factor;
		self.reloadMatrix = true;
	}

	pub fn setPosition(&mut self, x: glam::Vec3)
	{
		self.pos = x;
		self.reloadMatrix = true;
	}

	pub fn scale(&mut self, factor: glam::Vec3)
	{
		self.scale = self.scale * factor;
		self.reloadMatrix = true;
	}

	pub fn setScale(&mut self, x: glam::Vec3)
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

	pub fn setRotation(&mut self, x: glam::Vec3)
	{
		self.rotation = x;
		self.reloadMatrix = true;
	}

	pub fn setLighting(&mut self, light: bool)
	{
		self.lighting = light;
	}
}