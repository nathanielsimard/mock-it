use crate::output;
use crate::output::Output;
use crate::rule::Rule;
use crate::validator::*;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Mock<I, O> {
    calls: Arc<Mutex<Vec<I>>>,
    rules: Arc<Mutex<Vec<Rule<I, Output<O>>>>>,
    default: O,
}

impl<I, O: Clone> Clone for Mock<I, O> {
    fn clone(&self) -> Mock<I, O> {
        Mock {
            calls: self.calls.clone(),
            rules: self.rules.clone(),
            default: self.default.clone(),
        }
    }
}

impl<I: PartialEq, O: Clone> Mock<I, O> {
    pub fn new(default: O) -> Mock<I, O> {
        Mock {
            calls: Arc::new(Mutex::new(Vec::new())),
            rules: Arc::new(Mutex::new(Vec::new())),
            default,
        }
    }

    pub fn given(&self, input: I) -> Output<O> {
        let output = Output::new(Arc::new(Mutex::new(self.default.clone())));
        self.rules
            .lock()
            .unwrap()
            .push(Rule::new(input, output.clone()));

        output
    }

    pub fn called(&self, input: I) -> O {
        // Get the given value for this input
        let rules = self.rules.lock().unwrap();
        let given_value = rules.iter().find(|value| value.input == input);

        // Record this call
        self.calls.lock().unwrap().push(input);

        // Return the given value, or the default if there is no given value
        match given_value {
            Some(value) => output::value_of(value.output.clone()),
            None => self.default.clone(),
        }
    }

    pub fn was_called_with(&self, input: I) -> Validator<I> {
        let calls = self.calls.lock().unwrap();
        let was_called = calls.iter().any(|value| value == &input);

        Validator::new(self.calls.clone(), was_called, input)
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
                int_to_string: Mock::new(String::new()),
            }
        }
    }

    impl ATrait for MyMock {
        fn int_to_string(&self, input: i64) -> String {
            self.int_to_string.called(input)
        }
    }

    #[test]
    fn given_mock_with_rules_when_call_it_then_it_should_respect_those_rules() {
        let mock = MyMock::new();
        mock.int_to_string.given(65).will_return(String::from("65"));
        mock.int_to_string.given(63).will_return(String::from("63"));
        mock.int_to_string.given(-1).will_return(String::from("-1"));
        let a_trait = Box::new(mock);

        assert_eq!("65", a_trait.int_to_string(65));
        assert_eq!("63", a_trait.int_to_string(63));
        assert_eq!("-1", a_trait.int_to_string(-1));
        assert_eq!("", a_trait.int_to_string(0));
    }

    #[test]
    fn given_mock_called_with_values_when_was_called_with_then_is_true_for_those_values_false_otherwise(
    ) {
        let mock = MyMock::new();
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
        mock.int_to_string.given(65).will_return(String::from("65"));
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
