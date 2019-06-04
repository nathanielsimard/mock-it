use mock_it::{mock_it, verify};

#[derive(Clone, Default)]
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
    fn create(&self, name: String, surname: String) -> Person ;
}

//impl Clone for PersonFactoryMock {
//    fn clone(&self) -> Self {
//        PersonFactoryMock {
//            create: self.create.clone()
//        }
//    }
//}

fn main() {
    let person_factory_mock = PersonFactoryMock::new();
    let a_name = "John".to_string();
    let a_surname = "Bouchard".to_string();

    let person_factory = Box::new(person_factory_mock.clone());
    person_factory_mock
        .create
        .given((a_name.clone(), a_surname.clone()))
        .will_return(Person::new(a_name.clone(), a_surname.clone()));

    let _ = person_factory.create(a_name.clone(), a_surname.clone());

    assert!(verify(
        person_factory_mock
            .create
            .was_called_with((a_name, a_surname))
    ));
}
