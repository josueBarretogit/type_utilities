pub mod methods;


#[cfg(test)]
mod test { 

    #[test]
    fn is_none_and_works() {
        use crate::option::methods::IsNone;
        let case1 = Some(1);
        assert!(!case1.is_none_and(|| true));
        let case2 : Option<i32> = None;
        assert!(case2.is_none_and(|| case1.is_some()));
    }


}
