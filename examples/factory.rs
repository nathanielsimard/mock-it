use mock_it::{eq, mock_it, verify};

#[derive(Clone, Default, PartialEq, Debug)]
pub struct Person {
    name: String,
    surname: String,
}

impl Person {
    fn new(name: String, surname: String) -> Person {
        Person { name, surname }
    }
}

#[mock_it]
trait PersonFactory {
    fn create(&self, name: String, surname: String) -> Person;
}

fn main() {
    let person_factory_mock = PersonFactoryMock::new();
    let a_name = "John".to_string();
    let a_surname = "Bouchard".to_string();

    let person_factory = Box::new(person_factory_mock.clone());
    person_factory_mock
        .create_with(eq(a_name.clone()), eq(a_surname.clone()))
        .will_return(Person::new(a_name.clone(), a_surname.clone()));

    let person = person_factory.create(a_name.clone(), a_surname.clone());

    assert!(verify(person_factory_mock.create_was_called_with(
        eq(a_name.clone()),
        eq(a_surname.clone())
    )));

    assert_eq!(person, Person::new(a_name, a_surname))
}
