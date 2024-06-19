pub mod methods;

#[cfg(test)]
mod tests {
    #[test]
    fn try_split_at_works() {
        use crate::slice::methods::TrySplit;
        let case1 = [1, 2, 3, 4, 5, 6];

        // way out of bound, but won't panic
        let option1 = case1.try_split_at(20);

        assert_eq!(option1, None);

        let option2 = case1.try_split_at(3);

        assert_eq!(option2, Some(([1, 2, 3].as_ref(), [4, 5, 6].as_ref())));

        // In this case the left side is empty
        let option3 = case1.try_split_at(0);

        assert_eq!(option3, None);
    }
}
