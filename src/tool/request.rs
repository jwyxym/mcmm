use crate::{
	TOML,
	tool::{
		file
	}
};

use anyhow::{
	Error,
	anyhow
};

use reqwest::{
	Client,
	Response
};

use tokio::{
	spawn,
	fs::{
		File
	},
	io::{
		AsyncWriteExt
	},
	task::{
		JoinHandle
	}
};

use std::{
	fs::{
		OpenOptions
	},
	path::{
		Path,
		PathBuf
	}
};

use futures_util::{
	stream::{
		StreamExt
	}
};

use fs2::{
	FileExt
};

use urlencoding::{
	encode,
	decode
};

use url::{
	Url
};

use crate::tool::{
	structs::{
		Mods,
		ModInfo,
		ModInfos
	}
};

fn error(path: &PathBuf) -> Error {
	if let Some(path) = path.to_str() {
		anyhow!("文件下载失败: {}", path)
	} else {
		anyhow!("文件下载失败")
	}
}

pub async fn download(url: String, path: PathBuf) -> Result<(), Error> {
	let response: Response = Client::new().get(&url).send().await?;
	
	if !response.status().is_success() {
		return Err(anyhow!("下载错误：{}\n错误码:{}", url, response.status()));
	}
	
	let file = OpenOptions::new()
		.create(true)
		.write(true)
		.truncate(true)
		.open(&path)
		.map_err(|_| error(&path))?;
	file.lock_exclusive()?;
	let mut file: File = File::from(file);
	let mut stream = response.bytes_stream();
	while let Some(chunk) = stream.next().await {
		if let Ok(chunk) = chunk {
			if file.write_all(&chunk).await.is_err() {
				return Err(error(&path));
			}
		}
	}

	Ok(())
}

pub async fn search_mods(name: &str, loader: &str, version: &str, offset: usize) -> Result<Mods, Error> {
	let version: String = format!("[[\"project_type:mod\"],[\"versions:{}\"],[\"categories:{}\"]]", version, loader);
	let response: Response = Client::new().get(format!("https://api.modrinth.com/v2/search?query={}&offset={}&facets={}&limit=20", encode(name), offset, encode(&version))).send().await?;
	if !response.status().is_success() {
		return Err(anyhow!("搜索失败: {}", response.status()));
	}
	Ok(response.json::<Mods>().await
		.map_err(|e| anyhow!("解析 JSON 失败: {}", e))?)
}

pub async fn search_mod(id: &str) -> Result<ModInfos, Error> {
	let response: Response = Client::new().get(format!("https://api.modrinth.com/v2/project/{}/version", id)).send().await?;
	if !response.status().is_success() {
		return Err(anyhow!("搜索失败: {}", response.status()));
	}
	let content: Vec<ModInfo> = response.json::<Vec<ModInfo>>().await
		.map_err(|e| anyhow!("解析 JSON 失败: {}", e))?;
	Ok(ModInfos::new(content))
}

pub async fn tasks(ids: Vec<String>, version: String, loader: String, funcs: Vec<&str>) -> Vec<JoinHandle<Result<(String, String), Error>>> {
	let mut tasks: Vec<JoinHandle<Result<(String, String), Error>>> = Vec::new();
	for id in ids.into_iter() {
		let version: String = version.clone();
		let loader: String = loader.clone();
		let chk_url: bool = funcs.contains(&"url");
		let chk_id: bool = funcs.contains(&"url");
		let chk_name: bool = funcs.contains(&"url");
		let task = spawn(async move {
			if chk_url && let Ok(url) = Url::parse(&id) {
				if let Some(name) = url.path_segments()
					.and_then(|i| i.last())
					.and_then(|i| Some(decode(i))) {
						if let Ok(name) = name {
							let name: String = name.to_string();
							let mut name: PathBuf = Path::new(&name).to_path_buf();
							let query: Vec<&str> = id.split('?').collect();
							if let Some(query) = query.last() && query.len() > 1 {
								name = name.with_extension(format!("{}.jar", query));
							} else if name.extension().is_none() {
								name = name.with_extension("jar");
							}
							if let Some(name) = name.to_str() {
								return Ok((name.to_string(), url.to_string()));
							}
						}
					}
			} else if chk_id && let Ok(m) = search_mod(&id).await {
				if let Some(m) = m.chk(&version, &loader).file() {
					return Ok((m.name().to_string(), m.url().to_string()));
				}
			} else if chk_name && let Ok(config) = file::read(TOML) {
				if config.include(&id) {
					return Ok((id, "".to_string()));
				}
			}
			let err = anyhow!("删除mod失败: {}", id);
			println!("{}", err);
			Err(err)
		});
		tasks.push(task);
	}
	tasks
}