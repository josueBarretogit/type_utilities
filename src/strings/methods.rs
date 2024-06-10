use std::fmt::Write;

pub trait RemoveWhitespaces {
    /// As its name suggests removes all whitespaces of a string
    ///
    /// # Note to Implementors
    ///
    ///This implementation creates a new string so don't use this unless you are willing
    ///to pay the performance cost
    /// # Examples
    ///
    /// ```
    ///let case1 = String::from("first case ");
    /// assert_eq!("firstcase", case1.remove_whitespaces());
    /// let case2 = String::from(" this is another test with normal  ");
    /// assert_eq!("thisisanothertestwithnormal", case2.remove_whitespaces());
    ///```
    fn remove_whitespaces(&self) -> String;
}

pub trait CamelCase {
    fn to_camel_case(&self) -> String;
}

pub trait SnakeCase {
    fn to_snake_case(&self) -> String;
}

pub trait PascalCase {
    fn to_pascal_case(&self) -> String;
}

impl RemoveWhitespaces for String {
    fn remove_whitespaces(&self) -> String {
        let mut new_string_with_no_white_spaces = String::new();
        for character in self.chars() {
            if !character.is_whitespace() {
                new_string_with_no_white_spaces.push(character);
            }
        }
        new_string_with_no_white_spaces
    }
}

impl SnakeCase for String {
    fn to_snake_case(&self) -> String {
        let mut camel_cased = String::with_capacity(self.len());
        for word in self.split_whitespace() {
            camel_cased.push_str(format!("{word}_").as_str());
        }
        camel_cased.pop();
        camel_cased
    }
}

impl CamelCase for String {
    fn to_camel_case(&self) -> String {
        let mut words = self.split_whitespace();

        let firs_word = words.next().unwrap_or_default().to_lowercase();

        let result = words.fold(String::new(), |mut output, current| {
            let first_character = current.chars().nth(0).unwrap_or_default().to_uppercase();
            let rest_word = &current[1..current.len()];
            let _ = write!(output, "{first_character}{rest_word}");
            output
        });

        format!("{firs_word}{result}")
    }
}

impl PascalCase for String {
    fn to_pascal_case(&self) -> String {
        self.split_whitespace()
            .fold(String::new(), |mut output, current| {
                let first_character = current.chars().nth(0).unwrap_or_default().to_uppercase();
                let rest_word = &current[1..current.len()];
                let _ = write!(output, "{first_character}{rest_word}");
                output
            })
    }
}
