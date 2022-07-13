pub enum Matcher<I> {
    Val(I),
    Any,
}

impl<I: std::fmt::Debug> std::fmt::Debug for Matcher<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Val(val) => write!(f, "{:?}", val),
            Self::Any => write!(f, "Any"),
        }
    }
}

pub trait MockCompare {
    type Other;
    fn eq(&self, other: &Self::Other) -> bool;
}

impl<I: PartialEq> PartialEq for Matcher<I> {
    fn eq(&self, other: &Matcher<I>) -> bool {
        use crate::matcher::Matcher::*;

        match (self, other) {
            (&Val(ref a), &Val(ref b)) => a == b,
            _ => true,
        }
    }
}

pub fn eq<I>(input: I) -> Matcher<I> {
    Matcher::Val(input)
}

pub fn any<I>() -> Matcher<I> {
    Matcher::Any
}

#[cfg(test)]
mod test {
    use super::Matcher::*;
    use table_test::table_test;

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
