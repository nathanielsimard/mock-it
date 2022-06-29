use crate::rule::Rule;
use std::sync::{Arc, Mutex};

pub struct When<I, O> {
    input: I,
    rules: Arc<Mutex<Vec<Rule<I, O>>>>,
}

impl<I: PartialEq, O> When<I, O> {
    pub(crate) fn new(input: I, rules: Arc<Mutex<Vec<Rule<I, O>>>>) -> Self {
        When { input, rules }
    }

    /// Use the when return value when the mock is called with the specified
    /// input
    pub fn will_return(self, value: O) {
        let mut rules_locked = self.rules.lock().unwrap();
        let when_value = rules_locked
            .iter()
            .enumerate()
            .find(|(_i, value)| value.input == self.input);

        let rule = Rule::new(self.input, value);
        match when_value {
            Some((index, _value)) => {
                let _old_rule = std::mem::replace(&mut rules_locked[index], rule);
                ()
            }
            None => rules_locked.push(rule),
        }
    }
}

impl<I: PartialEq, O: Default> When<I, O> {
    /// Use `Default::default` when the mock is called with the specified input
    pub fn will_return_default(self) {
        self.will_return(O::default())
    }
}

#[cfg(test)]
mod tests {
    use super::When;
    use crate::rule::Rule;
    use std::sync::{Arc, Mutex};

    /// When `Given::will_return` is called with an output, the corresponding
    /// rule is added to the rules list
    #[test]
    fn add_rule_to_list() {
        let rules = Arc::new(Mutex::new(Vec::new()));
        let when = When::new("hello", rules.clone());

        when.will_return(true);

        let rules = rules.lock().unwrap();
        assert_eq!(*rules, vec![Rule::new("hello", true)]);
    }

    #[test]
    fn when_input_already_match_another_rule_replace_old_rule() {
        let rules = Arc::new(Mutex::new(Vec::new()));
        let when = When::new("sameinput", rules.clone());

        let assert_rule = |input, output| {
            let rules_locked = rules.lock().unwrap();
            let rule = rules_locked.get(0).unwrap();
            assert_eq!(rules_locked.len(), 1, "Rules should have only one rule.");
            assert_eq!(rule.input, input);
            assert_eq!(rule.output, output);
        };

        when.will_return("rule1");
        assert_rule("sameinput", "rule1");

        let when = When::new("sameinput", rules.clone());
        when.will_return("rule2");
        assert_rule("sameinput", "rule2");
    }

    /// When `Given::will_return_default` is called, a rule is made with the
    /// default of the output type
    #[test]
    fn add_default() {
        let rules = Arc::new(Mutex::new(Vec::new()));
        let when: When<&str, bool> = When::new("hello", rules.clone());

        when.will_return_default();

        let rules = rules.lock().unwrap();
        assert_eq!(*rules, vec![Rule::new("hello", false)]);
    }
}
