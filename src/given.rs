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

    pub fn will_return(self, value: O) {
        self.rules
            .lock()
            .unwrap()
            .push(Rule::new(self.input, value));
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
}
