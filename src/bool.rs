pub mod methods;

#[cfg(test)]
mod test {
    use crate::bool::methods::*;

    #[test]
    fn toggle_method_works() {
        let mut case1 = true;

        case1.toggle();

        assert!(!case1);

        case1.toggle();

        assert!(case1);
    }
}
