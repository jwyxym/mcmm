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