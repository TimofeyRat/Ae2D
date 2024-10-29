use crate::math;

pub struct Window {}

impl Window
{
	pub fn init(size: math::Point, title: &str)
	{
		println!("Initialized window of size {}x{} called \"{}\"", size.x, size.y, title);
	}
}