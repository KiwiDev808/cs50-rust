mod letter;

pub fn validate_cypher(cypher_arr: &mut Vec<char>) -> Result<(), String> {
    let cypher_size = cypher_arr.len();
    if cypher_size != 26 {
        return Err("[cypher] Cypher length must be 26 characters long".to_owned());
    }

    cypher_arr.sort_unstable();
    cypher_arr.dedup();

    let deduped_cypher_size = cypher_arr.len();
    if deduped_cypher_size != 26 {
        return Err("[cypher] Cypher characters must not be repeated".to_owned());
    }
    if !cypher_arr.iter().all(char::is_ascii_alphabetic) {
        return Err("[cypher] Cypher characters must be only characters from alphabet".to_owned());
    }

    Ok(())
}

pub fn cypher_word(cypher: &[char], word: &str) -> String {
    word.chars()
        .map(|x| match x {
            'A'..='Z' => {
                let letter_index = letter::get_letter_index(x).expect("Invalid letter");
                let cypher_letter = cypher[letter_index];
                cypher_letter.to_ascii_uppercase()
            }
            'a'..='z' => {
                let letter_index = letter::get_letter_index(x).expect("Invalid letter");
                let cypher_letter = cypher[letter_index];
                cypher_letter.to_ascii_lowercase()
            }
            _ => x,
        })
        .collect::<String>()
}
