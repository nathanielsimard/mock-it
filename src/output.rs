use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Output<O> {
    value: Rc<RefCell<O>>,
}

impl<O> Output<O> {
    pub fn new(value: Rc<RefCell<O>>) -> Output<O> {
        Output { value: value }
    }
}

impl<O: Clone> Output<O> {
    pub fn will_return(&self, value: O) {
        *self.value.borrow_mut() = value;
    }
}

pub fn value_of<O: Clone>(output: Output<O>) -> O {
    output.value.borrow().clone()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_default_output_when_will_return_expected_then_should_be_expected() {
        let default = 0;
        let expected = 5;
        let output = Output::new(Rc::new(RefCell::new(default)));

        output.will_return(expected);
        let actual = value_of(output);

        assert_eq!(expected, actual);
    }
    #[test]
    fn given_default_output_when_dont_call_will_return_then_should_be_default() {
        let default = 0;
        let output = Output::new(Rc::new(RefCell::new(default)));

        let actual = value_of(output);

        assert_eq!(default, actual);
    }
}
