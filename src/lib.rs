mod pokemon;

use log::{info, log_enabled, Level};

pub use pokemon::{Fight, PokemonTypes, DefenseType};

pub fn example() {
    let defense = DefenseType::TwoTypes(PokemonTypes::Fire, PokemonTypes::Ghost);

    let fight = Fight::new(PokemonTypes::Bug, defense);

    let effectivenes = fight.calculate_effectiveness();

    if log_enabled!(Level::Info) {
        info!("{fight}{effectivenes}");
    }
}
