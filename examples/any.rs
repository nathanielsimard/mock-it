extern crate mock_it;
use mock_it::mock::Mock;
use mock_it::input::Input;
use mock_it::input::Input::*;

#[derive(Debug, PartialEq, Clone)]
struct House {
    price: f64,
}

impl House {
    fn new(price: f64) -> House {
        House { price: price }
    }
}

trait HouseFactory {
    fn create(&self, specs: Vec<String>) -> Result<House, String>;
}

struct HouseFactoryMock {
    create: Mock<Input<Vec<String>>, Result<House, String>>,
}

impl HouseFactoryMock {
    fn new() -> HouseFactoryMock {
        HouseFactoryMock {
            create: Mock::new(),
        }
    }
}

impl HouseFactory for HouseFactoryMock {
    fn create(&self, specs: Vec<String>) -> Result<House, String> {
        self.create
            .called_with(Val(specs.clone()))
            .return_value_with_default(Err(format!("No rule when called with {:?}", specs)))
    }
}

fn main() {
    let house_factory_mock = HouseFactoryMock::new();
    let expected_house = House::new(250_000.99);
    house_factory_mock
        .create
        .given(Any)
        .will_return(Ok(expected_house.clone()));

    let house_factory = Box::new(house_factory_mock);
    let house = house_factory.create(vec![]);

    match house {
        Ok(house) => assert_eq!(expected_house, house),
        Err(message) => panic!(message),
    }
}
