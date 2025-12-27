use walkdir::{
	WalkDir
};

use anyhow::{
	Error, anyhow
};

use std::{
	fs::{
		OpenOptions,
		read_to_string,
		File
	},
	path::{
		Path
	},
	io::{
		Write
	}
};

use regex::{
	Regex
};

use basic_toml::{
	from_str,
	to_string
};

use fs2::{
	FileExt
};

use crate::tool::config::Config;

pub async fn walk<T: AsRef<Path>>(dir: T, mut callback: impl AsyncFnMut(&str, &str, &str) -> ()) {
	for entry in WalkDir::new(dir) {
		if let Ok(e) = entry {
			let path = e.path();
			if path.is_file() {
				if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
					if let Some(ext) = path.extension().and_then(|n| n.to_str()) {
						if let Some(path) = path.as_os_str().to_str() {
							callback(path, name, ext).await;
						}
					}
				}
			}
		}
	}
}

pub fn write(toml: &str, config: Config) -> Result<(), Error> {
	let toml_str: String = to_string(&config).map_err(|_| anyhow!("文件内容格式错误"))?;
	let mut file: File = OpenOptions::new()
		.create(true)
		.write(true)
		.truncate(true)
		.open(toml)
		.map_err(|_| anyhow!("文件写入错误{}", toml))?;
	file.lock_exclusive()?;
	write!(file, "{}", toml_str)
		.map_err(|_| anyhow!("文件写入错误{}", toml))?;
	Ok(())
}

pub fn read(toml: &str) -> Result<Config, Error> {
	let file: String = read_to_string(toml).map_err(|_| anyhow!("文件不存在: {}", toml))?;
	from_str(&file).map_err(|_| anyhow!("文件内容格式错误: {}", toml))
}

pub fn write_string(path: &str, string: &str) -> Result<(), Error> {
	let mut file: File = OpenOptions::new()
		.create(true)
		.write(true)
		.truncate(true)
		.open(path)
		.map_err(|_| anyhow!("文件写入错误{}", path))?;
	file.lock_exclusive()?;
	write!(file, "{}", string)
		.map_err(|_| anyhow!("文件写入错误{}", path))?;
	Ok(())
}

pub fn read_string(path: &str) -> Result<String, Error> {
	Ok(read_to_string(path)?)
}

pub fn read_script(path: &str) -> Result<String, Error> {
	let script: String = read_string(path)?;
	let re: Regex = Regex::new(r"(?m)^java\s+(.*)$")?;
	if let Some(scripts) = re.captures(&script) {
		if let Some(script) = scripts.get(0) {
			return Ok(script.as_str().trim().to_string());
		}
	}
	Err(anyhow!(""))
}