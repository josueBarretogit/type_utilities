use std::fmt::Write;

pub trait RemoveWhitespaces {
    /// Creates a new string that contains no whitespaces
    ///
    /// # Note to Implementors
    ///
    ///This implementation creates a new string so don't use this unless you are willing
    ///to pay the performance cost
    /// # Examples
    ///
    /// ```no_run
    /// use crate::strings::methods::*;
    /// let case1 = String::from("first case ");
    /// assert_eq!("firstcase", case1.remove_whitespaces());
    /// let case2 = String::from(" this is another test with normal  ");
    /// assert_eq!("thisisanothertestwithnormal", case2.remove_whitespaces());
    ///```
    fn remove_whitespaces(&self) -> String;
}

pub trait ToCases {
    /// Creates a new string in a `PascalCase` format
    ///
    /// # Examples
    ///
    ///
    /// ```no_run
    /// use crate::strings::methods::*;
    /// let case1 = String::from("this is the first case");
    ///
    /// assert_eq!("thisIsTheFirstCase", case1.to_camel_case());
    ///
    /// let case2 = String::from(" this is the   second case2  ");
    ///
    /// assert_eq!("thisIsTheSecondCase2", case2.to_camel_case());
    ///
    /// let case3 = String::from(" This is a third case 3");
    ///
    /// assert_eq!("thisIsAThirdCase3", case3.to_camel_case());
    ///```
    fn to_pascal_case(&self) -> String;
    /// Creates a new string in a `snake_case` format
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crate::strings::methods::*;
    /// let case1 = String::from("this is the first case");
    ///
    /// assert_eq!("this_is_the_first_case", case1.to_snake_case());
    ///
    /// let case2 = String::from(" this is the   second case2  ");
    ///
    /// assert_eq!("this_is_the_second_case2", case2.to_snake_case());
    ///```
    fn to_snake_case(&self) -> String;

    /// Creates a new string in a `camelCased` format
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crate::strings::methods::*;
    /// let case1 = String::from("this is the first case");
    ///
    /// assert_eq!("this_is_the_first_case", case1.to_snake_case());
    ///
    /// let case2 = String::from(" this is the   second case2  ");
    ///
    /// assert_eq!("this_is_the_second_case2", case2.to_snake_case());
    ///```
    fn to_camel_case(&self) -> String;
}

pub trait IsCases {
    /// Check if `String` is has a `camelCasel` form
    ///
    /// # Note to Implementors
    ///
    /// The rules for a string to be `camelCase` are: 
    /// - The `String` must not be `lower_case`
    /// - The first character must not be uppercase
    /// - There must not be whitespaces
    /// - All characters must be `alphabetic`
    ///
    /// # Examples
    ///```rust
    ///
    /// use crate::strings::methods::IsCases;
    /// let case1 = String::from("thisIsCamelCase");
    ///
    /// assert!(case1.is_camel_case());
    /// let case2 = String::from("ThisisNotCamelCase");
    ///
    /// assert!(!case2.is_camel_case());
    /// let case3 = String::from("this isnot_camel_case");
    ///
    /// assert!(!case3.is_camel_case());
    /// let case4 = String::from("this#isnotCamelcase");
    ///
    /// assert!(!case4.is_camel_case());
    /// let case5 = String::from("thisIs%&notcamelcase");
    ///
    /// assert!(!case5.is_camel_case());
    /// let case6 = String::from("thisisnotcamelcase");
    ///
    /// assert!(!case6.is_camel_case());
    ///```
    fn is_camel_case(&self) -> bool;
    fn is_pascal_case(&self) -> bool;
    fn is_snake_case(&self) -> bool;
}

pub trait SelectNth {
    /// Obtains the first character of a string
    ///
    /// # Examples
    ///
    ///
    ///```no_run
    /// use crate::strings::methods::SelectNth;
    /// let case1 = String::from("obtain the first character");
    /// assert_eq!("o", case1.first());
    /// let case2 = String::from(" Obtain the first character");
    /// assert_eq!(" ", case2.first());
    ///```
    ///
    fn first(&self) -> String;
}

impl SelectNth for String {
    fn first(&self) -> String {
        self.chars().nth(0).unwrap_or_default().to_string()
    }
}

impl SelectNth for &str {
    fn first(&self) -> String {
        self.chars().nth(0).unwrap_or_default().to_string()
    }
}

impl RemoveWhitespaces for String {
    fn remove_whitespaces(&self) -> String {
        let mut new_string_with_no_white_spaces = String::with_capacity(self.capacity());
        for character in self.chars() {
            if !character.is_whitespace() {
                new_string_with_no_white_spaces.push(character);
            }
        }
        new_string_with_no_white_spaces
    }
}

impl ToCases for String {
    fn to_snake_case(&self) -> String {
        let mut camel_cased = String::with_capacity(self.capacity());
        for word in self.split_whitespace() {
            camel_cased.push_str(format!("{word}_").as_str());
        }
        camel_cased.pop();
        camel_cased
    }

    fn to_camel_case(&self) -> String {
        let mut words = self.split_whitespace();

        let firs_word = words.next().unwrap_or_default().to_lowercase();

        let result = words.fold(
            String::with_capacity(self.capacity()),
            |mut output, current| {
                let first_character = current.first().to_uppercase();
                let rest_word = &current[1..current.len()];
                let _ = write!(output, "{first_character}{rest_word}");
                output
            },
        );

        format!("{firs_word}{result}")
    }

    fn to_pascal_case(&self) -> String {
        self.split_whitespace().fold(
            String::with_capacity(self.capacity()),
            |mut output, current| {
                let first_character = current.first().to_uppercase();
                let rest_word = &current[1..current.len()];
                let _ = write!(output, "{first_character}{rest_word}");
                output
            },
        )
    }
}

impl IsCases for String {
    fn is_camel_case(&self) -> bool {
        if *self == self.to_lowercase() {
            return false;
        }
        if self.first() == self.first().to_uppercase() {
            return false;
        }

        if self.contains(|ch: char| !ch.is_alphabetic()) {
            return false;
        }

        true
    }

    fn is_snake_case(&self) -> bool {
        true
    }
    fn is_pascal_case(&self) -> bool {
        true
    }
}
