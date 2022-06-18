// #[cfg_attr(test, mock_it::mock_it)]
#[mock_it::mock_it]
trait Nurse {
    fn heal(&self, pokemon: Pokemon) -> Result<Pokemon, String>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct Pokemon {
    hp: i32,
}

struct PokemonCenter {
    nurse: Box<dyn Nurse>,
    pokemons: Vec<Pokemon>,
}

impl PokemonCenter {
    pub fn accept(&mut self, pokemon: Pokemon) {
        self.pokemons.push(pokemon);
    }

    pub fn collect(&mut self) -> Result<Pokemon, String> {
        let pokemon = match self.pokemons.pop() {
            Some(val) => val,
            None => return Err("No pokemon".to_string()),
        };
        self.nurse.heal(pokemon)
    }
}

// #[cfg(test)]
pub mod tests {
    use super::*;
    use mock_it::{any, eq};

    // #[test]
    pub fn can_heal_pokemon() {
        // given
        let pikachu_no_hp = Pokemon { hp: 0 };
        let pikachu_full_hp = Pokemon { hp: 100 };

        let nurse_joy = NurseMock::new();
        nurse_joy
            .when_heal(eq(pikachu_no_hp.clone()))
            .will_return(Ok(pikachu_full_hp.clone()));

        let mut pokemon_center = PokemonCenter {
            nurse: Box::new(nurse_joy.clone()),
            pokemons: vec![],
        };

        // when
        pokemon_center.accept(pikachu_no_hp);
        let healed_pikachu = pokemon_center.collect().unwrap();

        //then
        assert_eq!(healed_pikachu, pikachu_full_hp);
        assert!(nurse_joy.expect_heal(any()).times(1).called());
    }
}

fn main() {
    tests::can_heal_pokemon();
}
