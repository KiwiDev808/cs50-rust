use std::io::{self, Write};

fn get_user_input() -> std::string::String {
    let mut pyramid_height = String::new();
    print!("Height: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut pyramid_height)
        .expect("Failed to read line");

    pyramid_height
}

fn is_valid_input(value: &str) -> bool {
    let parse_result = value.trim().parse::<i32>();

    match parse_result {
        Ok(parsed_value) => {
            if !(1..9).contains(&parsed_value) {
                return false;
            }
            true
        }
        Err(_error) => false,
    }
}

fn print_pyramid(height: i32) {
    for i in 1..height + 1 {
        print_pyramid_row(height, i)
    }
}

fn print_pyramid_row(height: i32, brick_number: i32) {
    let empty_space_count = height - brick_number;

    print_spaces(empty_space_count);
    print_hashs(brick_number);
    print!("  ");
    print_hashs(brick_number);
    print_spaces(empty_space_count);
    println!();
}

fn print_spaces(count: i32) {
    for _i in 1..count + 1 {
        print!(" ")
    }
}

fn print_hashs(count: i32) {
    for _i in 1..count + 1 {
        print!("#")
    }
}

fn main() {
    println!("Mario Pyramids");

    println!("Please input the pyramid height.");
    println!("The number must be a integer between 1 and 8 inclusive");

    let mut pyramid_height_str;
    loop {
        pyramid_height_str = get_user_input();
        if is_valid_input(&pyramid_height_str) {
            break;
        }
    }

    let parsed_height = pyramid_height_str
        .trim()
        .parse::<i32>()
        .expect("Invalid Input");

    print_pyramid(parsed_height)
}
