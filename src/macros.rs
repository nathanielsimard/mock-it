#[macro_export]
macro_rules! verify {
    ( $( $x:expr ),* ) => {
        {
            let mut result = true
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        verify!(validator)
    }
}
