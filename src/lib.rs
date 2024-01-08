mod pokemon;

pub use pokemon::{Fight, PokemonTypes};

const SEPARATOR: &str = "====================";

pub fn example() {
	fight_one_defense_type(PokemonTypes::Normal, PokemonTypes::Fighting);

	println!("{SEPARATOR}");

	fight_two_defense_type(PokemonTypes::Flying, PokemonTypes::Fighting, PokemonTypes::Normal);

	println!("{SEPARATOR}");

	fight_two_defense_type(PokemonTypes::Fighting, PokemonTypes::Flying, PokemonTypes::Normal);

	println!("{SEPARATOR}");

	fight_one_defense_type(PokemonTypes::Fighting, PokemonTypes::Flying);
	
	println!("{SEPARATOR}");

	fight_one_defense_type(PokemonTypes::Fighting, PokemonTypes::Normal);
}

fn fight_one_defense_type(attack_type: PokemonTypes, defense_type: PokemonTypes) {
	let defense = pokemon::DefenseType::OneType(defense_type);
	let fight = Fight::new(attack_type, defense);

	let effect = fight.calculate_effectiveness();

	println!("{} {}", fight, effect);
}

fn fight_two_defense_type(attack_type: PokemonTypes, defense_type1: PokemonTypes, defense_type2: PokemonTypes) {
	let defense = pokemon::DefenseType::TwoTypes(defense_type1, defense_type2);
	let fight = Fight::new(attack_type, defense);

	let effect = fight.calculate_effectiveness();

	println!("{} {}", fight, effect);
}