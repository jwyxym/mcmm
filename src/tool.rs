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
mod command;

use crate::{
	TOML
};

use anyhow::{
	Error, anyhow
};

use std::{
	fs::{
		create_dir_all, remove_file
	}, io::{
		stdin
	}, path::{
		Path,
		PathBuf
	}
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

pub async fn new(path: &str, version: &str, loader: &str, java: &str, dir: &str) -> Result<(), Error> {
	let mut path: String = path.to_string();
	let mut version: String = version.to_string();
	let mut loader: String = loader.to_string();
	let mut java: String = java.to_string();
	if java.len() > 0 && !(java.ends_with("java") || java.ends_with("java.exe")) {
		java = format!("{}{}", java, if cfg!(target_os = "windows") { "\\java" } else { "/java" })
	} else if java.len() == 0 {
		java = String::from("java")
	}
	if path == String::from("") {
		println!("请输入文件夹位置");
	}
	let p: &Path = Path::new(&path);
	if p.exists() {
		path = String::from("");
		println!("文件夹已存在：{}", path);
	}
	while path == String::from("") {
		stdin().read_line(&mut path).expect("");
		let p: &Path = Path::new(&path);
		if p.exists() {
			path = String::from("");
			println!("文件夹已存在：{}", path);
		}
	}
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
	let mut config: Config = Config::new(version.trim(), loader.trim(), dir);
	if let Some(toml_path) = Path::new(&path).join(TOML).to_str() {
		match loader.as_str() {
			"forge" => {
				let s: ProgressBar = spinner::new(100);
				let url: String = request::forge(&version).await?;
				s.inc(25);
				create_dir_all(&path).map_err(|_| anyhow!("路径错误: {}", path))?;
				request::download(url, Path::new(&path).join("forge_server_installer.jar")).await?;
				s.inc(25);
				command::new(
					&format!("{} -jar forge_server_installer.jar --installServer .", java), 
					false,
					&path
				)?;
				s.inc(50);
				if let Some(p) = Path::new(&path).join("eula.txt").to_str() {
					file::write_string(p, "eula=true")?;
				}
				s.inc(10);
				if let Some(p) = Path::new(&path).join("user_jvm_args.txt").to_str() {
					file::write_string(p, "-Xmx4G")?;
				}
				s.inc(10);
				if let Some(p) = Path::new(&path).join(if cfg!(target_os = "windows") { "run.bat" } else { "run.sh" }).to_str() {
					let script: String = file::read_script(p)?;
					config.set_script("start", &format!("{}{} nogui", java,script));
				}
				s.inc(10);
				s.finish_and_clear();
			}
			"neoforge" => {
				let s: ProgressBar = spinner::new(100);
				let url: String = request::neoforge(&version).await?;
				s.inc(25);
				create_dir_all(&path).map_err(|_| anyhow!("路径错误: {}", path))?;
				request::download(url, Path::new(&path).join("neoforge_server_installer.jar")).await?;
				s.inc(25);
				command::new(
					&format!("{} -jar neoforge_server_installer.jar --installServer .", java), 
					false,
					&path
				)?;
				s.inc(50);
				if let Some(p) = Path::new(&path).join("eula.txt").to_str() {
					file::write_string(p, "eula=true")?;
				}
				s.inc(10);
				if let Some(p) = Path::new(&path).join("user_jvm_args.txt").to_str() {
					file::write_string(p, "-Xmx4G")?;
				}
				s.inc(10);
				if let Some(p) = Path::new(&path).join(if cfg!(target_os = "windows") { "run.bat" } else { "run.sh" }).to_str() {
					let script: String = file::read_script(p)?;
					config.set_script("start", &format!("{}{} nogui", java,script));
				}
				s.inc(10);
				s.finish_and_clear();
			}
			"fabric" => {
				let s: ProgressBar = spinner::new(100);
				let url: String = request::fabric(&version).await?;
				s.inc(25);
				create_dir_all(&path).map_err(|_| anyhow!("路径错误: {}", path))?;
				request::download(url, Path::new(&path).join("fabric_server.jar")).await?;
				s.inc(25);
				command::new(
					&format!("{} -jar fabric_server.jar", java), 
					false,
					&path
				)?;
				s.inc(50);
				if let Some(p) = Path::new(&path).join("eula.txt").to_str() {
					file::write_string(p, "eula=true")?;
				}
				s.inc(15);
				config.set_script("start",&format!("{} -jar fabric_server.jar nogui", java));
				s.inc(15);
				s.finish_and_clear();
			}
			_ => return Err(anyhow!("加载器错误: {}", loader))
		}
		file::write(toml_path, config)
	} else {
		Err(anyhow!("路径错误: {}", path))
	}
}

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
	let config: Config = Config::new(version.trim(), loader.trim(), dir);
	file::write(TOML, config)
}

pub async fn install() -> Result<(), Error> {
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
			let task: JoinHandle<()> = spawn(async move {
				let _ = request::download(url, path).await;
			});
			tasks.push(task);
		}
	}).await;
	if tasks.len() > 0 {
		let s: ProgressBar = spinner::new(100);
		let step: u64 = ((1 / tasks.len()) * 100) as u64;
		for task in tasks {
			let _ = task.await;
			s.inc(step)
		}
		let result = clear(dir, names).await;
		s.finish_and_clear();
		return result;
	}
	Ok(())
}

pub async fn search(name: &str) -> Result<(), Error> {
	let config: Config = file::read(TOML)?;
	let mut offset: usize = 0;
	let mut input: String = String::from("");
	let mut list: Mods;
	loop {
		let s: ProgressBar = spinner::new_spinner();
		if let Ok(mods) = request::search_mods(name, config.loader(), config.version(), offset).await {
			s.finish_and_clear();
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
						return add(vec![m.id().to_string()]).await;
					}
				}
				break;
			}
		}
	}
	Ok(())
}

pub async fn add(ids: Vec<String>) -> Result<(), Error> {
	let s: ProgressBar = spinner::new_spinner();
	let config: Config = file::read(TOML)?;
	let version: String = config.version().to_string();
	let loader: String = config.loader().to_string();
	let tasks: Vec<JoinHandle<Result<(String, String), Error>>> = request::tasks(ids, version, loader, vec!["url", "id"]).await;
	if let Ok(mut config) = file::read(TOML) {
		for task in tasks {
			if let Ok(i) = task.await {
				if let Ok((name, url)) = i {
					config.push(&name, &url);
				}
			}
		}
		let _ = file::write(TOML, config);
	}
	s.finish_and_clear();
	Ok(())
}

pub async fn remove(ids: Vec<String>) -> Result<(), Error> {
	let s: ProgressBar = spinner::new_spinner();
	let config: Config = file::read(TOML)?;
	let version: String = config.version().to_string();
	let loader: String = config.loader().to_string();
	let tasks: Vec<JoinHandle<Result<(String, String), Error>>> = request::tasks(ids, version, loader, vec!["name", "url", "id"]).await;
	if let Ok(mut config) = file::read(TOML) {
		for task in tasks {
			if let Ok(i) = task.await {
				if let Ok((name, url)) = i {
					if url == String::from("") {
						config.remove_by_name(&name);
					} else {
						config.remove_by_name(&name);
						config.remove_by_url(&url);
					}
				}
			}
		}
		let _ = file::write(TOML, config);
	}
	s.finish_and_clear();
	Ok(())
}

pub async fn clear<T: AsRef<Path>>(dir: T, names: Vec<String>) -> Result<(), Error> {
	file::walk(dir, async |path, name, _| {
		if !names.contains(&name.to_string()) {
			if remove_file(path).is_err() {
				let _ = anyhow!("删除文件失败：{}", name);
			}
		}
	}).await;
	Ok(())
}

pub async fn run(k: &str) -> Result<(), Error> {
	let config: Config = file::read(TOML)?;
	if let Some(script) = config.script(k) {
		println!("{}", script);
		command::new(script, true, "./")?;
    	Ok(())
	} else {
		Err(anyhow!("读取命令失败: {}", k))
	}
}