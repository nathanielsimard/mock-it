use mock_it::mock_it;

fn main() {
    println!("Hello, world!");
}

#[mock_it]
trait Nurse {
    fn heal(&self, pokemon: Pokemon) -> Pokemon;
}

#[derive(PartialEq, Clone)]
pub struct Pokemon {
    hp: i32,
}

struct Hospital {
    nurse: dyn Nurse,
}

impl Hospital {
    fn heal(&self, pokemon: Pokemon) -> Pokemon {
        self.nurse.heal(pokemon)
    }
}

#[test]
fn can_heal_pokemon() {
    let joy = NurseMock {};

    let hospital = Hospital { nurse: joy };

    let pikachu_no_hp = Pokemon { hp: 0 };
    let pikachu_no_hp = Pokemon { hp: 100 };

    joy.when_heal(&pikachu_no_hp).will_return(&pikachu_full_hp);

    let healed_pikachu = hospital.heal(pikachu_no_hp);

    assert!(healed_pikachu, pikachu_full_hp)
}
