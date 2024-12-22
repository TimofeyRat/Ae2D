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

struct Polygon
{
	vertices: Vec<f32>,
	vbo: VBO,
	vao: VAO,
	tex0: u32,
	ambient: glam::Vec3,
	diffuse: glam::Vec3,
	specular: glam::Vec3
}

impl Polygon
{
	pub fn new() -> Self
	{
		Polygon
		{
			vao: VAO::new(),
			vbo: VBO::new(),
			vertices: vec![],
			tex0: 0,
			ambient: glam::vec3(0.0, 0.0, 0.0),
			diffuse: glam::vec3(0.0, 0.0, 0.0),
			specular: glam::vec3(0.0, 0.0, 0.0)
		}
	}

	pub fn loadMaterial(&mut self, mtl: std::sync::Arc<obj::Material>)
	{
		if mtl.name.is_empty() { return; }

		if mtl.ka.is_some()
		{
			let arr = mtl.ka.unwrap();
			self.ambient = glam::vec3(arr[0], arr[1], arr[2]);
		}
		if mtl.kd.is_some()
		{
			let arr = mtl.kd.unwrap();
			self.diffuse = glam::vec3(arr[0], arr[1], arr[2]);
		}
		if mtl.ks.is_some()
		{
			let arr = mtl.ks.unwrap();
			self.specular = glam::vec3(arr[0], arr[1], arr[2]);
		}

		if mtl.map_kd.is_some()
		{
			self.tex0 = crate::ae3d::Assets::getTexture(mtl.map_kd.as_ref().unwrap().clone());
		}
	}

	pub fn generate(data: &obj::ObjData, base: obj::SimplePolygon, mtl: Option<obj::ObjMaterial>) -> Self
	{
		let mut p = Polygon::new();

		for v in base.0
		{
			let pos = data.position.get(v.0).unwrap_or(&[0.0, 0.0, 0.0]);
			let uv = data.texture.get(v.1.unwrap_or(usize::MAX)).unwrap_or(&[0.0, 0.0]);
			let normal = data.normal.get(v.2.unwrap_or(usize::MAX)).unwrap_or(&[0.0, 0.0, 0.0]);

			p.vertices.push(*pos.get(0).unwrap());
			p.vertices.push(*pos.get(1).unwrap());
			p.vertices.push(*pos.get(2).unwrap());
			p.vertices.push(*uv.get(0).unwrap());
			p.vertices.push(*uv.get(1).unwrap());
			p.vertices.push(*normal.get(0).unwrap());
			p.vertices.push(*normal.get(1).unwrap());
			p.vertices.push(*normal.get(2).unwrap());
		}

		p.vbo.set(&p.vertices);
		p.vao.gen();

		if mtl.is_some()
		{
			match mtl.unwrap()
			{
				obj::ObjMaterial::Mtl(x) => { p.loadMaterial(x); }
				obj::ObjMaterial::Ref(x) =>
				{
					for mtl in data.material_libs.clone()
					{
						for m in mtl.materials.clone()
						{
							if m.name == x
							{
								p.loadMaterial(m);
							}
						}
					}
				}
			}
		}

		p
	}

	pub fn draw(&mut self, shader: &mut super::Shader::Shader)
	{
		shader.setVec3("ambient".to_string(), self.ambient.to_array());
		shader.setVec3("diffuse".to_string(), self.diffuse.to_array());
		shader.setVec3("specular".to_string(), self.specular.to_array());
		unsafe
		{
			gl::ActiveTexture(gl::TEXTURE0);
			gl::BindTexture(gl::TEXTURE_2D, self.tex0);
		}
		shader.setInt("tex".to_string(), 0);
		shader.setBool("enableTexture".to_string(), self.tex0 != 0);
		self.vao.bind();
		unsafe
		{
			let size = (self.vertices.len() / 8) as i32;
			gl::DrawArrays(
				if size == 3 { gl::TRIANGLES } else { gl::QUADS },
				0,
				size
			);
			let err = gl::GetError();
			if err != 0 { println!("{err}"); }
		}
	}
}

pub struct Mesh
{
	pos: glam::Vec3,
	scale: glam::Vec3,
	rotation: glam::Vec3,
	matrix: [f32; 16],
	reloadMatrix: bool,
	polygons: Vec<Polygon>,
	name: String,
	light: bool
}

impl Mesh
{
	pub fn new() -> Self
	{
		Self
		{
			pos: glam::Vec3::new(0.0, 0.0, 0.0),
			scale: glam::Vec3::new(1.0, 1.0, 1.0),
			rotation: glam::Vec3::new(0.0, 0.0, 0.0),
			matrix: [0.0; 16],
			reloadMatrix: true,
			polygons: vec![],
			name: String::new(),
			light: true
		}
	}

	pub fn draw(&mut self, shader: &mut super::Shader::Shader)
	{
		if self.reloadMatrix { self.updateMatrix(); }
		shader.setMat4("model".to_string(), &self.matrix);
		shader.setVec3("ambient".to_string(), [0.0; 3]);
		shader.setVec3("diffuse".to_string(), [0.0; 3]);
		shader.setInt("enableLight".to_string(), self.light as i32);
		for p in self.polygons.iter_mut()
		{
			p.draw(shader);
		}
	}

	fn updateMatrix(&mut self)
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
	
	pub fn loadFromFile(&mut self, path: String)
	{
		let result = obj::Obj::load(path);
		if result.is_err() { println!("Failed to load model: {}", result.err().unwrap()); return; }
		let mut obj = result.unwrap();
		obj.load_mtls();
		for o in &obj.data.objects
		{
			self.name = o.name.clone();
			for g in &o.groups
			{
				for p in &g.polys
				{
					self.polygons.push(Polygon::generate(&obj.data, p.clone(), g.material.clone()));
				}
			}
		}
	}

	pub fn setApplyLighting(&mut self, light: bool)
	{
		self.light = light;
	}

	pub fn shouldApplyLighting(&mut self) -> bool
	{
		self.light
	}
}

pub struct MaterialUsage
{
	tex: u32,
	ambient: glam::Vec3,
	diffuse: glam::Vec3,
	specular: glam::Vec3,
	start: i32,
	count: isize
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

		if mtl.ka.is_some()
		{
			let arr = mtl.ka.unwrap();
			self.ambient = glam::vec3(arr[0], arr[1], arr[2]);
		}
		if mtl.kd.is_some()
		{
			let arr = mtl.kd.unwrap();
			self.diffuse = glam::vec3(arr[0], arr[1], arr[2]);
		}
		if mtl.ks.is_some()
		{
			let arr = mtl.ks.unwrap();
			self.specular = glam::vec3(arr[0], arr[1], arr[2]);
		}

		if mtl.map_kd.is_some()
		{
			self.tex = crate::ae3d::Assets::getTexture(mtl.map_kd.as_ref().unwrap().clone());
		}
	}
}

pub struct NewMesh
{
	vertices: Vec<f32>,
	vbo: VBO,
	vao: VAO,
	polygons: Vec<MaterialUsage>,
	name: String
}

impl NewMesh
{
	pub fn new(path: String) -> Option<Self>
	{
		let result = obj::Obj::load(path.clone());
		if result.is_err() { println!("Failed to load model from {path}: {}", result.err().unwrap()); return None; }

		let mut obj = result.unwrap();
		obj.load_mtls();

		let mut mesh = NewMesh
		{
			vertices: vec![],
			vbo: VBO::new(),
			vao: VAO::new(),
			polygons: vec![],
			name: String::new()
		};

		for o in &obj.data.objects
		{
			mesh.name = o.name.clone();
			for g in &o.groups
			{
				for p in &g.polys
				{
					// p.0
				}
			}
		}

		Some(mesh)
	}
}