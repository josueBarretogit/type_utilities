//! This modules contains the trait that define new methods for `String` and &str
pub mod methods;

#[cfg(test)]
mod tests {

    #[test]
    fn string_remove_whitespaces() {
        use crate::strings::methods::RemoveWhitespaces;
        let case1 = String::from("first case ");

        assert_eq!("firstcase", case1.remove_whitespaces());
        let case2 = String::from(" this is another test with normal  ");
        assert_eq!("thisisanothertestwithnormal", case2.remove_whitespaces());
    }

    #[test]
    fn to_snake_case() {
        use crate::strings::methods::ToCases;
        let case1 = String::from("this is the first case");

        assert_eq!("this_is_the_first_case", case1.to_snake_case());

        let case2 = String::from(" this is the   second case2  ");

        assert_eq!("this_is_the_second_case2", case2.to_snake_case());
    }

    #[test]
    fn to_camel_case() {
        use crate::strings::methods::ToCases;
        let case1 = String::from("this is the first case");

        assert_eq!("thisIsTheFirstCase", case1.to_camel_case());

        let case2 = String::from(" this is the   second case2  ");

        assert_eq!("thisIsTheSecondCase2", case2.to_camel_case());

        let case3 = String::from(" This is a third case 3");

        assert_eq!("thisIsAThirdCase3", case3.to_camel_case());
    }

    #[test]
    fn to_pascal_case() {
        use crate::strings::methods::ToCases;
        let case1 = String::from("this is the first case");

        assert_eq!("ThisIsTheFirstCase", case1.to_pascal_case());

        let case2 = String::from(" this is the   second case2  ");

        assert_eq!("ThisIsTheSecondCase2", case2.to_pascal_case());

        let case3 = String::from(" This is a third case 3");

        assert_eq!("ThisIsAThirdCase3", case3.to_pascal_case());
    }

    #[test]
    fn is_camel_case_works() {
        use crate::strings::methods::IsCases;
        let case1 = String::from("thisIsCamelCase");

        assert!(case1.is_camel_case());

        let case2 = String::from("ThisisNotCamelCase");

        assert!(!case2.is_camel_case());

        let case3 = String::from("this isnot_camel_case");

        assert!(!case3.is_camel_case());

        let case4 = String::from("this#isnotCamelcase");

        assert!(!case4.is_camel_case());

        let case5 = String::from("thisIs%&notcamelcase");
        assert!(!case5.is_camel_case());

        let case6 = String::from("thisisnotcamelcase");
        assert!(!case6.is_camel_case());
    }

    #[test]
    fn is_pascal_case_works() {
        use crate::strings::methods::IsCases;
        let case1 = String::from("ThisIsPascalCase");

        assert!(case1.is_pascal_case());

        let case2 = String::from("thisisNotPascalCase");

        assert!(!case2.is_pascal_case());

        let case3 = String::from("this isnot_pascal_case");

        assert!(!case3.is_pascal_case());

        let case4 = String::from("this#isnotPascalcase");

        assert!(!case4.is_pascal_case());

        let case5 = String::from("thisIs%&notpascalcase");
        assert!(!case5.is_pascal_case());

        let case6 = String::from("thisisnotpascalcase");
        assert!(!case6.is_pascal_case());
    }

    #[test]
    fn first() {
        use crate::strings::methods::SelectNth;

        let case1 = String::from("obtain the first character");

        assert_eq!("o", case1.first());

        assert_eq!("Helo world".first(), "H");
    }
}
