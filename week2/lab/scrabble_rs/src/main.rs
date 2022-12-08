mod score;
mod user_input;

use std::cmp::Ordering;

fn print_result(score1: u32, score2: u32) {
    match score1.cmp(&score2) {
        Ordering::Greater => println!("Player 1 wins!"),
        Ordering::Less => println!("Player 2 wins!"),
        Ordering::Equal => println!("Tie!"),
    }
}

fn main() {
    // Get input words from both players
    let word1 = user_input::get_string("Player 1: ");
    let word2 = user_input::get_string("Player 2: ");

    // Score both words
    let score1 = score::compute_score(&word1);
    let score2 = score::compute_score(&word2);

    print_result(score1, score2);
}
