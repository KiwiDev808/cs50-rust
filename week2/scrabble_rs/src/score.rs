static LETTER_SCORE: [u32; 26] = [
    1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10,
];

static LETTER_INDEX: [&str; 26] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z",
];

pub fn compute_score(word: &str) -> u32 {
    let mut score = 0;
    for letter in word.chars() {
        let letter_score = get_letter_score(letter);
        score += letter_score;
    }

    score
}

fn get_letter_index(letter: char) -> Option<usize> {
    LETTER_INDEX
        .iter()
        .position(|&c| c == letter.to_uppercase().to_string())
}

fn get_letter_score(letter: char) -> u32 {
    let letter_index_option = get_letter_index(letter);

    match letter_index_option {
        Some(letter_index) => LETTER_SCORE[letter_index],
        None => 0,
    }
}
