pub trait RemoveWhitespaces {
    fn remove_whitespaces(&self) -> String;
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
