mod extractor;
mod sample_manipulator;

use std::{
    env,
    fs::File,
    io::{BufReader, Read, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check command-line arguments
    if args.len() != 4 {
        println!("Usage: ./volume input.wav output.wav factor");
        return;
    }

    // Open files and determine scaling factor
    let input = match File::open(&args[1]) {
        Ok(input_file) => input_file,
        Err(error) => {
            println!("{}", error);
            println!("Could not open input file.");
            return;
        }
    };

    let mut output = match File::create(&args[2]) {
        Ok(output_file) => output_file,
        Err(_) => {
            println!("Could not open output file.");
            return;
        }
    };

    let factor: f64 = match &args[3].parse::<f64>() {
        Ok(factor_value) => *factor_value,
        Err(_) => {
            println!("Invalid factor please put a number.");
            return;
        }
    };

    let mut reader = BufReader::new(input);
    let file_reference = reader.by_ref();

    let (original_header, wav_data) = extractor::extract_wav_data(file_reference);

    if wav_data.riff_header != "RIFF" {
        println!("Only RIFF files are supported");
        return;
    }

    output
        .write_all(&original_header)
        .expect("Error when writing output header");

    if wav_data.is_8_bit_sample() {
        for sample_byte in wav_data.data {
            let sample_with_factor =
                sample_manipulator::apply_factor_to_8_bit_sample(sample_byte, factor);
            let parsed_value = sample_manipulator::truncate_8_bit_sample(sample_with_factor);

            output
                .write_all(&[parsed_value])
                .expect("Error when writing output data");
        }
    } else if wav_data.is_16_bit_sample() {
        for sample_bytes in wav_data.data.chunks(2) {
            let sample_with_factor: isize = sample_manipulator::apply_factor_to_16_bit_sample(
                i16::from_le_bytes([sample_bytes[0], sample_bytes[1]]),
                factor,
            );

            let parsed_value =
                sample_manipulator::truncate_16_bit_sample(sample_with_factor).to_le_bytes();
            output
                .write_all(&parsed_value)
                .expect("Error when writing output data");
        }
    } else {
        println!("Unsuported bit sample")
    }
}
