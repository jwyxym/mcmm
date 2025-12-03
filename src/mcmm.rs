use tabled::{
	Tabled,
	Table
};

#[derive(Tabled)]
pub struct MCMM {
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

	pub fn log() -> () {
		println!("{}", Table::new(MCMM::new()).to_string());
	}
}