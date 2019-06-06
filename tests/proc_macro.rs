use mock_it::{mock_it, verify};

#[mock_it]
trait MyTrait {
    fn do_something(&self, arg1: String, arg2: usize) -> bool;
    fn no_return(&self, arg1: usize);
    fn no_args(&self) -> String;
}

/// The mock has a `new` method which initializes each mock with a default value
/// using `Default`
#[test]
fn mock_initializes_with_default() {
    let mock = MyTraitMock::new();
    let args = ("".to_string(), 0);
    let output = mock.do_something.called(args);

    assert_eq!(output, bool::default());
}

/// The mock implements the trait
#[test]
fn mock_implements_trait() {
    let _trait_obj: &MyTrait = &MyTraitMock::new();
}

/// The mock can be clone, but will keep its inner state
/// making it easy to share with other struct without
/// specific lifetime
#[test]
fn mock_can_be_cloned() {
    let mock = MyTraitMock::new();
    let _cloned_mock = mock.clone();
}

/// The mock records calls to the trait methods
#[test]
fn mock_is_called() {
    let mock = MyTraitMock::new();

    mock.do_something("test".to_string(), 42);

    assert!(verify(
        mock.do_something.was_called_with(("test".to_string(), 42))
    ));
}

/// The mock uses the provided inputs if given. Generally, this test makes sure
/// that the mock behaves the same as handwritten mocks.
#[test]
fn mock_respects_given() {
    let mock = MyTraitMock::new();

    mock.do_something
        .given(("test".to_string(), 42))
        .will_return(true);

    assert_ne!(mock.do_something("test 2".to_string(), 42), true);
    assert_eq!(mock.do_something("test".to_string(), 42), true);
}
