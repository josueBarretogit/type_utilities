pub mod methods;

#[cfg(test)]
mod tests {

    #[test]
    fn filter_works() {
        use crate::result::methods::Filter;
        let case1 = "2".parse::<i32>();
        let case1 = case1.filter(|nu| *nu > 1);
        assert_eq!(case1, Ok(&2));

        let case2 = "err".parse::<i32>();
        let case2 = case2.filter(|nu| *nu < 10);

        match case2 {
            Ok(_) => {},
            Err(e) => {
                assert!(e.is_some());
            },
        };

        let case3 = "10".parse::<i32>();
        let case3 = case3.filter(|nu| *nu > 20);

        match case3 {
            Ok(_) => {},
            Err(e) => assert!(e.is_none())
        };

    }

} 
