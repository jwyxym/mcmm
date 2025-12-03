use anyhow::{
	Error,
	anyhow
};

use reqwest::{
	Client,
	Response
};

use tokio::{
	fs::{
		File
	},
	io::{
		AsyncWriteExt
	}
};

use std::{
	path::{
		PathBuf
	},
	fs::{
		OpenOptions
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
	encode
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