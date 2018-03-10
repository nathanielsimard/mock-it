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

impl<O: Clone> Output<O> {
    pub fn will_return(&self, value: O) {
        *self.value.borrow_mut() = Some(value);
    }
}

pub fn value_of<O: Clone>(output: Output<O>, default: &O) -> O {
    match *output.value.borrow() {
        Some(ref value) => value.clone(),
        None => default.clone(),
    }
}
