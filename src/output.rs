use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Output<O> {
    value: Rc<RefCell<Option<O>>>,
}

impl<O> Output<O> {
    pub fn new(value: Rc<RefCell<Option<O>>>) -> Output<O> {
        Output { value: value }
    }
}

pub struct OutputHandler<O> {
    output: Option<Output<O>>,
}

impl<O> OutputHandler<O> {
    pub fn new(value: Option<Output<O>>) -> OutputHandler<O> {
        OutputHandler { output: value }
    }
}

impl<O: Clone> OutputHandler<O> {
    pub fn default(&self, default: O) -> O {
        if let Some(ref output) = self.output {
            return output.return_value(default);
        }
        default
    }
}

impl<O: Clone> Output<O> {
    pub fn will_return(&self, value: O) {
        *self.value.borrow_mut() = Some(value);
    }

    fn return_value(&self, default: O) -> O {
        match *self.value.borrow() {
            Some(ref value) => value.clone(),
            None => default,
        }
    }
}
