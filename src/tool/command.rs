use std::{
	io::{
		BufRead, BufReader
	}, process::{
		Child, Command, Stdio
	}
};
use anyhow::{
	Error, anyhow
};

pub fn new(script: &str, print: bool, dir: &str) -> Result<(), Error> {
	let mut child: Child = Command::new(
		if cfg!(target_os = "windows") { "cmd" } else { "sh" }
	)
		.args([if cfg!(target_os = "windows") { "/C" } else { "-c" }, script])
		.current_dir(dir)
		.stdout(Stdio::piped())
		.spawn()
		.map_err(|_| anyhow!("命令执行失败: {}", script))?;
	if let Some(stdout) = child.stdout.take() {
		let reader = BufReader::new(stdout);
		reader.lines().into_iter().for_each(|l| {
			if print && let Ok(line) = l {
				println!("{}", line);
			}
		});
	}
	Ok(())
}