use std::fmt::{self, Display};

use super::Multiplier;

#[derive(Debug)]
pub enum PokemonTypes {
	Normal,
	Fighting,
	Flying,
}

impl PokemonTypes {
	pub fn calculate_damage_multiplier(&self, attack_type: &Self) -> Multiplier {
		match (attack_type, self) {
			(PokemonTypes::Fighting, PokemonTypes::Normal) => Multiplier(2.0),
			(PokemonTypes::Fighting, PokemonTypes::Flying) => Multiplier(0.5),
			(PokemonTypes::Flying, PokemonTypes::Fighting) => Multiplier(2.0),
			(_, _) => Multiplier(1.0),
		}
	}
}


impl Display for PokemonTypes {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}