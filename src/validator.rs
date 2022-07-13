use std::sync::Arc;
use std::sync::Mutex;

pub fn verify<I: PartialEq>(validator: Validator<I>) -> bool {
    validator.called()
}

pub struct Validator<I> {
    calls: Arc<Mutex<Vec<I>>>,
    result: Option<bool>,
    input: I,
}

impl<I> Validator<I> {
    pub fn new(calls: Arc<Mutex<Vec<I>>>, input: I) -> Validator<I> {
        Validator {
            calls,
            result: None,
            input,
        }
    }
}

impl<I: PartialEq> Validator<I> {
    pub fn result(&mut self) -> bool {
        match self.result {
            Some(val) => val,
            None => {
                let calls = self.calls.lock().unwrap();
                let was_called = calls.iter().any(|value| value == &self.input);
                self.result = Some(was_called);
                was_called
            }
        }
    }
    pub fn times(mut self, times: usize) -> Validator<I> {
        if times == 0 {
            panic!("Can't call `times` with 0, use `was_call_with` instead");
        }

        let times_called = {
            let calls = self.calls.lock().unwrap();
            calls.iter().filter(|value| *value == &self.input).count()
        };

        if times_called != times {
            self.result = Some(false)
        }

        self
    }
    pub fn called(self) -> bool {
        let mut this = self;
        this.result()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use table_test::table_test;

    #[test]
    #[should_panic]
    fn times_called_with_zero_should_panic() {
        let validator = Validator::new(Arc::new(Mutex::new(Vec::new())), 5);
        validator.times(0);
    }

    #[test]
    fn times() {
        let table = vec![
            ((vec![1, 1, 3], true, 1, 1), false),
            ((vec![1, 1, 3], true, 1, 2), true),
            ((vec![1, 1, 3], true, 3, 1), true),
            ((vec![1, 1, 3], false, 3, 1), false),
            ((vec![1, 1, 3], false, 3, 1), false),
            ((vec![1, 1, 3], false, 3, 1), false),
        ];

        for (test_case, (calls, initial_result, input, times), expected) in table_test!(table) {
            let mut validator = Validator::new(Arc::new(Mutex::new(calls.clone())), input);
            validator.result = Some(initial_result);

            let actual = verify(validator.times(times));

            test_case
                .given(&format!(
                    "Validator: calls {:?}, initial_result {:?}, input {:?}",
                    calls, initial_result, input
                ))
                .given(&format!("times {:?}", times))
                .when("verify validator")
                .then(&format!("{:?}", expected))
                .assert_eq(expected, actual);
        }
    }
}
