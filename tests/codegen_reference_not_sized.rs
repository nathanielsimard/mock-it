use mock_it::{any, eq, mock_it};

#[mock_it]
pub trait ATrait {
    fn a_fn(&self, sized1: &str, sized2: &str) -> String;
}

#[test]
fn mock_implements_trait() {
    let _trait_obj: &dyn ATrait = &ATraitMock::new();
}

#[test]
fn mock_can_be_cloned() {
    let mock = ATraitMock::new();
    let _cloned_mock = mock.clone();
}

#[test]
#[should_panic]
fn mock_no_given_should_panic() {
    let mock = ATraitMock::new();
    let output = mock.a_fn("str1", "str2");
    assert_eq!(output, "my value".to_string());
}

#[test]
fn mock_can_configure_will_return() {
    let mock = ATraitMock::new();
    mock.when_a_fn(eq("str1"), eq("str2"))
        .will_return("my value".to_string());

    let output = mock.a_fn("str1", "str2");

    assert_eq!(output, "my value".to_string());
}

#[test]
fn mock_can_verify_called_with() {
    let mock = ATraitMock::new();
    mock.when_a_fn(any(), any())
        .will_return("default".to_string());

    let _output = mock.a_fn("str1", "str2");

    assert!(mock.expect_a_fn(eq("str1"), eq("str2")).called());
}
