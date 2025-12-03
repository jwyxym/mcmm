mod tool;

use anyhow::{
	Error,
	anyhow
};

use std::{
	env
};

use tabled::{
	Table,
	Tabled
};

pub const TOML: &str = "mods.toml";

#[derive(Tabled)]
struct MCMM {
	options: &'static str,
	functions: &'static str,
	parameters: &'static str,
}

impl MCMM {
	pub fn new() -> Vec<MCMM> {
		let mut vec: Vec<MCMM> = Vec::new();
		vec.push(MCMM{
			options: "init",
			functions: "初始化",
			parameters: ""
		});
		vec.push(MCMM{
			options: "install, i",
			functions: "下载",
			parameters: ""
		});
		vec.push(MCMM{
			options: "add, a",
			functions: "添加",
			parameters: "id(来自modrinth.com)"
		});
		vec.push(MCMM{
			options: "search, s",
			functions: "搜索",
			parameters: "关键词"
		});
		vec.push(MCMM{
			options: "clear, c",
			functions: "清空",
			parameters: ""
		});
		vec
	}
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	let mut args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		args.push(String::from(""));
	}
	match args[1].as_str() {
		"init" => tool::init(
			if args.len() > 2 { &args[2] } else { "" },
			if args.len() > 3 { &args[3] } else { "" },
			if args.len() > 4 { &args[4] } else { "mods" }
		).await,
		"install" | "i" => tool::install().await,
		"add" | "a" => if args.len() > 2 {
			tool::add(&args[2]).await
		} else { Err(anyhow!("请输入添加内容")) },
		"search" | "s" => if args.len() > 2 {
			tool::search(&args[2]).await
		} else { Err(anyhow!("请输入搜索内容")) },
		"clear" | "c" => tool::clear(
			if args.len() > 2 { &args[2] } else { "mods" },
			Vec::new()
		).await,
		_ => {
			println!("{}", Table::new(MCMM::new()).to_string());
			Ok(())
		}
	}
}
