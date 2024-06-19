pub mod methods;

#[cfg(test)]
mod test {

    #[test]
    fn toggle_method_works() {
        use crate::bool::methods::Toggle;

        let mut case1 = true;

        case1.toggle();

        assert!(!case1);

        case1.toggle();

        assert!(case1);
    }

    #[test]
    fn not_then_works() {
        use crate::bool::methods::Not;
        let case1 = false; 

        assert_eq!(case1.not_then(|| "negative"), Some("negative"));

        let case2 = true;

        assert_eq!(case2.not_then(|| 1), None);
    }

    #[test]
    fn not_then_some_works() {
        use crate::bool::methods::Not;
        let case1 = false;
        assert_eq!(case1.not_then_some("negative"), Some("negative"));
        let case2 = true;
        assert_eq!(case2.not_then_some(1), None);
    }
}
