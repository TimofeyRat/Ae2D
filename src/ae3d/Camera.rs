#[derive(PartialEq)]
pub enum CameraMode
{
	FirstPerson = 0,
	ThirdPerson = 1 
}

pub struct Camera
{
	mode: CameraMode,
	pos: glm::Vec3,
	yaw: f32,
	pitch: f32,
	shader: super::graphics::Shader::Shader,
	up: glm::Vec3,
	projection: [f32; 16],
	view: [f32; 16],
	reloadProjection: bool,
	reloadView: bool,
	fov: f32,
	thirdPersonDistance: f32,
	direction: glm::Vec3
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
			pos: glm::vec3(0.0, 0.0, 0.0),
			projection: [0.0; 16],
			reloadProjection: true,
			reloadView: true,
			shader: super::graphics::Shader::Shader::new(),
			up: glm::vec3(0.0, 1.0, 0.0),
			view: [0.0; 16],
			direction: glm::vec3(0.0, 0.0, 1.0)
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
		self.shader.setVec3("camPos".to_string(), self.pos.as_array());
		self.shader.setVec3("lightPos".to_string(), glm::vec3(100.0, 100.0, 100.0).as_array());
		mesh.draw(&mut self.shader);
	}

	pub fn updateProjection(&mut self)
	{
		let s = super::Window::Window::getSize();
		self.projection = super::math::GL::mat4_toGL(&glm::ext::perspective(
			self.fov,
			s.x / s.y,
			0.01, 2000.0
		));
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
		self.view = super::math::GL::mat4_toGL(&glm::ext::look_at(
			self.pos,
			self.pos + self.direction,
			self.up
		));
		self.reloadView = false;
	}

	pub fn updateTP_View(&mut self)
	{
		self.view = super::math::GL::mat4_toGL(&glm::ext::look_at(
			self.pos - (self.direction * self.thirdPersonDistance),
			self.pos,
			self.up
		));
		self.reloadView = false;
	}

	pub fn translate(&mut self, factor: glm::Vec3)
	{
		self.pos = self.pos + factor;
		self.reloadView = true;
	}

	pub fn fly(&mut self, factor: glm::Vec3)
	{
		let dir = glm::vec3(self.direction.x, 0.0, self.direction.z);
		self.pos = self.pos + dir * factor.z;
		self.pos = self.pos + self.up * factor.y;
		self.pos = self.pos + glm::cross(dir, self.up) * factor.x;
		self.reloadView = true;
	}

	pub fn rotate(&mut self, factor: glm::Vec2)
	{
		self.yaw += factor.x;
		self.pitch -= factor.y;
		self.pitch = self.pitch.clamp(-89.0, 89.0);
		self.direction.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
		self.direction.y = self.pitch.to_radians().sin();
		self.direction.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();
		self.direction = glm::normalize(self.direction);
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

	pub fn lookAt(&mut self, target: glm::Vec3)
	{
		self.direction = glm::normalize(target - self.pos);
	}

	pub fn getPosition(&mut self) -> glm::Vec3
	{
		self.pos
	}
}