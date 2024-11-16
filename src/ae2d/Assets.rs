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

pub fn openFile(path: String) -> Option<std::fs::File>
{
	let res = std::fs::File::open((getCurrentDir() + &path.to_string()).as_str());
	if res.is_err()
	{
		println!("Failed to open file {}: {}", path, res.err().unwrap());
		None
	}
	else
	{
		Some(res.unwrap())
	}
}

pub fn readJSON(path: String) -> Option<json::JsonValue>
{
	let code = readFile(path.clone());

	if code.is_none()
	{
		println!("Failed to open file {}", path.clone());
		return None
	}

	let parsedRes = json::parse(code.unwrap().as_str());
	if parsedRes.is_err()
	{
		println!("Failed to parse JSON from {}: {}", path, parsedRes.err().unwrap());
		return None
	}

	Some(parsedRes.unwrap())
}