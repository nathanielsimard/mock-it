use mock_it::mock_it;

#[mock_it]
trait ATrait {
    fn a_fn(&self, arg1: usize);
}

#[derive(Default)]
struct AStruct {
    mock: ATraitMock,
}

#[test]
fn mock_can_be_contructed_with_default() {
    let astruct = AStruct::default();
    let _mock = astruct.mock;
}
