#[derive(PartialEq)]
pub enum CameraMode
{
	FirstPerson = 0,
	ThirdPerson = 1 
}

pub struct Camera
{
	mode: CameraMode,
	pos: glam::Vec3,
	yaw: f32,
	pitch: f32,
	shader: super::graphics::Shader::Shader,
	up: glam::Vec3,
	projection: [f32; 16],
	view: [f32; 16],
	reloadProjection: bool,
	reloadView: bool,
	fov: f32,
	thirdPersonDistance: f32,
	direction: glam::Vec3
}

impl Camera
{
	pub fn perspective(mode: CameraMode, fov: f32) -> Self
	{
		Self
		{
			thirdPersonDistance: match mode { CameraMode::FirstPerson => { 0.0 }, CameraMode::ThirdPerson => { 1.0 } },
			mode,
			fov,
			yaw: 0.0,
			pitch: 0.0,
			pos: glam::vec3(0.0, 0.0, 0.0),
			projection: [0.0; 16],
			reloadProjection: true,
			reloadView: true,
			shader: super::graphics::Shader::Shader::new(),
			up: glam::vec3(0.0, 1.0, 0.0),
			view: [0.0; 16],
			direction: glam::vec3(0.0, 0.0, 1.0)
		}
	}

	pub fn draw(&mut self, mesh: &mut super::graphics::Mesh::Mesh)
	{
		if self.reloadProjection { self.updateProjection(); }
		if self.reloadView { self.updateView(); }
		if !self.shader.isLoaded()
		{
			self.shader.load(
				"res/shaders/render.vert".to_string(),
				"res/shaders/render.frag".to_string()
			);
		}

		self.shader.activate();

		self.shader.setMat4("projection".to_string(), &self.projection);
		self.shader.setMat4("view".to_string(), &self.view);
		self.shader.setVec3("camPos".to_string(), self.pos.to_array());
		self.shader.setVec3("lightPos".to_string(), glam::Vec3::new(-25.0, 25.0, 25.0).to_array());
		mesh.draw(&mut self.shader);
	}

	pub fn updateProjection(&mut self)
	{
		let s = super::Window::Window::getSize();
		self.projection = glam::Mat4::perspective_rh_gl(
			self.fov,
			s.x / s.y,
			0.01, 1000.0
		).to_cols_array();
		self.reloadProjection = false;
	}

	pub fn updateView(&mut self)
	{
		match self.mode
		{
			CameraMode::FirstPerson => { self.updateFP_View(); },
			CameraMode::ThirdPerson => { self.updateTP_View(); }
		}
	}

	pub fn updateFP_View(&mut self)
	{
		self.view = glam::Mat4::look_at_rh(
			self.pos,
			self.pos + self.direction,
			self.up
		).to_cols_array();
		self.reloadView = false;
	}

	pub fn updateTP_View(&mut self)
	{
		self.view = glam::Mat4::look_at_rh(
			self.pos - (self.direction * self.thirdPersonDistance),
			self.pos,
			self.up
		).to_cols_array();
		self.reloadView = false;
	}

	pub fn translate(&mut self, factor: glam::Vec3)
	{
		self.pos = self.pos + factor;
		self.reloadView = true;
	}

	pub fn fly(&mut self, factor: glam::Vec3)
	{
		let dir = glam::vec3(self.direction.x, 0.0, self.direction.z);
		self.pos = self.pos + dir * factor.z;
		self.pos = self.pos + self.up * factor.y;
		self.pos = self.pos + glam::Vec3::cross(dir, self.up) * factor.x;
		self.reloadView = true;
	}

	pub fn rotate(&mut self, factor: glam::Vec2)
	{
		self.yaw += factor.x;
		self.pitch -= factor.y;
		self.pitch = self.pitch.clamp(-89.0, 89.0);
		self.direction.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
		self.direction.y = self.pitch.to_radians().sin();
		self.direction.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();
		self.direction = glam::Vec3::normalize(self.direction);
		self.reloadView = true;
	}

	pub fn tpSetDistance(&mut self, x: f32)
	{
		self.thirdPersonDistance = x;
	}

	pub fn tpGetDistance(&mut self) -> f32
	{
		self.thirdPersonDistance
	}

	pub fn lookAt(&mut self, target: glam::Vec3)
	{
		self.direction = glam::Vec3::normalize(target - self.pos);
	}

	pub fn getPosition(&mut self) -> glam::Vec3
	{
		self.pos
	}
}