mod pokemon;

use pokemon::{Fight, p_type};

pub fn example() {
	fight_one_defense_type(p_type::PokemonTypes::Normal, p_type::PokemonTypes::Fighting);

	fight_two_defense_type(p_type::PokemonTypes::Flying, p_type::PokemonTypes::Fighting, p_type::PokemonTypes::Normal);

	fight_two_defense_type(p_type::PokemonTypes::Fighting, p_type::PokemonTypes::Flying, p_type::PokemonTypes::Normal);

	fight_one_defense_type(p_type::PokemonTypes::Fighting, p_type::PokemonTypes::Flying);
	
	fight_one_defense_type(p_type::PokemonTypes::Fighting, p_type::PokemonTypes::Normal);
}

fn fight_one_defense_type(attack_type: p_type::PokemonTypes, defense_type: p_type::PokemonTypes) {
	let defense = pokemon::DefenseType::OneType(defense_type);
	let fight = Fight::new(attack_type, defense);

	let effect = fight.calculate_effectiveness();

	println!("{} {}", fight, effect);
}

fn fight_two_defense_type(attack_type: p_type::PokemonTypes, defense_type1: p_type::PokemonTypes, defense_type2: p_type::PokemonTypes) {
	let defense = pokemon::DefenseType::TwoTypes(defense_type1, defense_type2);
	let fight = Fight::new(attack_type, defense);

	let effect = fight.calculate_effectiveness();

	println!("{} {}", fight, effect);
}