//! This modules contains the trait that define new methods for `Vec<T>` and `[T]`
pub mod methods;

#[cfg(test)]
mod tests {

    use crate::vec::methods::*;

    #[derive(Debug, Default, Clone, PartialEq)]
    struct Example {
        name: String,
        id: i32,
        other_property: Vec<u8>,
    }

    #[test]
    fn replace_method_works() {
        let case1 = vec![1, 3, 4, 5];

        let replaced_vec = case1.replace(|item| *item == 3, 4);
    

        assert_eq!(vec![1, 4, 4, 5], replaced_vec);

        let replaced_vec2 = case1.replace(|item| *item == 5, 10);

        assert_eq!(vec![1, 3, 4, 10], replaced_vec2);

        let case2: Vec<Example> = vec![
            Example {
                name: "josh".to_string(),
                id: 1,
                other_property: vec![],
            },
            Example {
                name: "ua".to_string(),
                id: 2,
                other_property: vec![],
            },
        ];

        let replaced_struct_vec = case2.replace(
            |example| example.name.as_str() == "josh",
            Example {
                name: "replaced".to_string(),
                id: 3,
                other_property: vec![1, 2],
            },
        );

        assert_eq!(
            vec![
                Example {
                    name: "replaced".to_string(),
                    id: 3,
                    other_property: vec![1, 2],
                },
                Example {
                    name: "ua".to_string(),
                    id: 2,
                    other_property: vec![],
                },
            ],
            replaced_struct_vec
        );
    }

    #[test]
    fn replace_mut_method_works() {
        let mut case1 = vec![1, 2, 3, 4, 5];

        case1.replace_mut(|item| *item == 2, 3);

        assert_eq!(vec![1, 3, 3, 4, 5], case1);

        let mut case2_struct_case = vec![
            Example {
                name: "name1".to_string(),
                id: 1,
                other_property: vec![1, 2, 3],
            },
            Example {
                name: "other".to_string(),
                id: 2,
                other_property: vec![3, 2, 4],
            },
        ];

        case2_struct_case.replace_mut(
            |item| item.name.as_str() == "other",
            Example {
                name: "other changed".to_string(),
                id: 2,
                other_property: vec![3, 2, 4],
            },
        );

        let case_to_compare_struct = vec![
            Example {
                name: "name1".to_string(),
                id: 1,
                other_property: vec![1, 2, 3],
            },
            Example {
                name: "other changed".to_string(),
                id: 2,
                other_property: vec![3, 2, 4],
            },
        ];

        assert_eq!(case_to_compare_struct, case2_struct_case);
    }

    #[test]
    fn try_split_at_works() {
        use crate::vec::methods::TrySplit;
        let case1 = vec![1, 2, 3, 4, 5, 6];

        // way out of bound, but won't panic
        let option1 = case1.try_split_at(20);

        assert_eq!(option1, None);

        let option2 = case1.try_split_at(3);

        assert_eq!(option2, Some(([1, 2, 3].as_ref(), [4, 5, 6].as_ref())));

        // In this case the left side is empty
        let option3 = case1.try_split_at(0);

        assert_eq!(option3, None);

        let case2 = [1, 2, 3, 4, 5, 6];

        let option4 = case2.try_split_at(3);

        assert_eq!(option4, Some(([1, 2, 3].as_ref(), [4, 5, 6].as_ref())));
    }
}
