# Implementing methods / utilities for rust primitive types

Please keep in mind that I am not a rust wizard and this methods may not be blazingly fast or may not follow the best practices

# Installation

Add this to your Cargo.toml

```toml

[dependencies]
# if you want methods for a specific type then add them as a feature
type_utilities = { version = "0.1.0", features = ["strings", "bool"] } 

# features currently available:
# strings, bool, vec

```

And bring the traits to scope like this: 

```rust

use type_utilities::strings::methods::*;

```

## Strings

so far, for strings `String` and `&str` I have implemented: 

`string.first()`

### Examples
```rust
        use crate::strings::methods::SelectNth;
        let case1 = String::from("obtain the first character");
        
        assert_eq!("o", case1.first());

        let case2 = String::from(" Obtain the first character");

        assert_eq!(" ", case2.first());
```

`string.remove_whitespaces()`

### Examples
```rust
        use crate::strings::methods::RemoveWhitespaces;
        let case1 = String::from("first case ");

        assert_eq!("firstcase", case1.remove_whitespaces());

        let case2 = String::from(" this is another test with normal  ");
        assert_eq!("thisisanothertestwithnormal", case2.remove_whitespaces());
```

`string.to_snake_case()`

### Examples
```rust 
        use crate::strings::methods::ToCases;
        let case1 = String::from("this is the first case");

        assert_eq!("this_is_the_first_case", case1.to_snake_case());

        let case2 = String::from(" this is the   second case2  ");

        assert_eq!("this_is_the_second_case2", case2.to_snake_case());

```

`string.to_camel_case()`

### Examples
```rust
        use crate::strings::methods::ToCases;
        let case1 = String::from("this is the first case");

        assert_eq!("thisIsTheFirstCase", case1.to_camel_case());

        let case2 = String::from(" this is the   second case2  ");

        assert_eq!("thisIsTheSecondCase2", case2.to_camel_case());

        let case3 = String::from(" This is a third case 3");

        assert_eq!("thisIsAThirdCase3", case3.to_camel_case());

```

`string.to_pascal_case()`

### Examples
```rust
        use crate::strings::methods::ToCases;
        let case1 = String::from("this is the first case");

        assert_eq!("ThisIsTheFirstCase", case1.to_pascal_case());

        let case2 = String::from(" this is the   second case2  ");

        assert_eq!("ThisIsTheSecondCase2", case2.to_pascal_case());

        let case3 = String::from(" This is a third case 3");

        assert_eq!("ThisIsAThirdCase3", case3.to_pascal_case());

```

## Bool

so far, for `bool` I have implemented:

`bool.toggle()`
```rust
    use crate::bool::methods::*;

    #[test]
    fn toggle_method_works() {
        let mut case1 = true;

        case1.toggle();

        assert!(!case1);

        case1.toggle();

        assert!(case1);
    }
```

## Vec

so far, for `Vec<T> where T: PartialEq + Clone ` I have implemented:

[Implemented methods](./src/vec.rs)
