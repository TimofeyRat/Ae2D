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

pub struct Polygon
{
	vertices: Vec<f32>,
	vbo: VBO,
	vao: VAO,
	material: std::sync::Arc<obj::Material>,
	tex0: u32
	/*
		Implementation:
			1. Members are material and 3 vertices(pos/normal/texture)
			2. When loading face of mesh convert the IDs of vertices to the vertices itself and push here not 1/0/0 but (xyz)/(xyz)/(xy)
			3. Create local vao/vbo
	*/
}

impl Polygon
{
	pub fn new() -> Self
	{
		Polygon
		{
			material: obj::Material::new(String::new()).into(),
			vao: VAO::new(),
			vbo: VBO::new(),
			vertices: vec![],
			tex0: 0
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
				obj::ObjMaterial::Mtl(x) => { p.material = x; }
				obj::ObjMaterial::Ref(x) =>
				{
					for mtl in data.material_libs.clone()
					{
						for m in mtl.materials.clone()
						{
							if m.name == x
							{
								p.material = m;
							}
						}
					}
				}
			}
		}

		if p.material.map_kd.is_some()
		{
			let res = stb_image::image::load(p.material.map_kd.as_ref().unwrap().clone());
			match res
			{
				stb_image::image::LoadResult::Error(e) => println!("Failed to load texture: {e}"),
				stb_image::image::LoadResult::ImageF32(_) => {}
				stb_image::image::LoadResult::ImageU8(data) =>
				{
					unsafe
					{
						gl::GenTextures(1, &mut p.tex0);
						gl::BindTexture(gl::TEXTURE_2D, p.tex0);
						
						gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
						gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
						gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
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
							data.data.as_ptr() as *const std::ffi::c_void
						);
					}
				}
			}
		}

		p
	}

	pub fn draw(&mut self, shader: &mut super::Shader::Shader)
	{
		shader.setVec3("ambient".to_string(), &self.material.ka.unwrap_or([1.0; 3]));
		shader.setVec3("diffuse".to_string(), &self.material.kd.unwrap_or([1.0; 3]));
		shader.setVec3("specular".to_string(), &self.material.ks.unwrap_or([1.0; 3]));
		// self.material.map
		if self.tex0 != 0
		{
			unsafe
			{
				gl::ActiveTexture(gl::TEXTURE0);
				gl::BindTexture(gl::TEXTURE_2D, self.tex0);
				shader.setInt("tex".to_string(), 0);
			}
		}
		self.vao.bind();
		unsafe
		{
			gl::DrawArrays(
				gl::TRIANGLES,
				0,
				3
			);
			let err = gl::GetError();
			if err != 0 { println!("{err}"); }
		}
	}
}

pub struct Mesh
{
	pos: glm::Vec3,
	scale: glm::Vec3,
	rotation: glm::Vec3,
	matrix: [f32; 16],
	reloadMatrix: bool,
	polygons: Vec<Polygon>
}

impl Mesh
{
	pub fn new() -> Self
	{
		Self
		{
			pos: glm::Vec3::new(0.0, 0.0, 0.0),
			scale: glm::Vec3::new(1.0, 1.0, 1.0),
			rotation: glm::Vec3::new(0.0, 0.0, 0.0),
			matrix: [0.0; 16],
			reloadMatrix: true,
			polygons: vec![]
		}
	}

	pub fn draw(&mut self, shader: &mut super::Shader::Shader)
	{
		if self.reloadMatrix { self.updateMatrix(); }
		shader.setMat4("model".to_string(), &self.matrix);
		for p in self.polygons.iter_mut()
		{
			p.draw(shader);
		}
	}

	fn updateMatrix(&mut self)
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
	
	pub fn loadFromFile(&mut self, path: String)
	{
		let result = obj::Obj::load(path);
		if result.is_err() { println!("Failed to load model: {}", result.err().unwrap()); return; }
		let mut obj = result.unwrap();
		obj.load_mtls();
		for o in &obj.data.objects
		{
			for g in &o.groups
			{
				for p in &g.polys
				{
					self.polygons.push(Polygon::generate(&obj.data, p.clone(), g.material.clone()));
				}
			}
		}
	}
}