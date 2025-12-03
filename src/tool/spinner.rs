use indicatif::{
	ProgressBar,
	ProgressStyle
};

use std::{
	time::{
		Duration
	}
};

pub fn new() -> ProgressBar {
	let spinner: ProgressBar = ProgressBar::new_spinner();
	spinner.set_style(
		ProgressStyle::default_spinner()
			.template("{spinner:.green} {msg}")
			.unwrap()
	);
	spinner.enable_steady_tick(Duration::from_millis(100));
	spinner
}