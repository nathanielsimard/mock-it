use mock_it::mock_it;
use std::sync::Arc;

#[mock_it]
pub trait ATraitWithReferences {
    fn create(&self, str1: &str, str2: &String) -> String;
}

#[test]
fn mock_implements_trait() {
    let _trait_obj: &dyn ATraitWithReferences = &ATraitWithReferencesMock::new();
}

#[test]
fn mock_can_be_cloned() {
    let mock = ATraitWithReferencesMock::new();
    let _cloned_mock = mock.clone();
}

#[test]
fn mock_is_called() {
    let mock = ATraitWithReferencesMock::new();
    mock.create
        .given((Arc::from("allo"), Arc::from(String::from("allo"))))
        .will_return("llo".to_string())
}
