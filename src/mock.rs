use crate::rule::Rule;
use crate::validator::*;
use crate::when::When;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Mock<I, O> {
    name: String,
    calls: Arc<Mutex<Vec<I>>>,
    rules: Arc<Mutex<Vec<Rule<I, O>>>>,
}

impl<I, O> Clone for Mock<I, O> {
    fn clone(&self) -> Mock<I, O> {
        Mock {
            name: self.name.clone(),
            calls: self.calls.clone(),
            rules: self.rules.clone(),
        }
    }
}

impl<I, O> Mock<I, O> {
    pub fn new(name: String) -> Mock<I, O> {
        Mock {
            name,
            calls: Arc::new(Mutex::new(Vec::new())),
            rules: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl<I, O: Clone> Mock<I, O> {
    pub fn when(&self, input: I) -> When<I, O> {
        When::new(input, self.rules.clone())
    }
}

impl<I: PartialEq + std::fmt::Debug, O: Clone> Mock<I, O> {
    pub fn called(&self, input: I) -> O {
        let input_str = format!("{:?}", input);

        // Get the when value for this input
        let rules = self.rules.lock().unwrap();
        let when_value = rules.iter().find(|value| value.input == input);

        // Record this call
        self.calls.lock().unwrap().push(input);

        // Return the when value, or fail if there is no when value
        match when_value {
            Some(value) => value.output.clone(),
            None => panic!(
                "Mock \"{}\" called with unexpected input: {:?}, did you forget to configure your mock ?",
                self.name, input_str
            ),
        }
    }
}

impl<I, O: Clone> Mock<I, O> {
    pub fn was_called_with(&self, input: I) -> Validator<I> {
        Validator::new(self.calls.clone(), input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    trait ATrait {
        fn int_to_string(&self, input: i64) -> String;
    }

    #[derive(Clone)]
    struct MyMock {
        int_to_string: Mock<i64, String>,
    }

    impl MyMock {
        fn new() -> MyMock {
            MyMock {
                int_to_string: Mock::new("AMockName".to_string()),
            }
        }
    }

    impl ATrait for MyMock {
        fn int_to_string(&self, input: i64) -> String {
            self.int_to_string.called(input)
        }
    }

    /// Given some rules, the mock should return the output in the rule when
    /// called
    #[test]
    fn mock_output() {
        let mock = MyMock::new();
        mock.int_to_string.when(65).will_return(String::from("65"));
        mock.int_to_string.when(63).will_return(String::from("63"));
        mock.int_to_string.when(-1).will_return(String::from("-1"));
        let a_trait = Box::new(mock);

        assert_eq!("65", a_trait.int_to_string(65));
        assert_eq!("63", a_trait.int_to_string(63));
        assert_eq!("-1", a_trait.int_to_string(-1));
    }

    /// The mock records the input it was called with
    #[test]
    fn was_called_with() {
        let mock = MyMock::new();
        mock.int_to_string.when(65).will_return(String::from(""));
        mock.int_to_string.when(63).will_return(String::from(""));
        mock.int_to_string.when(-1).will_return(String::from(""));
        let a_trait = Box::new(mock.clone());

        a_trait.int_to_string(65);
        a_trait.int_to_string(63);
        a_trait.int_to_string(-1);

        assert!(verify(mock.int_to_string.was_called_with(65)));
        assert!(verify(mock.int_to_string.was_called_with(63)));
        assert!(verify(mock.int_to_string.was_called_with(-1)));
        assert!(!verify(mock.int_to_string.was_called_with(0)));
    }

    #[test]
    fn given_mock_called_5_times_when_times_5_then_return_true_false_otherwise() {
        let mock = MyMock::new();
        mock.int_to_string.when(65).will_return(String::from("65"));
        let a_trait = Box::new(mock.clone());

        for _ in 0..5 {
            a_trait.int_to_string(65);
        }

        assert_eq!(
            true,
            verify(mock.int_to_string.was_called_with(65).times(5))
        );
        assert_eq!(
            false,
            verify(mock.int_to_string.was_called_with(65).times(4))
        );
        assert_eq!(
            false,
            verify(mock.int_to_string.was_called_with(65).times(1))
        );
        assert_eq!(
            false,
            verify(mock.int_to_string.was_called_with(65).times(6))
        );
    }
}
