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
