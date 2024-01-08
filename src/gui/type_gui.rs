use std::default::Default;
use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Debug, PartialEq, Default)]
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
            "Normal",
            "Fighting",
            "Flying",
            "Poison",
            "Ground",
            "Rock",
            "Bug",
            "Ghost",
            "Steel",
            "Fire",
            "Water",
            "Grass",
            "Electric",
            "Psychic",
            "Ice",
            "Dragon",
            "Dark",
            "Fairy",
            "None"
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
