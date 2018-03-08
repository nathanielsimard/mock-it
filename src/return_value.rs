pub struct ReturnValue<R> {
    return_value: Option<fn() -> R>,
    return_value_clonable: Option<R>,
}

impl<R> ReturnValue<R> {
    pub fn new() -> ReturnValue<R> {
        ReturnValue {
            return_value: None,
            return_value_clonable: None,
        }
    }
}

impl<R> ReturnValue<R> {
    pub fn will_return(&mut self, value_factory: fn() -> R) {
        self.return_value = Some(value_factory);
    }

    pub fn return_value(&self, default: R) -> R {
        match self.return_value {
            Some(ref factory) => factory(),
            None => default,
        }
    }
}

impl<R: Clone> ReturnValue<R> {
    pub fn will_return_clone_of(&mut self, value: R) {
        self.return_value_clonable = Some(value);
    }

    pub fn return_value_cloned(&self, default: R) -> R {
        match self.return_value_clonable.clone() {
            Some(value) => value,
            None => default,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait ATrait {
        fn create_string(&self) -> String;
    }

    struct MyMock {
        create_string: ReturnValue<String>,
    }

    impl MyMock {
        fn new() -> MyMock {
            MyMock {
                create_string: ReturnValue::new(),
            }
        }
    }

    impl ATrait for MyMock {
        fn create_string(&self) -> String {
            self.create_string.return_value(String::new())
        }
    }

    struct OtherMock {
        create_string: ReturnValue<String>,
    }

    impl OtherMock {
        fn new() -> OtherMock {
            OtherMock {
                create_string: ReturnValue::new(),
            }
        }
    }

    impl ATrait for OtherMock {
        fn create_string(&self) -> String {
            self.create_string.return_value_cloned(String::new())
        }
    }

    #[test]
    fn it_can_mock_one_return_value() {
        let mut my_mock = MyMock::new();
        my_mock
            .create_string
            .will_return(|| -> String { String::from("Something") });

        let a_trait = Box::new(my_mock);

        assert_eq!("Something", a_trait.create_string());
    }

    #[test]
    fn it_can_mock_one_clonable_return_value() {
        let mut my_mock = OtherMock::new();
        my_mock
            .create_string
            .will_return_clone_of(String::from("Something"));

        let a_trait = Box::new(my_mock);

        assert_eq!("Something", a_trait.create_string());
    }
}
