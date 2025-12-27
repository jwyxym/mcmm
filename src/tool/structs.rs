use serde_derive::{
	Deserialize
};

use tabled::{
	Table,
	Tabled
};

#[derive(Deserialize, Clone)]
pub struct Mods {
	hits: Vec<Mod>,
	offset: usize,
	limit: usize
}

#[derive(Deserialize, Clone)]
pub struct Mod {
	project_id: String,
	title: String
}

#[derive(Deserialize)]
pub struct ModInfos {
	content: Vec<ModInfo>
}

#[derive(Deserialize, Clone)]
pub struct ModInfo {
	loaders: Vec<String>,
	game_versions: Vec<String>,
	files: Vec<Info>
}

#[derive(Deserialize, Clone)]
pub struct Info {
	filename: String,
	url: String
}

#[derive(Deserialize, Clone)]
pub struct NeoForge {
	versions: Vec<String>
}

#[derive(Deserialize, Clone)]
pub struct Fabric {
	version: String
}

#[derive(Tabled)]
pub struct ModsGrid {
	key: usize,
	title: String,
	id: String
}

impl Mods {
	pub fn list(&self) -> &Vec<Mod> {
		&self.hits
	}

	pub fn index(&self, i: usize) -> Option<&Mod> {
		self.hits.get(i)
	}

	pub fn offset(&self) -> usize {
		self.offset
	}

	pub fn limit(&self) -> usize {
		self.limit
	}

	pub fn log(&self) -> () {
		let data: Vec<ModsGrid> = self.list().iter()
			.enumerate()
			.map(|(ct, i)| ModsGrid {
				key: ct as usize,
				title: i.title().to_string(),
				id: i.id().to_string()
			})
			.collect();
		let table: String = Table::new(data).to_string();
		println!("{}", table);
	}
}

impl Mod {
	pub fn title(&self) -> &str {
		&self.title
	}

	pub fn id(&self) -> &str {
		&self.project_id
	}
}

impl ModInfos {
	pub fn new(content: Vec<ModInfo>) -> ModInfos {
		ModInfos {
			content: content
		}
	}

	pub fn chk(&self, version: &str, loader: &str) -> ModInfos {
		let content: Vec<ModInfo> = self.content.iter()
			.filter(|x| x.chk_version(version)
				&& x.chk_loader(loader)
			)
			.cloned().collect();
		ModInfos::new(content)
	}

	pub fn file(&self) -> Option<&Info> {
		if let Some(m) = self.content.get(0){
			m.file()
		} else {
			None
		}
		
	}
}

impl ModInfo {
	pub fn chk_version(&self, i: &str) -> bool {
		self.game_versions.contains(&i.to_string())
	}

	pub fn chk_loader(&self, i: &str) -> bool {
		self.loaders.contains(&i.to_string())
	}

	pub fn file(&self) -> Option<&Info> {
		self.files.get(0)
	}
}

impl Info {
	pub fn name(&self) -> &str {
		&self.filename
	}

	pub fn url(&self) -> &str {
		&self.url
	}
}

impl NeoForge {
	pub fn versions(&self) -> Vec<String> {
		self.versions.clone()
	}
}

impl Fabric {
	pub fn version(&self) -> &str {
		&self.version
	}
}