use std::{
    env,
    fs::File,
    io::{BufReader, Read, Write},
};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        println!("Usage: ./recover IMAGE");
        return;
    }

    // Remember filenames
    let infile = &args[0];

    // Open input file
    let input = match File::open(infile) {
        Ok(input_file) => input_file,
        Err(error) => {
            println!("{}", error);
            println!("Could not open {}.", infile);
            return;
        }
    };

    let mut reader = BufReader::new(input);
    let file_reference = reader.by_ref();

    let mut image_counter = 1;

    let mut output: Option<File> = None;

    loop {
        let mut buffer = Vec::new();
        file_reference.take(512).read_to_end(&mut buffer).unwrap();

        if buffer.is_empty() {
            break;
        }

        if is_jpeg_file(&buffer) {
            let filename = create_file_name(image_counter);
            output = match File::create(&filename) {
                Ok(output_file) => Some(output_file),
                Err(_) => {
                    println!("Could not create {}.", filename);
                    return;
                }
            };
            image_counter += 1;
            output
                .as_ref()
                .unwrap()
                .write_all(&buffer)
                .expect("Error writing to the buffer");
        } else if output.is_some() {
            output
                .as_ref()
                .unwrap()
                .write_all(&buffer)
                .expect("Error writing to the buffer");
        }

        buffer.clear();
    }
}

fn create_file_name(counter: i32) -> String {
    format!("images/{:03}.jpg", counter)
}

fn is_jpeg_file(buffer: &[u8]) -> bool {
    buffer[0] == 0xff && buffer[1] == 0xd8 && buffer[2] == 0xff && (buffer[3] & 0xf0) == 0xe0
}
