mod extractor;

use std::{
    env,
    fs::File,
    io::{BufReader, Read, Write},
};

use extractor::WavData;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check command-line arguments
    if args.len() != 3 {
        println!("Usage: ./volume input.wav output.wav");
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

    let mut reader = BufReader::new(input);
    let file_reference = reader.by_ref();

    let (original_header, wav_data) = extractor::extract_wav_data(file_reference);

    if wav_data.riff_header != "RIFF" {
        println!("Only RIFF files are supported");
        return;
    }

    if !is_wave_file(&wav_data) {
        println!("Input is not a WAV file.");
        return;
    }

    output
        .write_all(&original_header)
        .expect("Error when writing output header");

    if wav_data.is_16_bit_sample() {
        let block_size = wav_data.get_block_size() as usize;
        for sample_bytes in wav_data.data.chunks(block_size).rev() {
            output
                .write_all(sample_bytes)
                .expect("Error when writing output data");
        }
    } else {
        println!("Unsuported bit sample")
    }
}

fn is_wave_file(wav_header: &WavData) -> bool {
    wav_header.format == "WAVE"
}
