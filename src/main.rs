mod tool;
mod log;
use log::{
	MCMM
};

use anyhow::{
	Error,
	anyhow
};

use std::{
	env
};

pub const TOML: &str = "mods.toml";

#[tokio::main]
async fn main() -> Result<(), Error> {
	let mut args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		args.push(String::from(""));
	}
	match args[1].as_str() {
		"new" | "n" => tool::new(
			if args.len() > 2 { &args[2] } else { "" },
			if args.len() > 3 { &args[3] } else { "" },
			if args.len() > 4 { &args[4] } else { "" },
			if args.len() > 5 { &args[5] } else { "" },
			if args.len() > 6 { &args[6] } else { "mods" },
		).await,
		"init" => tool::init(
			if args.len() > 2 { &args[2] } else { "" },
			if args.len() > 3 { &args[3] } else { "" },
			if args.len() > 4 { &args[4] } else { "mods" }
		).await,
		"install" | "i" => tool::install().await,
		"add" | "a" => if args.len() > 2 {
			tool::add(args.split_off(2)).await
		} else { Err(anyhow!("请输入添加内容")) },
		"remove" | "r" => if args.len() > 2 {
			tool::remove(args.split_off(2)).await
		} else { Err(anyhow!("请输入删除内容")) },
		"search" | "s" => if args.len() > 2 {
			tool::search(&args[2]).await
		} else { Err(anyhow!("请输入搜索内容")) },
		"clear" | "c" => tool::clear(
			if args.len() > 2 { &args[2] } else { "mods" },
			Vec::new()
		).await,
		"run" =>  if args.len() > 2 {
			tool::run(&args[2]).await
		} else { Err(anyhow!("请输入命令")) },
		_ => Ok(MCMM::log())
	}
}
