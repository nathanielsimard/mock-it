# Mock It
[![Build Status](https://travis-ci.org/nathanielsimard/mock_it.svg?branch=master)](https://travis-ci.org/nathanielsimard/mock_it)
[![codecov.io](https://codecov.io/gh/nathanielsimard/mock_it/coverage.svg?branch=master)](https://codecov.io/gh/nathanielsimard/mock_it)
[![Current Crates.io Version](https://img.shields.io/crates/v/mock-it.svg)](https://crates.io/crates/mock-it)

This library aims to make mocking reliable.
Most mocking libraries in Rust are experimental using code generation with rust nightly.
It is great in term of ease of use, but it has limitations.
It is not the most robust way of mocking yet.
This is where `Mock_it` tries to fill the gap. 
You will have to implement the trait you are willing to mock, but without coding any logic.
This way you can be sure your mock works as expected without having to maintain it much.

# Install

Specify this crate as `[dev-dependencies]`.

```toml
[dev-dependencies]
mock-it = "0.1.0"
```

```rust
#[cfg(test)] // <-- not needed in integration tests
extern crate mock_it;
```

# Usage

First, you need to have a `trait` that you want to mock. Let's take a Factory that creates persons.

```rust
trait PersonFactory {
    fn create(&self, name: String, surname: String) -> Result<Person, String>;
}
```

Each function of the `trait` will need one mock.

```rust
#[derive(Clone)]
struct PersonFactoryMock {
    create: Mock<(String, String), Result<Person, String>>,
}
```

In this case, our mock has one field `create` that will be used for mocking the function of the same name.
The mock signature is `Mock<Input, Ouput>`.
In this case we have two strings as input, so we use a tuple of strings instead.
Note that the mock can be cloned without losing his internal state because it uses inner mutability with ref counting.
Now we can implement the `trait`.

```rust
impl PersonFactory for PersonFactoryMock {
    fn create(&self, name: String, surname: String) -> Result<Person, String> {
        self.create.called((name.clone(), surname.clone()))
    }
}
```

The only thing to do here is to pass the inputs in a tuple to the function `called` to match the mock signature.
To facilitate the use of the mock we can create a `new` function.

```rust
impl PersonFactoryMock {
    fn new() -> PersonFactoryMock {
        PersonFactoryMock {
            create: Mock::new(Err("Default value".to_string())),
        }
    }
}
```

We can see that we need to pass a value into the constructor.
This is the default value returned by the function if no rule is satisfied.
Now we need to create the mock and add some rules.

```rust
let person_factory_mock = PersonFactoryMock::new();
person_factory_mock
    .create
    .given((String::from("MyName"), String::from("MySurname")))
    .will_return(Ok(Person::new(String::from("MyName"), String::from("MySurname"))));

let person_factory = Box::new(person_factory_mock.clone());

let default_value = person_factory.create(String::from("YourName"), String::from("YourSurname"));
let me = person_factory.create(String::from("MyName"), String::from("MySurname"));
```

The first call will return the default value which is an error and the second call will return the person with my name and surname.
If we want to do assertion, it is also possible.

```rust
assert!(
    person_factory_mock
        .create
        .was_called_with((String::from("MyName"), String::from("MySurname")))
);
```

That's it ! If you want more examples, just check [here](examples).