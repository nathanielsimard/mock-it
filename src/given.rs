use crate::rule::Rule;
use std::sync::{Arc, Mutex};

pub struct Given<I, O> {
    input: I,
    rules: Arc<Mutex<Vec<Rule<I, O>>>>,
}

impl<I, O> Given<I, O> {
    pub(crate) fn new(input: I, rules: Arc<Mutex<Vec<Rule<I, O>>>>) -> Self {
        Given { input, rules }
    }

    /// Use the given return value when the mock is called with the specified
    /// input
    pub fn will_return(self, value: O) {
        self.rules
            .lock()
            .unwrap()
            .push(Rule::new(self.input, value));
    }
}

impl<I, O: Default> Given<I, O> {
    /// Use `Default::default` when the mock is called with the specified input
    pub fn will_return_default(self) {
        self.will_return(O::default())
    }
}

#[cfg(test)]
mod tests {
    use super::Given;
    use crate::rule::Rule;
    use std::sync::{Arc, Mutex};

    /// When `Given::will_return` is called with an output, the corresponding
    /// rule is added to the rules list
    #[test]
    fn add_rule_to_list() {
        let rules = Arc::new(Mutex::new(Vec::new()));
        let given = Given::new("hello", rules.clone());

        assert_eq!(rules.lock().unwrap().len(), 0);

        given.will_return(true);

        let rules = rules.lock().unwrap();
        assert_eq!(*rules, vec![Rule::new("hello", true)]);
    }

    /// When `Given::will_return_default` is called, a rule is made with the
    /// default of the output type
    #[test]
    fn add_default() {
        let rules = Arc::new(Mutex::new(Vec::new()));
        let given: Given<&str, bool> = Given::new("hello", rules.clone());

        assert_eq!(rules.lock().unwrap().len(), 0);

        given.will_return_default();

        let rules = rules.lock().unwrap();
        assert_eq!(*rules, vec![Rule::new("hello", false)]);
    }
}
