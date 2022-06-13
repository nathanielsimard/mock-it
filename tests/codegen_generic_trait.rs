// TODO: Add Clone and PartialEq when implementing the Mock for Input Generics.
// TODO: Add Clone when implementing the Mock for Output Generics.
// TODO: Support generic in functions signatures.

mod simple {
    use mock_it::{any, eq, mock_it, verify};

    #[mock_it]
    trait ATrait<T>
    where
        T: Clone + PartialEq,
    {
        fn a_fn(&self, arg1: T);
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
}

mod two_methods {
    use mock_it::{any, eq, mock_it, verify};

    #[mock_it]
    trait ATrait<T>
    where
        T: Clone + PartialEq,
    {
        fn a_fn(&self, arg1: T);
        fn another_fn(&self, arg1: &str);
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
}

mod with_lifetime {
    use mock_it::{any, eq, mock_it, verify};

    #[mock_it]
    trait ATrait<'a, T>
    where
        T: Clone + PartialEq,
    {
        fn a_fn(&self, arg1: &'a T) -> &'a str;
    }

    #[test]
    #[should_panic]
    fn mock_no_given_should_panic() {
        let mock = ATraitMock::new();
        let _output = mock.a_fn(&23);
    }

    #[test]
    fn mock_can_configure_will_return() {
        let output_expected = "output";
        let mock = ATraitMock::new();
        mock.a_fn_with(eq(&23)).will_return(output_expected);

        let output = mock.a_fn(&23);

        assert_eq!(output, output_expected);
    }

    #[test]
    fn mock_can_verify_called_with() {
        let output = "output";
        let mock = ATraitMock::new();
        mock.a_fn_with(any()).will_return(output);

        let _ = mock.a_fn(&42);

        assert!(verify(mock.a_fn_was_called_with(eq(&42))));
    }
}
