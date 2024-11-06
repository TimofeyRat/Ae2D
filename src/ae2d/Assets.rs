use std::io::Read;

pub fn getCurrentDir() -> String
{
	String::from(std::env::current_dir().unwrap().to_str().unwrap()) + "/"
}
pub fn readFile(path: String) -> Option<String>
{
	let file = std::fs::File::open(getCurrentDir() + &path.clone());
	if file.is_err() { None }
	else
	{
		let mut buf = String::new();
		file.unwrap().read_to_string(&mut buf);
		Some(buf)
	}
}