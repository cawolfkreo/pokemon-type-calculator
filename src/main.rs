#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod gui;

use std::process;

fn main() {
	env_logger::init();

	if let Err(error) = gui::run() {
		eprintln!("found and error while running the poke calculator:");
		eprintln!("{}", error);
		process::exit(1);
	}
}
