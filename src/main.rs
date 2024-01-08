mod gui;

use std::process;

use pokemon_type_effectiveness::example;

fn main() {
	env_logger::init();
	example();

	if let Err(error) = gui::run() {
		eprintln!("found and error while running the poke calculator:");
		eprintln!("{}", error);
		process::exit(1);
	}
}
