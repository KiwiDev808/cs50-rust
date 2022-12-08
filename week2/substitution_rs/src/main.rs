mod cypher;
mod user_input;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("[args] Should be passed only cypher as argument");
        process::exit(1);
    }

    let cypher_arr: Vec<char> = args[1].chars().collect();

    let validation_result = cypher::validate_cypher(&mut cypher_arr.clone());

    if validation_result.is_err() {
        let err_message = validation_result
            .err()
            .expect("Failed on parsing error message");
        println!("{}", err_message);
        process::exit(1)
    }

    let plaintext = user_input::get_string("plaintext: ");
    let cyphered_word = cypher::cypher_word(&cypher_arr, &plaintext);

    println!("ciphertext: {}", cyphered_word)
}
