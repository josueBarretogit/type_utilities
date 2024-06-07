pub trait RemoveWhitespaces {
    fn remove_whitespaces(&self) -> String;
}

pub trait CamelCase {
    fn to_camel_case(&self) -> String;
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

impl CamelCase for String {
    fn to_camel_case(&self) -> String {
        let mut camel_cased = String::new();
        for word in self.split_whitespace() {
            camel_cased.push_str(format!("{word}_").as_str());
        }
        camel_cased.pop();
        camel_cased
    }
}
