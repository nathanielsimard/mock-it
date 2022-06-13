use mock_it::{any, eq, mock_it, verify};

#[mock_it]
trait ATrait {
    fn a_fn(&self, arg1: usize);
}

#[test]
#[should_panic]
fn mock_no_given_should_panic() {
    let mock = ATraitMock::new();
    let _output = mock.a_fn(23);
}

#[test]
fn mock_can_configure_will_return() {
    let mock = ATraitMock::new();
    mock.a_fn_with(eq(23)).will_return(());

    let output = mock.a_fn(23);

    assert_eq!(output, ());
}

#[test]
fn mock_can_verify_called_with() {
    let mock = ATraitMock::new();
    mock.a_fn_with(any()).will_return(());

    let _output = mock.a_fn(42);

    assert!(verify(mock.a_fn_was_called_with(eq(42))));
}
