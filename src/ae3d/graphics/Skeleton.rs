use super::Mesh::Mesh;

pub struct Skeleton
{
	mesh: Option<Mesh>
}

impl Skeleton
{
	pub fn new(model: String) -> Self
	{
		Self
		{
			mesh: Mesh::new(model)
		}
	}

	pub fn getMesh(&mut self) -> &mut Mesh
	{
		self.mesh.as_mut().unwrap()
	}
}