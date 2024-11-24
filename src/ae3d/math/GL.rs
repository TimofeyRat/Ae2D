pub fn mat4_identity() -> glm::Mat4
{
	glm::Mat4::new(
		glm::Vec4::new(1.0, 0.0, 0.0, 0.0),
		glm::Vec4::new(0.0, 1.0, 0.0, 0.0),
		glm::Vec4::new(0.0, 0.0, 1.0, 0.0),
		glm::Vec4::new(0.0, 0.0, 0.0, 1.0)
	)
}

pub fn mat4_toGL(mat: glm::Mat4) -> [f32; 16]
{
	let mut v = [0.0; 16];
	v[0] = mat.c0.x; v[1] = mat.c0.y; v[2] = mat.c0.z; v[3] = mat.c0.w;
	v[4] = mat.c1.x; v[5] = mat.c1.y; v[6] = mat.c1.z; v[3] = mat.c1.w;
	v[8] = mat.c2.x; v[9] = mat.c2.y; v[10] = mat.c2.z; v[11] = mat.c2.w;
	v[12] = mat.c3.x; v[13] = mat.c3.y; v[14] = mat.c3.z; v[15] = mat.c3.w;
	v
}