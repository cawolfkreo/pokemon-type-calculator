use core::fmt;
use std::fmt::Display;

pub mod poke_type;
pub mod multiplier;

pub use poke_type::PokemonTypes;
pub use multiplier::Multiplier;

pub enum DefenseType {
	OneType(poke_type::PokemonTypes),
	TwoTypes(poke_type::PokemonTypes, poke_type::PokemonTypes)
}

impl DefenseType {
	fn calculate_damage_multiplier(&self, attack_type: &poke_type::PokemonTypes) -> Multiplier {
		match self {
			Self::OneType(defense_type) => defense_type.calculate_damage_multiplier(attack_type),
			Self::TwoTypes(defense_1, defense_2 ) => {
				defense_1.calculate_damage_multiplier(attack_type) * defense_2.calculate_damage_multiplier(attack_type)
			}
		}
	}
}

impl Display for DefenseType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			DefenseType::OneType(defense) => write!(f, "{defense}"),
			DefenseType::TwoTypes(def1, def2) => write!(f, "{def1}/{def2}")
		}
	}
}

pub struct Fight {
	pub attack_type: poke_type::PokemonTypes,
	pub defensive_types: DefenseType,
}

impl Fight {
	pub fn new(attack_type: poke_type::PokemonTypes, defensive_types: DefenseType) -> Self{
		Self {
			attack_type, defensive_types
		}
	}

	pub fn calculate_effectiveness(&self) -> Effectivenes {
		let damage_multiplier = self.defensive_types.calculate_damage_multiplier(&self.attack_type);

		Effectivenes::new(damage_multiplier)
	}
}

impl Display for Fight {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Move type {} attacks {}.", self.attack_type, self.defensive_types)
	}
}

pub enum Effectivenes {
	SuperEffective(Multiplier),
	Regular(Multiplier),
	NotVeryEffective(Multiplier),
	HasNoEffect(Multiplier),
}

impl Effectivenes {
	pub fn new(multiplier: Multiplier) -> Self {
		if multiplier == 0.0 {
			return Effectivenes::HasNoEffect(multiplier);
		}

		if multiplier > 0.0 && multiplier < 1.0  {
			return Effectivenes::NotVeryEffective(multiplier);
		}

		if multiplier >= 2.0 {
			return Effectivenes::SuperEffective(multiplier);
		} 

		Effectivenes::Regular(multiplier)
	}

	pub fn get_multiplier(&self) -> Multiplier {
		let mul_ref = match self {
			Effectivenes::SuperEffective(mul) => mul,
			Effectivenes::Regular(mul) => mul,
			Effectivenes::NotVeryEffective(mul) => mul,
			Effectivenes::HasNoEffect(mul) => mul,
		};

		mul_ref.clone()
	}
}

impl Display for Effectivenes {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let message = match self {
			Effectivenes::SuperEffective(_) => String::from("It's super effective!"),
			Effectivenes::Regular(_) => String::from("regular damage."),
			Effectivenes::NotVeryEffective(_) => String::from("Not very effective..."),
			Effectivenes::HasNoEffect(_) => String::from("No effect!"),
		};
		write!(f, "{message} Multiplier = {}", self.get_multiplier())
	}
}

#[cfg(test)]
pub mod test {
	use super::*;

	const EXPECTED_MULTI: Multiplier = Multiplier(2.0);

	const EXPECTED_EFFECTIVENESS: Effectivenes = Effectivenes::SuperEffective(EXPECTED_MULTI);

	#[test]
	pub fn should_return_correct_multiplier() {
		// Arrange
		let test_effectiveness: Effectivenes = Effectivenes::new(EXPECTED_MULTI);

		// Act
		let act = test_effectiveness.get_multiplier();

		// Assert
		assert_eq!(act.0, EXPECTED_MULTI.0, "The received multiplier {act} is different from the expected {EXPECTED_MULTI}");
	}

	#[test]
	pub fn should_show_the_right_effectiveness() {
		// Arrange

		// ACT
		let test_effectiveness: Effectivenes = Effectivenes::new(EXPECTED_MULTI);

		//Assert
		match (EXPECTED_EFFECTIVENESS, test_effectiveness) {
			(Effectivenes::HasNoEffect(_), Effectivenes::HasNoEffect(_)) => (),
			(Effectivenes::NotVeryEffective(_), Effectivenes::NotVeryEffective(_)) => (),
			(Effectivenes::Regular(_), Effectivenes::Regular(_)) => (),
			(Effectivenes::SuperEffective(_), Effectivenes::SuperEffective(_)) => (),
			(_, test_effectiveness) => panic!("The effectiveness received '{test_effectiveness}' is not the expected one: '{EXPECTED_EFFECTIVENESS}'"),
		}
	}
}
