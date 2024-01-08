use std::fmt::{self, Display};

use super::Multiplier;

#[derive(Debug)]
pub enum PokemonTypes {
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
	Fairy
}

impl PokemonTypes {
	pub fn calculate_damage_multiplier(&self, attack_type: &Self) -> Multiplier {
		// Taken from: https://bulbapedia.bulbagarden.net/wiki/Type/Type_chart
		match (attack_type, self) {
			// Normal defends
			(PokemonTypes::Fighting,	PokemonTypes::Normal)	=> Multiplier(2.0),
			(PokemonTypes::Ghost,		PokemonTypes::Normal)	=> Multiplier(0.0),
			// Fighting defends 
			(PokemonTypes::Flying,		PokemonTypes::Fighting) => Multiplier(2.0),
			(PokemonTypes::Rock,		PokemonTypes::Fighting) => Multiplier(0.5),
			(PokemonTypes::Bug,			PokemonTypes::Fighting) => Multiplier(0.5),
			(PokemonTypes::Psychic,		PokemonTypes::Fighting) => Multiplier(2.0),
			(PokemonTypes::Dark,		PokemonTypes::Fighting) => Multiplier(0.5),
			(PokemonTypes::Fairy,		PokemonTypes::Fighting)	=> Multiplier(2.0),
			// Flying defends
			(PokemonTypes::Fighting,	PokemonTypes::Flying)	=> Multiplier(0.5),
			(PokemonTypes::Ground,		PokemonTypes::Flying)	=> Multiplier(0.0),
			(PokemonTypes::Rock,		PokemonTypes::Flying)	=> Multiplier(2.0),
			(PokemonTypes::Bug,			PokemonTypes::Flying)	=> Multiplier(0.5),
			(PokemonTypes::Grass,		PokemonTypes::Flying)	=> Multiplier(0.5),
			(PokemonTypes::Electric,	PokemonTypes::Flying)	=> Multiplier(2.0),
			(PokemonTypes::Ice,			PokemonTypes::Flying)	=> Multiplier(2.0),
			// Poison defends
			(PokemonTypes::Fighting, 	PokemonTypes::Poison)	=> Multiplier(0.5),
			(PokemonTypes::Poison, 		PokemonTypes::Poison)	=> Multiplier(0.5),
			(PokemonTypes::Ground,	 	PokemonTypes::Poison)	=> Multiplier(2.0),
			(PokemonTypes::Bug,			PokemonTypes::Poison)	=> Multiplier(0.5),
			(PokemonTypes::Grass,		PokemonTypes::Poison)	=> Multiplier(0.5),
			(PokemonTypes::Psychic,		PokemonTypes::Poison)	=> Multiplier(2.0),
			(PokemonTypes::Fairy,		PokemonTypes::Poison)	=> Multiplier(0.5),
			// Ground defends
			(PokemonTypes::Poison,		PokemonTypes::Ground)	=> Multiplier(0.5),
			(PokemonTypes::Rock,		PokemonTypes::Ground)	=> Multiplier(0.5),
			(PokemonTypes::Water,		PokemonTypes::Ground)	=> Multiplier(2.0),
			(PokemonTypes::Grass,		PokemonTypes::Ground)	=> Multiplier(2.0),
			(PokemonTypes::Electric,	PokemonTypes::Ground)	=> Multiplier(0.0),
			(PokemonTypes::Ice,			PokemonTypes::Ground)	=> Multiplier(2.0),
			// Rock defends
			(PokemonTypes::Normal,		PokemonTypes::Rock)		=> Multiplier(0.5),
			(PokemonTypes::Fighting,	PokemonTypes::Rock)		=> Multiplier(2.0),
			(PokemonTypes::Flying,		PokemonTypes::Rock)		=> Multiplier(0.5),
			(PokemonTypes::Poison,		PokemonTypes::Rock)		=> Multiplier(0.5),
			(PokemonTypes::Ground,		PokemonTypes::Rock)		=> Multiplier(2.0),
			(PokemonTypes::Steel,		PokemonTypes::Rock)		=> Multiplier(2.0),
			(PokemonTypes::Fire,		PokemonTypes::Rock)		=> Multiplier(0.5),
			(PokemonTypes::Water,		PokemonTypes::Rock)		=> Multiplier(2.0),
			(PokemonTypes::Grass,		PokemonTypes::Rock)		=> Multiplier(2.0),
			// Bug defends
			(PokemonTypes::Fighting,	PokemonTypes::Bug)		=> Multiplier(0.5),
			(PokemonTypes::Flying,		PokemonTypes::Bug)		=> Multiplier(2.0),
			(PokemonTypes::Ground,		PokemonTypes::Bug)		=> Multiplier(0.5),
			(PokemonTypes::Rock,		PokemonTypes::Bug)		=> Multiplier(2.0),
			(PokemonTypes::Fire,		PokemonTypes::Bug)		=> Multiplier(2.0),
			(PokemonTypes::Grass,		PokemonTypes::Bug)		=> Multiplier(0.5),
			// Ghost defends
			(PokemonTypes::Normal,		PokemonTypes::Ghost)	=> Multiplier(0.0),
			(PokemonTypes::Fighting,	PokemonTypes::Ghost)	=> Multiplier(0.0),
			(PokemonTypes::Poison,		PokemonTypes::Ghost)	=> Multiplier(0.5),
			(PokemonTypes::Bug,			PokemonTypes::Ghost)	=> Multiplier(0.5),
			(PokemonTypes::Ghost,		PokemonTypes::Ghost)	=> Multiplier(2.0),
			(PokemonTypes::Dark,		PokemonTypes::Ghost)	=> Multiplier(2.0),
			// Steel defends
			(PokemonTypes::Normal,		PokemonTypes::Steel)	=> Multiplier(0.5),
			(PokemonTypes::Fighting,	PokemonTypes::Steel)	=> Multiplier(2.0),
			(PokemonTypes::Flying,		PokemonTypes::Steel)	=> Multiplier(0.5),
			(PokemonTypes::Poison,		PokemonTypes::Steel)	=> Multiplier(0.0),
			(PokemonTypes::Ground,		PokemonTypes::Steel)	=> Multiplier(2.0),
			(PokemonTypes::Rock,		PokemonTypes::Steel)	=> Multiplier(0.5),
			(PokemonTypes::Bug,			PokemonTypes::Steel)	=> Multiplier(0.5),
			(PokemonTypes::Steel,		PokemonTypes::Steel)	=> Multiplier(0.5),
			(PokemonTypes::Fire,		PokemonTypes::Steel)	=> Multiplier(2.0),
			(PokemonTypes::Grass,		PokemonTypes::Steel)	=> Multiplier(0.5),
			(PokemonTypes::Psychic,		PokemonTypes::Steel)	=> Multiplier(0.5),
			(PokemonTypes::Ice,			PokemonTypes::Steel)	=> Multiplier(0.5),
			(PokemonTypes::Dragon,		PokemonTypes::Steel)	=> Multiplier(0.5),
			(PokemonTypes::Fairy,		PokemonTypes::Steel)	=> Multiplier(0.5),
			// Fire defends
			(PokemonTypes::Ground,		PokemonTypes::Fire)		=> Multiplier(2.0),
			(PokemonTypes::Rock,		PokemonTypes::Fire)		=> Multiplier(2.0),
			(PokemonTypes::Bug,			PokemonTypes::Fire)		=> Multiplier(0.5),
			(PokemonTypes::Steel,		PokemonTypes::Fire)		=> Multiplier(0.5),
			(PokemonTypes::Fire,		PokemonTypes::Fire)		=> Multiplier(0.5),
			(PokemonTypes::Water,		PokemonTypes::Fire)		=> Multiplier(2.0),
			(PokemonTypes::Grass,		PokemonTypes::Fire)		=> Multiplier(0.5),
			(PokemonTypes::Ice,			PokemonTypes::Fire)		=> Multiplier(0.5),
			(PokemonTypes::Fairy,		PokemonTypes::Fire)		=> Multiplier(0.5),
			// Water defend
			(PokemonTypes::Steel,		PokemonTypes::Water)	=> Multiplier(0.5),
			(PokemonTypes::Fire,		PokemonTypes::Water)	=> Multiplier(0.5),
			(PokemonTypes::Water,		PokemonTypes::Water)	=> Multiplier(0.5),
			(PokemonTypes::Grass,		PokemonTypes::Water)	=> Multiplier(2.0),
			(PokemonTypes::Electric,	PokemonTypes::Water)	=> Multiplier(2.0),
			(PokemonTypes::Ice,			PokemonTypes::Water)	=> Multiplier(0.5),
			// Grass defend
			(PokemonTypes::Flying,		PokemonTypes::Grass)	=> Multiplier(2.0),
			(PokemonTypes::Poison,		PokemonTypes::Grass)	=> Multiplier(2.0),
			(PokemonTypes::Ground,		PokemonTypes::Grass)	=> Multiplier(0.5),
			(PokemonTypes::Bug,			PokemonTypes::Grass)	=> Multiplier(2.0),
			(PokemonTypes::Fire,		PokemonTypes::Grass)	=> Multiplier(2.0),
			(PokemonTypes::Water,		PokemonTypes::Grass)	=> Multiplier(0.5),
			(PokemonTypes::Grass,		PokemonTypes::Grass)	=> Multiplier(0.5),
			(PokemonTypes::Electric,	PokemonTypes::Grass)	=> Multiplier(0.5),
			(PokemonTypes::Ice,			PokemonTypes::Grass)	=> Multiplier(2.0),
			// Electric defend
			(PokemonTypes::Flying,		PokemonTypes::Electric)	=> Multiplier(0.5),
			(PokemonTypes::Ground,		PokemonTypes::Electric)	=> Multiplier(2.0),
			(PokemonTypes::Steel,		PokemonTypes::Electric)	=> Multiplier(0.5),
			(PokemonTypes::Electric,	PokemonTypes::Electric)	=> Multiplier(0.0),
			// Psychic defend
			(PokemonTypes::Fighting,	PokemonTypes::Psychic)	=> Multiplier(0.5),
			(PokemonTypes::Bug,			PokemonTypes::Psychic)	=> Multiplier(2.0),
			(PokemonTypes::Ghost,		PokemonTypes::Psychic)	=> Multiplier(2.0),
			(PokemonTypes::Psychic,		PokemonTypes::Psychic)	=> Multiplier(0.5),
			(PokemonTypes::Dark,		PokemonTypes::Psychic)	=> Multiplier(2.0),
			// Ice defend
			(PokemonTypes::Fighting,	PokemonTypes::Ice)		=> Multiplier(2.0),
			(PokemonTypes::Rock,		PokemonTypes::Ice)		=> Multiplier(2.0),
			(PokemonTypes::Steel,		PokemonTypes::Ice)		=> Multiplier(2.0),
			(PokemonTypes::Fire,		PokemonTypes::Ice)		=> Multiplier(2.0),
			(PokemonTypes::Ice,			PokemonTypes::Ice)		=> Multiplier(0.5),
			// Dragon defend
			(PokemonTypes::Fire,		PokemonTypes::Dragon)	=> Multiplier(0.5),
			(PokemonTypes::Water,		PokemonTypes::Dragon)	=> Multiplier(0.5),
			(PokemonTypes::Grass,		PokemonTypes::Dragon)	=> Multiplier(0.5),
			(PokemonTypes::Electric,	PokemonTypes::Dragon)	=> Multiplier(0.5),
			(PokemonTypes::Ice,			PokemonTypes::Dragon)	=> Multiplier(2.0),
			(PokemonTypes::Dragon,		PokemonTypes::Dragon)	=> Multiplier(2.0),
			(PokemonTypes::Fairy,		PokemonTypes::Dragon)	=> Multiplier(2.0),
			// Dark defend
			(PokemonTypes::Fighting,	PokemonTypes::Dark)		=> Multiplier(2.0),
			(PokemonTypes::Bug,			PokemonTypes::Dark)		=> Multiplier(2.0),
			(PokemonTypes::Ghost,		PokemonTypes::Dark)		=> Multiplier(0.5),
			(PokemonTypes::Psychic,		PokemonTypes::Dark)		=> Multiplier(0.0),
			(PokemonTypes::Dark,		PokemonTypes::Dark)		=> Multiplier(0.5),
			(PokemonTypes::Fairy,		PokemonTypes::Dark)		=> Multiplier(2.0),
			// Fairy defend
			(PokemonTypes::Fighting,	PokemonTypes::Fairy)	=> Multiplier(0.0),
			(PokemonTypes::Poison,		PokemonTypes::Fairy)	=> Multiplier(2.0),
			(PokemonTypes::Bug,			PokemonTypes::Fairy)	=> Multiplier(0.5),
			(PokemonTypes::Steel,		PokemonTypes::Fairy)	=> Multiplier(2.0),
			(PokemonTypes::Dragon,		PokemonTypes::Fairy)	=> Multiplier(0.0),
			(PokemonTypes::Dark,		PokemonTypes::Fairy)	=> Multiplier(0.5),
			// all regular multiplliers
			(_, _) => Multiplier(1.0),
		}
	}
}


impl Display for PokemonTypes {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}
