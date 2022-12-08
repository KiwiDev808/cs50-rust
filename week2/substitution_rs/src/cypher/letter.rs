static LETTER_INDEX: [&str; 26] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z",
];

pub fn get_letter_index(letter: char) -> Option<usize> {
    LETTER_INDEX
        .iter()
        .position(|&c| c == letter.to_uppercase().to_string())
}
