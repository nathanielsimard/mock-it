use mock_it::mock_it;

#[mock_it]
trait ATrait {
    fn a_fn(&self, arg1: usize);
}

#[test]
#[should_panic(expected = "Mock \"ATraitMock.a_fn\" called with unexpected input: \"23\"")]
fn mock_no_when_should_panic() {
    let mock = ATraitMock::new();
    let _output = mock.a_fn(23);
}
