use mock_it::{any, eq, mock_it};

#[mock_it]
trait ATrait {
    fn a_fn(&self, arg1: usize) -> String;
}

#[test]
fn mock_same_config_should_be_eq_simple() {
    let mock1 = ATraitMock::new();
    let mock2 = ATraitMock::new();

    mock1.when_a_fn(any()).will_return("same".to_string());
    mock2.when_a_fn(any()).will_return("same".to_string());

    assert_eq!(mock1, mock2);
}

#[test]
fn mock_same_config_should_be_eq_complex() {
    let mock1 = ATraitMock::new();
    let mock2 = ATraitMock::new();

    mock1.when_a_fn(eq(3)).will_return("same".to_string());
    mock2.when_a_fn(eq(3)).will_return("same".to_string());

    assert_eq!(mock1, mock2);
}

#[test]
fn mock_different_when_should_not_be_eq() {
    let mock1 = ATraitMock::new();
    let mock2 = ATraitMock::new();

    mock1.when_a_fn(eq(4)).will_return("same".to_string());
    mock2.when_a_fn(eq(3)).will_return("same".to_string());

    assert_ne!(mock1, mock2);
}

#[test]
fn mock_different_will_return_should_not_be_eq() {
    let mock1 = ATraitMock::new();
    let mock2 = ATraitMock::new();

    mock1.when_a_fn(eq(3)).will_return("same".to_string());
    mock2.when_a_fn(eq(3)).will_return("not same".to_string());

    assert_ne!(mock1, mock2);
}

#[test]
fn mock_same_config_different_called_should_not_be_eq() {
    let mock1 = ATraitMock::new();
    let mock2 = ATraitMock::new();

    mock1.when_a_fn(eq(3)).will_return("same".to_string());
    mock2.when_a_fn(eq(3)).will_return("same".to_string());
    let _ = mock1.calling_a_fn(eq(3));

    assert_ne!(mock1, mock2);
}
