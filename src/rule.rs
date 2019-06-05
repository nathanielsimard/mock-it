#[derive(Debug, PartialEq)]
pub struct Rule<I, O> {
    pub input: I,
    pub output: O,
}

impl<I, O> Rule<I, O> {
    pub fn new(input: I, output: O) -> Rule<I, O> {
        Rule { input, output }
    }
}
