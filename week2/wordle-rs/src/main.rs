mod user_input;

use std::{
    env,
    fs::File,
    i32,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

use rand::Rng;

const LISTSIZE: i32 = 1000;

enum GuessAcurracy {
    Exact,
    Close,
    Wrong,
}

impl GuessAcurracy {
    fn as_int32(&self) -> i32 {
        match self {
            GuessAcurracy::Exact => 2,
            GuessAcurracy::Close => 1,
            GuessAcurracy::Wrong => 0,
        }
    }
}

// ANSI color codes for boxed in letters
enum Color {
    Green,
    Yellow,
    Red,
    Reset,
}

impl Color {
    fn as_str(&self) -> &'static str {
        match self {
            Color::Green => "\x1b[42;97m",
            Color::Yellow => "\x1b[43;97m",
            Color::Red => "\x1b[41;97m",
            Color::Reset => "\x1b[0;39m",
        }
    }
}

fn parse_wordsize(wordsize_str: &str) -> Result<i32, ParseIntError> {
    wordsize_str.parse::<i32>()
}

fn is_valid_wordsize(wordsize: i32) -> bool {
    let allowed_wordsizes = [5, 6, 7, 8];
    allowed_wordsizes.contains(&wordsize)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("[args] Please pass only wordsize value as argument");
        return;
    }

    let wordsize = match parse_wordsize(&args[1]) {
        Ok(value) => value,
        Err(_) => {
            println!("[args] wordsize value must be a integer");
            return;
        }
    };

    if !is_valid_wordsize(wordsize) {
        println!("[args] Please pass a valid wordsize with value 5, 6, 7, or 8");
        return;
    }

    // open correct file, each file has exactly LISTSIZE words
    let wl_filename = format!("{}.txt", 5);

    // load word file into an array of size LISTSIZE
    let mut options: Vec<String> = vec![];

    let input = File::open(wl_filename).expect("error");
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        if line.is_err() {
            continue;
        }
        let parsed_line = line.expect("Error");
        options.push(parsed_line)
    }

    // pseudorandomly select a word for this game
    let random_num = rand::thread_rng().gen_range(0..LISTSIZE);
    let choice = &options[random_num as usize];

    // allow one more guess than the length of the word
    let guesses: i32 = wordsize + 1;
    let mut won: bool = false;

    // print greeting, using ANSI color codes to demonstrate
    println!(
        r"{}This is WORDLE50{}",
        Color::Green.as_str(),
        Color::Reset.as_str()
    );
    println!(
        r"You have {} tries to guess the {}-letter word I'm thinking of\n",
        guesses, wordsize
    );

    // main game loop, one iteration for each guess
    for i in 0..guesses {
        // obtain user's guess
        let guess: String = get_guess(wordsize);

        // array to hold guess status, initially set to zero
        let mut status: Vec<i32> = vec![0; wordsize as usize];

        // Calculate score for the guess
        let score: i32 = check_word(guess.clone(), wordsize, &mut status, choice.to_string());

        print!(r"Guess {}: ", i + 1);

        // Print the guess
        print_word(guess, wordsize, status);

        // if they guessed it exactly right, set terminate loop
        if score == GuessAcurracy::Exact.as_int32() * wordsize {
            won = true;
            break;
        }
    }
    if won {
        println!("You win!");
    } else {
        println!(r"You target word was {choice}")
    }
}

fn is_valid_guess(guess: &str, wordsize: i32) -> bool {
    if guess.len() != wordsize as usize {
        return false;
    }
    true
}
fn get_guess(wordsize: i32) -> String {
    let mut guess: String;
    loop {
        guess = user_input::get_string("Input a 5-letter word: ");
        if is_valid_guess(&guess, wordsize) {
            break;
        }
    }

    guess
}

fn check_word(guess: String, _wordsize: i32, status: &mut [i32], choice: String) -> i32 {
    let choice_chars = choice.chars().into_iter().collect::<Vec<char>>();
    let mut score: i32 = 0;

    for (letter_index, letter) in guess.chars().enumerate() {
        if letter == choice_chars[letter_index] {
            status[letter_index] = GuessAcurracy::Exact.as_int32();
        } else if choice_chars.contains(&letter) {
            status[letter_index] = GuessAcurracy::Close.as_int32();
        } else {
            status[letter_index] = GuessAcurracy::Wrong.as_int32();
        }
        score += status[letter_index];
    }

    score
}

fn get_status_color(status_item: i32) -> String {
    if status_item == GuessAcurracy::Exact.as_int32() {
        return Color::Green.as_str().to_string();
    } else if status_item == GuessAcurracy::Close.as_int32() {
        return Color::Yellow.as_str().to_string();
    } else {
        return Color::Red.as_str().to_string();
    }
}

fn print_word(guess: String, _wordsize: i32, status: Vec<i32>) {
    let reset_color = Color::Reset.as_str().to_string();
    for (letter_index, letter) in guess.chars().enumerate() {
        let letter_color = get_status_color(status[letter_index]);
        print!(r"{letter_color}{letter}{reset_color}")
    }
    println!();
}
