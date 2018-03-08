use std::rc::Rc;
use std::cell::RefCell;
use output::Output;
use output::OutputHandler;

pub struct Mock<I, O> {
    called_with: Rc<RefCell<Vec<(I, Output<O>)>>>,
}

impl<I, O> Clone for Mock<I, O> {
    fn clone(&self) -> Mock<I, O> {
        Mock {
            called_with: self.called_with.clone(),
        }
    }
}

impl<I: PartialEq, O> Mock<I, O> {
    pub fn new() -> Mock<I, O> {
        Mock {
            called_with: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn given(&self, input: I) -> Output<O> {
        let return_value = Rc::new(RefCell::new(None));
        self.called_with
            .borrow_mut()
            .push((input, Output::new(return_value.clone())));
        Output::new(return_value)
    }

    pub fn was_called_with(&self, input: I) -> bool {
        for value in &*self.called_with.borrow() {
            if value.0 == input {
                return true;
            }
        }
        return false;
    }
}

impl<I: PartialEq, O: Clone> Mock<I, O> {
    pub fn called_with(&self, input: I) -> OutputHandler<O> {
        for value in &*self.called_with.borrow() {
            if value.0 == input {
                return OutputHandler::new(Some(value.1.clone()));
            }
        }

        return OutputHandler::new(None);
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
                .return_value_with_default(String::new())
        }
    }

    #[test]
    fn it_work() {
        let mock = MyMock::new();
        mock.int_to_string
            .given(65)
            .will_return(String::from("Something"));
        let a_trait = Box::new(mock);

        assert_eq!("Something", a_trait.int_to_string(65));
        assert_ne!("Something", a_trait.int_to_string(64));
    }
}
