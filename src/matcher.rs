#[derive(Debug)]
pub enum Matcher<I> {
    Val(I),
    Any,
}

impl<I: PartialEq> PartialEq for Matcher<I> {
    fn eq(&self, other: &Matcher<I>) -> bool {
        use matcher::Matcher::*;

        match (self, other) {
            (&Val(ref a), &Val(ref b)) => a == b,
            _ => true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Matcher::*;

    #[test]
    fn test_eq() {
        let table = vec![
            ((Val(5), Val(6)), false),
            ((Val(5), Val(5)), true),
            ((Any, Val(5)), true),
            ((Val(5), Any), true),
            ((Any, Any), true),
        ];

        for (test_case, (matcher_1, matcher_2), expected) in table_test!(table) {
            let actual = matcher_1.eq(&matcher_2);

            test_case
                .given(&format!("{:?}, {:?}", matcher_1, matcher_2))
                .when("equal")
                .then(&format!("is {}", expected))
                .assert_eq(expected, actual);
        }
    }
}
