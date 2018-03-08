pub enum Input<I> {
    Val(I),
    Any,
}

impl<I: PartialEq> PartialEq for Input<I> {
    fn eq(&self, other: &Input<I>) -> bool {
        use input::Input::*;

        match (self, other) {
            (&Val(ref a), &Val(ref b)) => a == b,
            _ => true,
        }
    }
}
