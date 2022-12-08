use std::io::{self, Write};

fn get_user_input() -> std::string::String {
    let mut card_number = String::new();
    print!("Number: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut card_number)
        .expect("Failed to read line");

    return card_number.trim().to_string();
}

fn is_valid_input(value: &str) -> bool {
    let parse_result = value.trim().parse::<i64>();

    match parse_result {
        Ok(_ok) => true,
        Err(_error) => false,
    }
}

fn char_to_int(value: char) -> u32 {
    value.to_digit(10).expect("Char is not a integer")
}

fn is_even(number: usize) -> bool {
    number % 2 == 0
}

fn get_card_sum(card_number_str: &str) -> u32 {
    let mut even_digits_sum = 0;
    let mut odd_digits_sum = 0;

    for (i, digit) in card_number_str.chars().rev().enumerate() {
        let digit_index = i + 1;
        let num_digit = char_to_int(digit);

        if is_even(digit_index) {
            let digit_product = num_digit * 2;

            if digit_product >= 10 {
                even_digits_sum += digit_product / 10;
                even_digits_sum += digit_product - 10;
            } else {
                even_digits_sum += digit_product;
            }
        } else {
            odd_digits_sum += num_digit;
        }
    }
    even_digits_sum + odd_digits_sum
}

fn extract_card_data(card_number_str: &str) -> (usize, u32, &str) {
    let card_digits_length = card_number_str.chars().count();
    let digits_final_sum = get_card_sum(card_number_str);
    let card_two_first_digits = &card_number_str[..2];

    (card_digits_length, digits_final_sum, card_two_first_digits)
}

fn get_card_brand(card_length: usize, card_sum: u32, two_first_digits: &str) -> &'static str {
    let digit_length_outside_range = !(13..=16).contains(&card_length);
    let card_sum_not_ends_with_0 = !card_sum % 10 == 0;

    if digit_length_outside_range {
        return "INVALID";
    }
    if card_sum_not_ends_with_0 {
        return "INVALID";
    }

    if is_amex(card_length, two_first_digits) {
        "AMEX"
    } else if is_visa(card_length, two_first_digits) {
        "VISA"
    } else if is_mastercard(card_length, two_first_digits) {
        "MASTERCARD"
    } else {
        "INVALID"
    }
}

fn is_amex(card_length: usize, two_first_digits: &str) -> bool {
    let amex_start_numbers = ["34", "37"];

    card_length == 15 && amex_start_numbers.contains(&two_first_digits)
}

fn is_visa(card_length: usize, two_first_digits: &str) -> bool {
    let first_digit = &two_first_digits[..1];

    (card_length == 13 || card_length == 16) && first_digit == "4"
}

fn is_mastercard(card_length: usize, two_first_digits: &str) -> bool {
    let mastercard_start_numbers = ["51", "52", "53", "54", "55"];

    card_length == 16 && mastercard_start_numbers.contains(&two_first_digits)
}
fn main() {
    let mut card_number_str;
    loop {
        card_number_str = get_user_input();
        if is_valid_input(&card_number_str) {
            break;
        }
    }

    let (card_length, card_sum, two_first_digits) = extract_card_data(&card_number_str);

    let card_brand = get_card_brand(card_length, card_sum, two_first_digits);
    println!(" {}", card_brand);
}
