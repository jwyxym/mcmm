use serde_derive::{
	Deserialize
};

use serde::{
	Serialize
};

use std::{
	collections::{
		BTreeMap
	},
	option::{
		Option
	}
};

#[derive(Deserialize, Clone, Serialize)]
pub struct Config {
	server: ServerInfo,
	mods : BTreeMap<String, String>,
	scripts : BTreeMap<String, String>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct ServerInfo {
	dir: String,
	version: String,
	loader: String,
}

impl Config {
	pub fn new(version: &str, loader: &str, dir: &str) -> Config {
		let info: ServerInfo = ServerInfo{
			dir: dir.to_string(),
			version: version.to_string(),
			loader: loader.to_string()
		};
		Config {
			server: info,
			mods : BTreeMap::new(),
			scripts : BTreeMap::new(),
		}
	}

	pub fn dir(&self) -> &str {
		&self.server.dir
	}

	pub fn version(&self) -> &str {
		&self.server.version
	}

	pub fn loader(&self) -> &str {
		&self.server.loader
	}

	pub async fn get_content(&self, mut callback: impl AsyncFnMut(&str, &str) -> ()) -> () {
		for (key, value) in &self.mods {
			callback(key, value).await;
		}
	}

	pub fn push(&mut self, k: &str, v: &str) -> () {
		self.mods.insert(k.to_string(), v.to_string());
	}

	pub fn script(&self, k: &str) -> Option<&str> {
		self.scripts.get(k).and_then(|s| Some(s.as_str()))
	}
}