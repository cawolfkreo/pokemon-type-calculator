use std::default::Default;
use std::fmt::{self, Display};
use std::str::FromStr;

use pokemon_type_effectiveness::PokemonTypes;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum TypeGui {
	Normal,
	Fighting,
	Flying,
	Poison,
	Ground,
	Rock,
	Bug,
	Ghost,
	Steel,
	Fire,
	Water,
	Grass,
	Electric,
	Psychic,
	Ice,
	Dragon,
	Dark,
	Fairy,
	#[default]
	None
}

impl TypeGui {
	pub fn list_all_types() -> Vec<&'static str> {
		vec![
			"None",		"Bug",		"Dark",
			"Dragon", 	"Electric", "Fairy",
			"Fighting",	"Fire",		"Flying", 
			"Ghost",	"Grass",	"Ground",
			"ice",		"Normal",	"Poison",
			"Psychic",  "Rock",		"Steel",
			"Water"
		]
	}
}

impl Display for TypeGui {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl FromStr for TypeGui {
	type Err = ();
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"Normal" => Ok(TypeGui::Normal),
			"Fighting" => Ok(TypeGui::Fighting),
			"Flying" => Ok(TypeGui::Flying),
			"Poison" => Ok(TypeGui::Poison),
			"Ground" => Ok(TypeGui::Ground),
			"Rock" => Ok(TypeGui::Rock),
			"Bug" => Ok(TypeGui::Bug),
			"Ghost" => Ok(TypeGui::Ghost),
			"Steel" => Ok(TypeGui::Steel),
			"Fire" => Ok(TypeGui::Fire),
			"Water" => Ok(TypeGui::Water),
			"Grass" => Ok(TypeGui::Grass),
			"Electric" => Ok(TypeGui::Electric),
			"Psychic" => Ok(TypeGui::Psychic),
			"Ice" => Ok(TypeGui::Ice),
			"Dragon" => Ok(TypeGui::Dragon),
			"Dark" => Ok(TypeGui::Dark),
			"Fairy" => Ok(TypeGui::Fairy),
			"None" => Ok(TypeGui::None),
			_ => Err(())
		}
	}
}

impl From<TypeGui> for Option<PokemonTypes>{ 
	fn from(val: TypeGui) -> Self {
		match val {
			TypeGui::Normal     => Some(PokemonTypes::Normal),
			TypeGui::Fighting   => Some(PokemonTypes::Fighting),
			TypeGui::Flying     => Some(PokemonTypes::Flying),
			TypeGui::Poison     => Some(PokemonTypes::Poison),
			TypeGui::Ground     => Some(PokemonTypes::Ground),
			TypeGui::Rock       => Some(PokemonTypes::Rock),
			TypeGui::Bug        => Some(PokemonTypes::Bug),
			TypeGui::Ghost      => Some(PokemonTypes::Ghost),
			TypeGui::Steel      => Some(PokemonTypes::Steel),
			TypeGui::Fire       => Some(PokemonTypes::Fire),
			TypeGui::Water      => Some(PokemonTypes::Water),
			TypeGui::Grass      => Some(PokemonTypes::Grass),
			TypeGui::Electric   => Some(PokemonTypes::Electric),
			TypeGui::Psychic    => Some(PokemonTypes::Psychic),
			TypeGui::Ice        => Some(PokemonTypes::Ice),
			TypeGui::Dragon     => Some(PokemonTypes::Dragon),
			TypeGui::Dark       => Some(PokemonTypes::Dark),
			TypeGui::Fairy      => Some(PokemonTypes::Fairy),
			TypeGui::None       => None
		}
	}
}
