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
        Person { name, surname }
    }
}

trait PersonFactory<'a, T, E>
where
    T: Into<String>,
    E: std::error::Error,
{
    fn create(&self, name: String, surname: T) -> Result<&'a Person, E>;
}

#[derive(Clone)]
struct PersonFactoryMock<'a, T, E>
where
    T: Into<String>,
    E: std::error::Error,
    T: Clone + PartialEq,
    E: Clone,
{
    create: Mock<(Matcher<String>, Matcher<T>), Result<&'a Person, E>>,
}

impl<'a, T: Into<String> + Clone + PartialEq, E: std::error::Error> PersonFactoryMock<'a, T, E>
where
    E: Clone + PartialEq,
{
    fn new() -> PersonFactoryMock<'a, T, E> {
        PersonFactoryMock {
            create: Mock::new(),
        }
    }
}

impl<'a, T, E> PersonFactory<'a, T, E> for PersonFactoryMock<'a, T, E>
where
    T: Into<String> + PartialEq + Clone,
    E: std::error::Error,
    E: Clone + PartialEq,
{
    fn create(&self, name: String, surname: T) -> Result<&'a Person, E> {
        self.create
            .called((Val(name.clone()), Val(surname.clone())))
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

#[test]
fn will_return() {
    let table = vec![
        (
            ("John".to_string(), "Bouchard".to_string()),
            Person::new("John".to_string(), "Bouchard".to_string()),
        ),
        (
            ("".to_string(), "".to_string()),
            Person::new("".to_string(), "".to_string()),
        ),
    ];

    for (test_case, (name, surname), person) in table_test!(table) {
        let expected_person = Person::new(name.clone(), surname.clone());
        let person_factory_mock = PersonFactoryMock::new();
        let person_factory: Box<dyn PersonFactory<'_, String, Error>> =
            Box::new(person_factory_mock.clone());

        person_factory_mock
            .create
            .given((Val(name.clone()), Val(surname.clone())))
            .will_return(Ok(&expected_person));

        let actual = person_factory
            .create(name.clone(), surname.clone())
            .unwrap();
        test_case
            .given(&format!("name: {:?}, surname: {:?}", name, surname))
            .when("add rule 'will return' to create")
            .then("calling create return the person with the same name and surname")
            .assert_eq(&person, actual);
    }
}

/// The mock can validate the number of times it was called when using `Matcher`
#[test]
fn validate_times_using_matcher() {
    let times = 4;
    let name = "John".to_string();
    let surname = "Bouchard".to_string();
    let expected_person = Person::new(name.clone(), surname.clone());
    let person_factory_mock = PersonFactoryMock::new();
    let person_factory: Box<dyn PersonFactory<'_, String, Error>> =
        Box::new(person_factory_mock.clone());

    person_factory_mock
        .create
        .given((Val(name.clone()), Any))
        .will_return(Ok(&expected_person));

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
