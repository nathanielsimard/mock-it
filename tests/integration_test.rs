#[macro_use]
extern crate table_test;

use mock_it::Matcher;
use mock_it::Matcher::*;
use mock_it::*;

#[derive(PartialEq, Debug, Clone)]
struct Person {
    name: String,
    surname: String,
}

impl Person {
    fn new(name: String, surname: String) -> Person {
        Person {
            name,
            surname,
        }
    }
}

trait PersonFactory {
    fn create(&self, name: String, surname: String) -> Result<Person, String>;
}

#[derive(Clone)]
struct PersonFactoryMock {
    create: Mock<(Matcher<String>, Matcher<String>), Result<Person, String>>,
}

impl PersonFactoryMock {
    fn new() -> PersonFactoryMock {
        PersonFactoryMock {
            create: Mock::new(Err("".to_string())),
        }
    }
}

impl PersonFactory for PersonFactoryMock {
    fn create(&self, name: String, surname: String) -> Result<Person, String> {
        self.create
            .called((Val(name.clone()), Val(surname.clone())))
    }
}

#[test]
fn will_return() {
    let table = vec![
        (
            ("John".to_string(), "Bouchard".to_string()),
            Ok(Person::new("John".to_string(), "Bouchard".to_string())),
        ),
        (
            ("".to_string(), "".to_string()),
            Ok(Person::new("".to_string(), "".to_string())),
        ),
    ];

    for (test_case, (name, surname), person) in table_test!(table) {
        let person_factory_mock = PersonFactoryMock::new();
        let person_factory = Box::new(person_factory_mock.clone());

        person_factory_mock
            .create
            .given((Val(name.clone()), Val(surname.clone())))
            .will_return(Ok(Person::new(name.clone(), surname.clone())));

        let actual = person_factory.create(name.clone(), surname.clone());
        test_case
            .given(&format!("name: {:?}, surname: {:?}", name, surname))
            .when("add rule 'will return' to create")
            .then("calling create return the person with the same name and surname")
            .assert_eq(person, actual);
    }
}

#[test]
fn given_person_factory_mock_then_call_it_4_times_with_any_values_then_validate_times_4_return_true(
) {
    let times = 4;
    let name = "John".to_string();
    let surname = "Bouchard".to_string();
    let person_factory_mock = PersonFactoryMock::new();
    let person_factory = Box::new(person_factory_mock.clone());

    person_factory_mock
        .create
        .given((Val(name.clone()), Val(surname.clone())))
        .will_return(Ok(Person::new(name.clone(), surname.clone())));

    for i in 0..times {
        let _ = person_factory.create(name.clone(), i.to_string());
    }

    assert!(verify(
        person_factory_mock
            .create
            .was_called_with((Any, Any))
            .times(times),
    ));
}
