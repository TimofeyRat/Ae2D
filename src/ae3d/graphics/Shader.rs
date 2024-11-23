pub struct Shader
{
	vertex: u32,
	fragment: u32,
	program: u32
}

impl Shader
{
	pub fn new() -> Self
	{
		Self
		{
			vertex: 0,
			fragment: 0,
			program: 0
		}
	}

	fn loadShader(p: String, vertex: bool) -> u32
	{
		let shader: u32;
		let res = crate::ae3d::Assets::readFile(p.clone());
		if res.is_none()
		{
			println!("Failed to load shader from {p}");
			return 0;
		}

		let code = res.unwrap();

		unsafe
		{
			shader = gl::CreateShader(if vertex { gl::VERTEX_SHADER } else { gl::FRAGMENT_SHADER });
			gl::ShaderSource(
				shader,
				1,
				&(code.clone().as_ptr() as *const i8),
				std::ptr::null()
			);
			gl::CompileShader(shader);
			let mut success = 0;
			gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
			if success == 0
			{
				let infoLog = std::ptr::null_mut();
				let mut written = 0;
				gl::GetShaderInfoLog(shader, 512, &mut written, infoLog);
				println!("Failed to compile shader from {p}: {}", String::from_raw_parts(infoLog as *mut u8, written as usize, 512));
			}
		}
		shader
	}

	fn linkProgram(&mut self)
	{
		unsafe
		{
			self.program = gl::CreateProgram();
			gl::AttachShader(self.program, self.vertex);
			gl::AttachShader(self.program, self.fragment);
			gl::LinkProgram(self.program);
			gl::DeleteShader(self.vertex);
			gl::DeleteShader(self.fragment);
		}
	}

	pub fn loadVertex(&mut self, p: String)
	{
		self.vertex = Shader::loadShader(p, true);
		self.linkProgram();
	}

	pub fn loadFragment(&mut self, p: String)
	{
		self.fragment = Shader::loadShader(p, false);
		self.linkProgram();
	}

	pub fn load(&mut self, vertex: String, fragment: String)
	{
		self.vertex = Shader::loadShader(vertex, true);
		self.fragment = Shader::loadShader(fragment, false);
		self.linkProgram();
	}

	pub fn activate(&mut self)
	{
		unsafe
		{
			gl::UseProgram(self.program);
		}
	}
}