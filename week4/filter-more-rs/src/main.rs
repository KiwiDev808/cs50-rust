use std::{
    env,
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom, Write},
    mem,
};

use filter::bmp::{BitMapFileHeader, BitMapInfoHeader};

use crate::filter::bmp::RGBTriple;

mod filter;

enum Filters {
    B,
    E,
    G,
    R,
}

impl Filters {
    fn from_flag(flag: &str) -> Option<Filters> {
        match flag {
            "-b" => Some(Filters::B),
            "-e" => Some(Filters::E),
            "-g" => Some(Filters::G),
            "-r" => Some(Filters::R),
            _ => None,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 3 {
        println!("Usage: ./filter [flag] infile outfile\n");
        return;
    }

    // Get filter flag and check validity
    let filter = match Filters::from_flag(&args[0]) {
        Some(value) => value,
        None => {
            println!("Invalid filter.");
            return;
        }
    };

    // Ensure only one filter
    if args.iter().skip(1).any(|x| Filters::from_flag(x).is_some()) {
        println!("Only one filter allowed.");
        return;
    }

    // Remember filenames
    let infile = &args[1];
    let outfile = &args[2];

    // Open input file
    let input = match File::open(&args[1]) {
        Ok(input_file) => input_file,
        Err(error) => {
            println!("{}", error);
            println!("Could not open {}.", infile);
            return;
        }
    };

    // Open output file
    let mut output = match File::create(&args[2]) {
        Ok(output_file) => output_file,
        Err(_) => {
            println!("Could not create {}.", outfile);
            return;
        }
    };

    let mut reader = BufReader::new(input);
    let file_reference = reader.by_ref();

    // // Read infile's BITMAPFILEHEADER
    let bf = match BitMapFileHeader::from_reader(file_reference) {
        Ok(result) => result,
        Err(_) => {
            println!("Error when reading bitmap file header");
            return;
        }
    };

    // // Read infile's BITMAPINFOHEADER
    let bi = match BitMapInfoHeader::from_reader(file_reference) {
        Ok(result) => result,
        Err(_) => {
            println!("Error when reading bitmap file header");
            return;
        }
    };

    // // Ensure infile is (likely) a 24-bit uncompressed BMP 4.0
    if bf.bf_type != 0x4d42
        || bf.bf_off_bits != 54
        || bi.bi_size != 40
        || bi.bi_bit_count != 24
        || bi.bi_compression != 0
    {
        println!("Unsupported file format.\n");
        return;
    }

    // Get image's dimensions
    let height = bi.bi_height.unsigned_abs() as usize;
    let width = bi.bi_width as usize;

    let image_size = width * height;
    let mut image: Vec<RGBTriple> = vec![
        RGBTriple {
            rgbt_blue: 0,
            rgbt_green: 0,
            rgbt_red: 0,
        };
        image_size
    ];

    // // Determine padding for scanlines
    let padding = ((4 - (width * mem::size_of::<RGBTriple>()) % 4) % 4) as i64;

    // Iterate over infile's scanlines
    for image_rows in image.chunks_mut(width) {
        // Read row into pixel array
        for image_row in image_rows.iter_mut() {
            *image_row = match RGBTriple::from_reader(file_reference) {
                Ok(value) => value,
                Err(_) => {
                    println!("Error when reading image");
                    return;
                }
            };
        }

        // Skip over padding
        file_reference
            .seek(SeekFrom::Current(padding))
            .expect("error when skipping padding");
    }

    // Filter image
    match filter {
        // Blur
        Filters::B => filter::blur(height, width, &mut image),

        // Edges
        Filters::E => filter::edges(height, width, &mut image),

        // Grayscale
        Filters::G => filter::grayscale(&mut image),

        // Reflect
        Filters::R => filter::reflect(width, &mut image),
    }

    // Write outfile's BITMAPFILEHEADER
    let bf_byte = match bf.to_byte() {
        Ok(value) => value,
        Err(_) => {
            println!("Error when converting file header to byte");
            return;
        }
    };

    output
        .write_all(&bf_byte)
        .expect("Error when creating file header on output");

    // // Write outfile's BITMAPINFOHEADER
    let bi_byte = match bi.to_byte() {
        Ok(value) => value,
        Err(_) => {
            println!("Error when converting info header to byte");
            return;
        }
    };

    output
        .write_all(&bi_byte)
        .expect("Error when creating info header on output");

    let padding_bytes = vec![0u8; padding as usize];
    for image_rows in image.chunks(width) {
        // Write new pixels to outfile
        for image_row in image_rows {
            let value = match image_row.to_byte() {
                Ok(value) => value,
                Err(_) => {
                    println!("Error when reading image");
                    return;
                }
            };

            output
                .write_all(&value)
                .expect("Error when writing image on output");
        }

        // Write padding at end of row
        output
            .write_all(&padding_bytes)
            .expect("Error when writing image on output");
    }
}
