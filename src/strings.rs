pub mod methods;

#[cfg(test)]
mod tests {
    use crate::strings::methods::*;

    #[test]
    fn string_remove_whitespaces() {
        let case1 = String::from("first case ");
        assert_eq!("firstcase", case1.remove_whitespaces());
        let case2 = String::from(" this is another test with normal  ");
        assert_eq!("thisisanothertestwithnormal", case2.remove_whitespaces());
    }

    #[test]
    fn to_camel_case() {
        let case1 = String::from("this is the first case");

        assert_eq!("this_is_the_first_case", case1.to_camel_case());

        let case2 = String::from(" this is the   second case2");

        assert_eq!("this_is_the_second_case2", case2.to_camel_case());

    }

}
