use std::cell::RefCell;
use std::rc::Rc;

pub fn verify<I>(validator: Validator<I>) -> bool {
    validator.result
}

pub struct Validator<I> {
    calls: Rc<RefCell<Vec<I>>>,
    result: bool,
    input: I,
}

impl<I: PartialEq> Validator<I> {
    pub fn new(calls: Rc<RefCell<Vec<I>>>, result: bool, input: I) -> Validator<I> {
        Validator {
            calls: calls,
            result: result,
            input: input,
        }
    }

    pub fn times(mut self, times: usize) -> Validator<I> {
        if times == 0 {
            panic!("Can't call `times` with 0, use `was_call_with` instead");
        }

        let mut counter = 0;
        for value in &*self.calls.borrow() {
            if value == &self.input {
                counter += 1;
            }
        }
        if counter != times {
            self.result = false
        }
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn times_called_with_zero_should_panic() {
        let validator = Validator::new(Rc::new(RefCell::new(Vec::new())), true, 5);
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
            let validator =
                Validator::new(Rc::new(RefCell::new(calls.clone())), initial_result, input);

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
