mod ae2d;
use ae2d::{window::Window, math};

fn main()
{
	Window::init(math::Point::new(1024.0, 576.0), "Ae2D");
}
