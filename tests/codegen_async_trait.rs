use async_trait::async_trait;
use mock_it::{any, eq, mock_it};

#[mock_it]
#[async_trait]
trait ATrait {
    async fn a_fn(&self, arg1: usize);
}

#[tokio::test]
#[should_panic]
async fn mock_no_when_should_panic() {
    let mock = ATraitMock::new();
    let _output = mock.a_fn(23).await;
}

#[tokio::test]
async fn mock_can_configure_will_return() {
    let mock = ATraitMock::new();
    mock.when_a_fn(eq(23)).will_return(());

    let output = mock.a_fn(23).await;

    assert_eq!(output, ());
}

#[tokio::test]
async fn mock_can_verify_called_with() {
    let mock = ATraitMock::new();
    mock.when_a_fn(any()).will_return(());

    let _output = mock.a_fn(42).await;

    assert!(mock.expect_a_fn(eq(42)).called());
}
