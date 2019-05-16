use mock_it::*;

#[derive(Clone)]
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
    create: Mock<(String, String), Result<Person, String>>,
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
        self.create.called((name.clone(), surname.clone()))
    }
}

fn main() {
    let person_factory_mock = PersonFactoryMock::new();
    let person_factory = Box::new(person_factory_mock.clone());
    let a_name = "John".to_string();
    let a_surname = "Bouchard".to_string();

    person_factory_mock
        .create
        .given((a_name.clone(), a_surname.clone()))
        .will_return(Ok(Person::new(a_name.clone(), a_surname.clone())));

    let _ = person_factory.create(a_name.clone(), a_surname.clone());

    assert!(verify(
        person_factory_mock
            .create
            .was_called_with((a_name, a_surname))
    ));
}
