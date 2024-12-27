mod letters {
    pub fn count_letters(text: &str) -> usize {
        text.chars().filter(|c| c.is_alphabetic()).count()
    }
}
mod numbers {
    pub fn count_numbers(text: &str) -> usize {
        text.chars().filter(|c| c.is_ascii_digit()).count()
    }
}
pub fn count_letters_and_numbers(text: &str) -> (usize, usize) {
    (letters::count_letters(text), numbers::count_numbers(text))
}
