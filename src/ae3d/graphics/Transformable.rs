pub struct Transformable2D
{
	position: glam::Vec2,
	rotation: f32,
	scale: glam::Vec2,
	origin: glam::Vec2,
	model: glam::Mat4,
	reloadModel: bool
}

impl Transformable2D
{
	pub fn new() -> Self
	{
		Self
		{
			position: glam::Vec2::ZERO,
			rotation: 0.0,
			scale: glam::Vec2::ONE,
			origin: glam::Vec2::ZERO,
			model: glam::Mat4::IDENTITY,
			reloadModel: true
		}
	}

	fn update(&mut self)
	{
		self.model =
			glam::Mat4::from_translation(glam::vec3(self.position.x, self.position.y, 0.0))
			.mul_mat4(&glam::Mat4::from_rotation_z(self.rotation))
			.mul_mat4(&glam::Mat4::from_translation(-glam::vec3(self.origin.x, self.origin.y, 0.0)))
			.mul_mat4(&glam::Mat4::from_scale(glam::vec3(self.scale.x, self.scale.y, 1.0)));
	}

	pub fn getMatrix(&mut self) -> glam::Mat4
	{
		if self.reloadModel { self.update(); }

		self.model
	}
}