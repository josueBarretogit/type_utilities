pub mod string_methods;

#[cfg(test)]
mod tests {
    use crate::strings::string_methods::*;

    #[test]
    fn string_remove_whitespaces() {
        let case1 = String::from("first case ");
        assert_eq!("firstcase", case1.remove_whitespaces());
        let case2 = String::from(" this is another test with normal  ");
        assert_eq!("thisisanothertestwithnormal", case2.remove_whitespaces());
    }

}
