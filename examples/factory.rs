use mock_it::{eq, mock_it};

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
    fn create(&self, name: &str, surname: &str) -> Person;
}

fn main() {
    let person_factory_mock = PersonFactoryMock::new();
    let a_name = "John".to_string();
    let a_surname = "Bouchard".to_string();

    let person_factory = Box::new(person_factory_mock.clone());
    person_factory_mock
        .when_create(eq(&a_name), eq(&a_surname))
        .will_return(Person::new(a_name.clone(), a_surname.clone()));

    let person = person_factory.create(&a_name, &a_surname);

    assert!(person_factory_mock
        .expect_create(eq(&a_name), eq(&a_surname))
        .times(1)
        .called());

    assert_eq!(person, Person::new(a_name, a_surname))
}
