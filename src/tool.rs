mod config;
use config::{
	Config
};
mod request;
mod structs;
use structs::{
	Mods
};
mod file;
mod spinner;

use crate::{
	TOML
};

use anyhow::{
	Error, anyhow
};

use std::{
	fs::{
		create_dir_all,
		read_to_string,
		remove_file
	},
	path::{
		Path,
		PathBuf
	},
	io::{
		stdin
	}
};

use basic_toml::{
	from_str
};

use tokio::{
	spawn,
	task::{
		JoinHandle
	}
};

use indicatif::{
	ProgressBar
};

pub async fn init(version: &str, loader: &str, dir: &str) -> Result<(), Error> {
	let mut version: String = version.to_string();
	let mut loader: String = loader.to_string();
	if version == String::from("") {
		println!("请输入版本['1.21.10', '1.21.1'...]:");
	}
	while version == String::from("") {
		stdin().read_line(&mut version).expect("");
	}
	if loader == String::from("") {
		println!("请输入加载器['forge', 'neoforge'...]:");
	}
	while loader == String::from("") {
		stdin().read_line(&mut loader).expect("");
	}
	spinner::new();
	let config: Config = Config::new(version.trim(), loader.trim(), dir);
	file::write(TOML, config)
}

pub async fn install() -> Result<(), Error> {
	spinner::new();
	let config: Config = file::read(TOML)?;
	let dir: &str = config.dir();
	create_dir_all(dir).map_err(|_| anyhow!("路径错误: {}", dir))?;
	let mut tasks: Vec<JoinHandle<()>> = Vec::new();
	let mut names: Vec<String> = Vec::new();
	config.get_content(async |name, url| {
		let path: PathBuf = Path::new(dir).join(name);
		let url: String = url.to_string();
		names.push(name.to_string());
		if !path.exists() {
			let task = spawn(async move {
				let _ = request::download(url, path).await;
			});
			tasks.push(task);
		}
	}).await;
	for task in tasks {
		let _ = task.await;
	}
	clear(dir, names).await
}

pub async fn search(name: &str) -> Result<(), Error> {
	let file: String = read_to_string(TOML).map_err(|_| anyhow!("文件不存在: {}", TOML))?;
	let config: Config = from_str(&file).map_err(|_| anyhow!("文件内容格式错误: {}", TOML))?;
	let mut offset: usize = 0;
	let mut input: String = String::from("");
	let mut list: Mods;
	loop {
		let s: ProgressBar = spinner::new();
		if let Ok(mods) = request::search_mods(name, config.loader(), config.version(), offset).await {
			s.finish();
			println!("输入 n 下一页, p 上一页, {{序号}}选择, 其他退出");
			mods.log();
			println!("第{}页/共{}页", mods.offset(), mods.limit());
			list = mods;
		} else {
			break;
		}
		println!("请选择mod序号:");
        stdin().read_line(&mut input).expect("");
		match input.trim() {
			"n" => if offset < usize::MAX { offset += 1; },
			"p" => if offset > 0 { offset -= 1; },
			_ => {
				if let Some(num) = input.trim().parse::<usize>().ok() {
					if let Some(m) = list.index(num){
						return add(m.id()).await;
					}
				}
				break;
			}
		}
	}
	Ok(())
}

pub async fn add(id: &str) -> Result<(), Error> {
	spinner::new();
	let config: Config = file::read(TOML)?;
	if let Ok(m) = request::search_mod(id).await {
		if let Some(m) = m.chk(config.version(), config.loader()).file() {
			let mut config: Config = file::read(TOML)?;
			config.push(m.name(), m.url());
			return file::write(TOML, config);
		}
	}
	Err(anyhow!("添加mod失败: {}", id))
}

pub async fn clear<T: AsRef<Path>>(dir: T, names: Vec<String>) -> Result<(), Error> {
	spinner::new();
	file::walk(dir, async |path, name, _| {
		if !names.contains(&name.to_string()) {
			if remove_file(path).is_err() {
				let _ = anyhow!("删除文件失败：{}", name);
			}
		}
	}).await;
	Ok(())
}