use std::sync::Arc;

use mock_it::mock_it;

#[mock_it]
pub trait TraitAsInput: std::fmt::Debug {
    fn allo(&self) -> String;
}

pub type Ref = Arc<dyn TraitAsInput>;

impl PartialEq for dyn TraitAsInput {
    fn eq(&self, other: &(dyn TraitAsInput)) -> bool {
        let message_self = format!("{:?}", self);
        let message_other = format!("{:?}", other);

        message_self == message_other
    }
}

#[mock_it]
trait ATrait {
    fn a_fn(&self, arg1: Ref);
}

#[test]
fn mock_should_impl_debug() {
    let mock = ATraitMock::new();
    format!("{:?}", mock);
}
