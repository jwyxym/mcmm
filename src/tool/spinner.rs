use indicatif::{
	ProgressBar,
	ProgressStyle
};

use std::{
	time::{
		Duration
	}
};

pub fn new_spinner() -> ProgressBar {
	let spinner: ProgressBar = ProgressBar::new_spinner();
	spinner.set_style(
		ProgressStyle::default_spinner()
			.template("{spinner:.white} {msg}")
			.unwrap()
	);
	spinner.enable_steady_tick(Duration::from_millis(100));
	spinner
}

pub fn new(len: usize) -> ProgressBar {
	let spinner: ProgressBar = ProgressBar::new(len as u64);
	spinner.set_style(
		ProgressStyle::default_bar()
			.template("{spinner} [{elapsed_precise}] [{bar:40.white/white}] {pos}/{len}")
			.unwrap()
	);
	spinner.enable_steady_tick(Duration::from_millis(100));
	spinner
}