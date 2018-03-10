extern crate mock_it;
use mock_it::Mock;

#[derive(PartialEq, Clone)]
struct Person {
    name: String,
    age: i64,
}

impl Person {
    fn new(name: &str, age: i64) -> Person {
        Person {
            name: name.to_string(),
            age: age,
        }
    }
}

trait Repository {
    fn persist(&self, person: Person) -> Option<String>;
}

struct Service {
    repository: Box<Repository>,
}

impl Service {
    fn new(repository: Box<Repository>) -> Service {
        Service {
            repository: repository,
        }
    }

    fn persist(&self, name: &str, age: i64) -> Result<String, String> {
        if age <= 0 {
            Err("Age invalid".to_string())
        } else if name == String::from("") {
            Err("Empty name".to_string())
        } else {
            let name = self.repository.persist(Person::new(&name, age));
            if let Some(name) = name {
                Ok(name)
            } else {
                Err("Unable to persist".to_string())
            }
        }
    }
}

#[derive(Clone)]
struct RepositoryMock {
    persist: Mock<Person, Option<String>>,
}

impl RepositoryMock {
    fn new() -> RepositoryMock {
        RepositoryMock {
            persist: Mock::new(None),
        }
    }
}

impl Repository for RepositoryMock {
    fn persist(&self, person: Person) -> Option<String> {
        self.persist.called(person)
    }
}

fn main() {
    let a_valid_person = Person::new("John", 27);
    let repository_mock = RepositoryMock::new();

    repository_mock
        .persist
        .given(a_valid_person.clone())
        .will_return(Some(a_valid_person.name.clone()));

    let service = Service::new(Box::new(repository_mock.clone()));

    let result = service.persist(&a_valid_person.name, a_valid_person.age);

    assert_eq!(Ok(a_valid_person.name.clone()), result);
    assert!(repository_mock.persist.was_called_with(a_valid_person));
}
