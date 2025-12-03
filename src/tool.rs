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
		stdin,
		BufRead,
		BufReader
	},
	process::{
		Command,
		Stdio,
		Child
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
	let s: ProgressBar = spinner::new();
	let config: Config = Config::new(version.trim(), loader.trim(), dir);
	let result = file::write(TOML, config);
	s.finish();
	result
}

pub async fn install() -> Result<(), Error> {
	let s: ProgressBar = spinner::new();
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
	let result = clear(dir, names).await;
	s.finish();
	result
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
	let s: ProgressBar = spinner::new();
	let config: Config = file::read(TOML)?;
	let version: String = config.version().to_string();
	let loader: String = config.loader().to_string();
	let mut tasks = Vec::new();
	for id in ids.into_iter() {
		let version: String = version.clone();
		let loader: String = loader.clone();
		let task = spawn(async move {
			if let Ok(m) = request::search_mod(&id).await {
				if let Some(m) = m.chk(&version, &loader).file() {
					return Ok((m.name().to_string(), m.url().to_string()));
				}
			}
			let err = anyhow!("添加mod失败: {}", id);
			println!("{}", err);
			Err(err)
		});
		tasks.push(task);
	}
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
	s.finish();
	Ok(())
}

pub async fn clear<T: AsRef<Path>>(dir: T, names: Vec<String>) -> Result<(), Error> {
	let s: ProgressBar = spinner::new();
	file::walk(dir, async |path, name, _| {
		if !names.contains(&name.to_string()) {
			if remove_file(path).is_err() {
				let _ = anyhow!("删除文件失败：{}", name);
			}
		}
	}).await;
	s.finish();
	Ok(())
}

pub async fn run(k: &str) -> Result<(), Error> {
	let config: Config = file::read(TOML)?;
	if let Some(script) = config.script(k) {
		let mut child: Child = Command::new(
				if cfg!(target_os = "windows") { "cmd" } else { "sh" }
			)
			.args([if cfg!(target_os = "windows") { "/C" } else { "-c" }, script])
			.stdout(Stdio::piped())
			.spawn()
			.map_err(|_| anyhow!("命令执行失败: {}", script))?;
		if let Some(stdout) = child.stdout.take() {
			let reader = BufReader::new(stdout);
			reader.lines().into_iter().for_each(|l| {
				if let Ok(line) = l {
					println!("{}", line);
				}
			});
		}
    	Ok(())
	} else {
		Err(anyhow!("读取命令失败: {}", k))
	}
}