use std::{
    fs::File,
    io::{BufReader, Read},
};

pub struct WavData {
    pub riff_header: String,
    pub num_channels: u16,
    pub bits_per_sample: u16,
    pub data: Vec<u8>,
}

impl WavData {
    pub fn is_8_bit_sample(&self) -> bool {
        self.bits_per_sample == 8
    }

    pub fn is_16_bit_sample(&self) -> bool {
        self.bits_per_sample == 16
    }
}

pub type TwoBytes = [u8; 2];
pub type FourBytes = [u8; 4];

fn byte_to_str(byte_arr: &[u8]) -> String {
    String::from_utf8_lossy(byte_arr).to_string()
}

fn extract_chunk_descriptor(binary_chunk_descriptor: &[u8]) -> (String, u32, String) {
    let chunk_id_bytes = &binary_chunk_descriptor[0..4];
    let chunk_size_bytes: FourBytes = binary_chunk_descriptor[4..8]
        .try_into()
        .expect("Error when reading chunk descriptor");
    let format_bytes: FourBytes = binary_chunk_descriptor[8..12]
        .try_into()
        .expect("Error when reading chunk descriptor");

    let chunk_id = byte_to_str(chunk_id_bytes);
    let chunk_size = u32::from_le_bytes(chunk_size_bytes);
    let chunk_format = byte_to_str(&format_bytes);

    (chunk_id, chunk_size, chunk_format)
}

fn extract_sub_chunk_data(binary_sub_chunk_data: &[u8]) -> (String, u32) {
    let sub_chunk_id_bytes: FourBytes = binary_sub_chunk_data[0..4]
        .try_into()
        .expect("Error when reading sub chunk data");
    let sub_chunk_size_bytes: FourBytes = binary_sub_chunk_data[4..8]
        .try_into()
        .expect("Error when reading sub chunk data");

    let sub_chunk_id = byte_to_str(&sub_chunk_id_bytes);
    let sub_chunk_size = u32::from_le_bytes(sub_chunk_size_bytes);

    (sub_chunk_id, sub_chunk_size)
}

fn extract_sub_chunk_format(
    binary_sub_chunk_format: &[u8],
) -> (String, u32, u16, u16, u32, u32, u16, u16) {
    let sub_chunk_id_bytes: FourBytes = binary_sub_chunk_format[0..4]
        .try_into()
        .expect("Error when reading sub chunk format");
    let sub_chunk_size_bytes: FourBytes = binary_sub_chunk_format[4..8]
        .try_into()
        .expect("Error when reading sub chunk format");
    let audio_format_bytes: TwoBytes = binary_sub_chunk_format[8..10]
        .try_into()
        .expect("Error when reading sub chunk format");
    let num_channels_bytes: TwoBytes = binary_sub_chunk_format[10..12]
        .try_into()
        .expect("Error when reading sub chunk format");
    let sample_rate_bytes: FourBytes = binary_sub_chunk_format[12..16]
        .try_into()
        .expect("Error when reading sub chunk format");
    let byte_rate_bytes: FourBytes = binary_sub_chunk_format[16..20]
        .try_into()
        .expect("Error when reading sub chunk format");
    let block_align_bytes: TwoBytes = binary_sub_chunk_format[20..22]
        .try_into()
        .expect("Error when reading sub chunk format");
    let bits_per_sample_bytes: TwoBytes = binary_sub_chunk_format[22..24]
        .try_into()
        .expect("Error when reading sub chunk format");

    let sub_chunk_id = byte_to_str(&sub_chunk_id_bytes);
    let sub_chunk_size = u32::from_le_bytes(sub_chunk_size_bytes);
    let sub_chunk_format = u16::from_le_bytes(audio_format_bytes);
    let num_channels = u16::from_le_bytes(num_channels_bytes);
    let sample_rate = u32::from_le_bytes(sample_rate_bytes);
    let byte_rate = u32::from_le_bytes(byte_rate_bytes);
    let block_align = u16::from_le_bytes(block_align_bytes);
    let bits_per_sample = u16::from_le_bytes(bits_per_sample_bytes);

    (
        sub_chunk_id,
        sub_chunk_size,
        sub_chunk_format,
        num_channels,
        sample_rate,
        byte_rate,
        block_align,
        bits_per_sample,
    )
}

pub fn extract_wav_data(buffer: &mut BufReader<File>) -> (Vec<u8>, WavData) {
    let mut file_data = Vec::new();
    buffer.read_to_end(&mut file_data).expect("error reading");

    let binary_chunk_descriptor = &file_data[0..12];
    let binary_sub_chunk_format = &file_data[12..36];
    let binary_sub_chunk_2 = &file_data[36..44];
    let original_header = &file_data[0..44];
    let binary_chunk_data = file_data[44..].to_vec();

    let (riff_header, chunk_size, chunk_format) = extract_chunk_descriptor(binary_chunk_descriptor);
    let (
        _sub_chunk_id,
        _sub_chunk_size,
        sub_chunk_format,
        num_channels,
        sample_rate,
        _byte_rate,
        _block_align,
        bits_per_sample,
    ) = extract_sub_chunk_format(binary_sub_chunk_format);

    let (_sub_chunk2_id, sub_chunk2_size) = extract_sub_chunk_data(binary_sub_chunk_2);

    println!(
        "riff_header {}, chunk_size {}, chunk_format {}",
        riff_header, chunk_size, chunk_format
    );
    println!(
        "Audio format {}, num_channels: {}, sample_rate: {}, bits_per_sample {}",
        sub_chunk_format, num_channels, sample_rate, bits_per_sample
    );

    println!("file data byte size {}", sub_chunk2_size);

    (
        original_header.to_vec(),
        WavData {
            riff_header,
            num_channels,
            bits_per_sample,
            data: binary_chunk_data,
        },
    )
}
