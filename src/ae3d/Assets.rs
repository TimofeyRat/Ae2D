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

fn loadTexture(path: String) -> u32
{
	let res = stb_image::image::load(path.clone());
	match res
	{
		stb_image::image::LoadResult::Error(err) => { println!("Failed to load texture {path}: {err}"); return 0; }
		stb_image::image::LoadResult::ImageF32(data) => unsafe
		{
			let mut tex: u32 = 0;
			gl::GenTextures(1, &mut tex);
			gl::BindTexture(gl::TEXTURE_2D, tex);

			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

			gl::TexImage2D(
				gl::TEXTURE_2D,
				0,
				gl::RGBA as i32,
				data.width as i32,
				data.height as i32,
				0,
				gl::RGBA,
				gl::FLOAT,
				data.data.as_ptr() as *const _
			);

			return tex;
		}
		stb_image::image::LoadResult::ImageU8(data) => unsafe
		{
			let mut tex: u32 = 0;
			gl::GenTextures(1, &mut tex);
			gl::BindTexture(gl::TEXTURE_2D, tex);

			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

			gl::TexImage2D(
				gl::TEXTURE_2D,
				0,
				gl::RGBA as i32,
				data.width as i32,
				data.height as i32,
				0,
				gl::RGBA,
				gl::UNSIGNED_BYTE,
				data.data.as_ptr() as *const _
			);

			return tex;
		}
	}
}

pub struct Texture
{
	path: String,
	id: u32
}

pub static mut textures: Vec<Texture> = vec![];

pub fn getTexture(path: String) -> u32
{
	unsafe
	{
		for tex in textures.iter()
		{
			if tex.path == path.clone() { return tex.id; }
		}
		let id = loadTexture(path.clone());
		textures.push(Texture { id, path });
		return id;
	}
}