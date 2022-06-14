use mock_it::{any, eq};

#[cfg_attr(test, mock_it::mock_it)]
pub trait ATrait {
    fn a_fn(&self, sized1: &usize, sized2: &String) -> String;
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
    let output = mock.a_fn(&23, &"Allo".to_string());
    assert_eq!(output, "my value".to_string());
}

#[test]
fn mock_can_configure_will_return() {
    let mock = ATraitMock::new();
    mock.when_a_fn(eq(&23), eq(&"Allo".to_string()))
        .will_return("my value".to_string());

    let output = mock.a_fn(&23, &"Allo".to_string());

    assert_eq!(output, "my value".to_string());
}

#[test]
fn mock_can_verify_called_with() {
    let mock = ATraitMock::new();
    mock.when_a_fn(any(), any())
        .will_return("default".to_string());

    let _output = mock.a_fn(&23, &"Allo".to_string());

    assert!(mock.expect_a_fn(eq(&23), eq(&"Allo".to_string())).called());
}
