<div style="display: flex; justify-content: left; align-items: center;">
    <img src="./assets/mock-it.svg" width="100px" />
    <span style="margin-left: 24px;font-size: 48px; font-weight: 500;">mock-it</span>
</div>

<br/>

[![Current Crates.io Version](https://img.shields.io/crates/v/mock-it.svg)](https://crates.io/crates/mock-it)

> Mock it, don't mock all 🙃

Our goal is to enhance the DX behind mocking your depedencies when you test.
It lets you use a syntax closer to `given` `when` `then` instead of having to
assert your `then` **BEFORE** you call your function.

## Features

* Intuitive usage 😌
* Mock your traits 🦾
* Configure your mocks 👷‍♀️
* Separate configuration from assertion 🕵️‍♀️

## Example

``` rust
#[cfg_attr(test, mock_it::mock_it)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use mock_it::{any, eq};

    #[test]
    fn can_heal_pokemon() {
        // given
        let pikachu_no_hp = Pokemon { hp: 0 };
        let pikachu_full_hp = Pokemon { hp: 100 };

        let nurse_joy = NurseMock::new();
        nurse_joy.when_heal(eq(pikachu_no_hp.clone()))
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
```

## Constraints

* Traits inputs must implement both PartialEq and Clone
* Traits ouput must implement both Clone
