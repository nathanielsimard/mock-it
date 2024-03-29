mod simple {
    use mock_it::{any, eq, mock_it};

    #[mock_it]
    trait ATrait<T> {
        fn a_fn(&self, arg1: T);
    }

    #[test]
    #[should_panic]
    fn mock_no_when_should_panic() {
        let mock = ATraitMock::new();
        let _output = mock.a_fn(23);
    }

    #[test]
    fn mock_can_configure_will_return() {
        let mock = ATraitMock::new();
        mock.when_a_fn(eq(23)).will_return(());

        let output = mock.a_fn(23);

        assert_eq!(output, ());
    }

    #[test]
    fn mock_can_verify_called_with() {
        let mock = ATraitMock::new();
        mock.when_a_fn(any()).will_return(());

        let _output = mock.a_fn(42);

        assert!(mock.expect_a_fn(eq(42)).called());
    }
}

mod two_methods {
    use mock_it::{any, eq, mock_it};

    #[mock_it]
    trait ATrait<T> {
        fn a_fn(&self, arg1: T);
        fn another_fn(&self, arg1: &str);
    }

    #[test]
    #[should_panic]
    fn mock_no_when_should_panic() {
        let mock = ATraitMock::new();
        let _output = mock.a_fn(23);
    }

    #[test]
    fn mock_can_configure_will_return() {
        let mock = ATraitMock::new();
        mock.when_a_fn(eq(23)).will_return(());

        let output = mock.a_fn(23);

        assert_eq!(output, ());
    }

    #[test]
    fn mock_can_verify_called_with() {
        let mock = ATraitMock::new();
        mock.when_a_fn(any()).will_return(());

        let _output = mock.a_fn(42);

        assert!(mock.expect_a_fn(eq(42)).called());
    }
}

mod with_lifetime {
    use mock_it::{any, eq, mock_it};

    #[mock_it]
    trait ATrait<'a, T> {
        fn a_fn(&self, arg1: &'a T) -> &'a str;
    }

    #[test]
    #[should_panic]
    fn mock_no_when_should_panic() {
        let mock = ATraitMock::new();
        let _output = mock.a_fn(&23);
    }

    #[test]
    fn mock_can_configure_will_return() {
        let output_expected = "output";
        let mock = ATraitMock::new();
        mock.when_a_fn(eq(&23)).will_return(output_expected);

        let output = mock.a_fn(&23);

        assert_eq!(output, output_expected);
    }

    #[test]
    fn mock_can_verify_called_with() {
        let output = "output";
        let mock = ATraitMock::new();
        mock.when_a_fn(any()).will_return(output);

        let _ = mock.a_fn(&42);

        assert!(mock.expect_a_fn(eq(&42)).called());
    }
}

mod output_not_partial_eq {
    use mock_it::{any, eq, mock_it};

    #[derive(Clone, Debug)]
    pub struct Output;

    #[mock_it]
    trait ATrait<T, O> {
        fn a_fn(&self, arg1: T) -> O;
    }

    #[test]
    #[should_panic]
    fn mock_no_when_should_panic() {
        let mock = ATraitMock::new();
        let _output: Output = mock.a_fn(23);
    }

    #[test]
    fn mock_can_configure_will_return() {
        let mock = ATraitMock::new();
        mock.when_a_fn(eq(23)).will_return(Output {});

        let _output = mock.a_fn(23);
    }

    #[test]
    fn mock_can_verify_called_with() {
        let mock = ATraitMock::new();
        mock.when_a_fn(any()).will_return(Output {});

        let _output = mock.a_fn(42);

        assert!(mock.expect_a_fn(eq(42)).called());
    }
}
