use return_value::ReturnValue;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Mock<I, R> {
    called_with: Option<(I, ReturnValueRef<R>)>,
}

#[derive(Clone)]
pub struct ReturnValueRef<R> {
    value: Rc<RefCell<ReturnValue<R>>>,
}

pub struct Builder<R> {
    value: Option<ReturnValueRef<R>>,
}

impl<R> Builder<R> {
    fn new(value: Option<ReturnValueRef<R>>) -> Builder<R> {
        Builder {
            value: value,
        }
    }
}

impl<R> Builder<R> {
    pub fn return_value_with_default(&self, default: R) -> R {
        if let Some(ref value) = self.value {
            return value.return_value(default);
        }
        default
    }
}
impl<R: Clone> Builder<R> {
    pub fn return_clonable_with_default(&self, default: R) -> R {
        if let Some(ref value) = self.value {
            return value.return_value_cloned(default);
        }
        default
    }
}

impl<R: Clone> ReturnValueRef<R> {
    pub fn will_return_clone_of(&self, value: R) {
        let mut return_value = self.value.borrow_mut();
        return_value.will_return_clone_of(value);
    }

    pub fn return_value_cloned(&self, default: R) -> R {
        let return_value = self.value.borrow();
        return_value.return_value_cloned(default)
    }
}

impl<R> ReturnValueRef<R> {
    fn new(value: Rc<RefCell<ReturnValue<R>>>) -> ReturnValueRef<R> {
        ReturnValueRef { value: value }
    }

    pub fn will_return(&self, factory: fn() -> R) {
        let mut return_value = self.value.borrow_mut();
        return_value.will_return(factory);
    }

    pub fn return_value(&self, default: R) -> R {
        let return_value = self.value.borrow();
        return_value.return_value(default)
    }
}

impl<I: PartialEq, R> Mock<I, R> {
    pub fn new() -> Mock<I, R> {
        Mock { called_with: None }
    }

    pub fn given(&mut self, input: I) -> ReturnValueRef<R> {
        let return_value = Rc::new(RefCell::new(ReturnValue::new()));
        self.called_with = Some((input, ReturnValueRef::new(return_value.clone())));
        ReturnValueRef::new(return_value)
    }
}

impl<I: PartialEq, R: Clone> Mock<I, R> {
    pub fn called_with(&self, input: I) -> Builder<R> {
        if let Some(ref value) = self.called_with {
            if value.0 == input {
                return Builder::new(Some(value.1.clone()));
            }
        }

        return Builder::new(None);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    trait ATrait {
        fn int_to_string(&self, input: i64) -> String;
    }

    struct MyMock {
        int_to_string: Mock<i64, String>,
    }

    impl MyMock {
        fn new() -> MyMock {
            MyMock {
                int_to_string: Mock::new(),
            }
        }
    }

    impl ATrait for MyMock {
        fn int_to_string(&self, input: i64) -> String {
            self.int_to_string
                .called_with(input)
                .return_clonable_with_default(String::new())
        }
    }

    #[test]
    fn it_work() {
        let mut mock = MyMock::new();
        mock.int_to_string
            .given(65)
            .will_return_clone_of(String::from("Something"));
        let a_trait = Box::new(mock);

        assert_eq!("Something", a_trait.int_to_string(65));
        assert_ne!("Something", a_trait.int_to_string(64));
    }
}
