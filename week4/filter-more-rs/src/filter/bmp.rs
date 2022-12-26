use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use std::io::{self, Error, Read};

type Byte = u8;
type DWord = u32;
type Long = i32;
type Word = u16;

pub struct BitMapFileHeader {
    pub bf_type: Word,
    bf_size: DWord,
    bf_reserved1: Word,
    bf_reserved2: Word,
    pub bf_off_bits: DWord,
}

pub struct BitMapInfoHeader {
    pub bi_size: DWord,
    pub bi_width: Long,
    pub bi_height: Long,
    bi_planes: Word,
    pub bi_bit_count: Word,
    pub bi_compression: DWord,
    bi_size_image: DWord,
    bi_xpels_per_meter: Long,
    bi_ypels_per_meter: Long,
    bi_clr_used: DWord,
    bi_clr_important: DWord,
}

#[derive(Clone)]
pub struct RGBTriple {
    pub rgbt_blue: Byte,
    pub rgbt_green: Byte,
    pub rgbt_red: Byte,
}

impl BitMapFileHeader {
    pub fn from_reader(rdr: &mut impl Read) -> io::Result<Self> {
        let bf_type = rdr.read_u16::<LittleEndian>()?;
        let bf_size = rdr.read_u32::<LittleEndian>()?;
        let bf_reserved1 = rdr.read_u16::<LittleEndian>()?;
        let bf_reserved2 = rdr.read_u16::<LittleEndian>()?;
        let bf_off_bits = rdr.read_u32::<LittleEndian>()?;

        Ok(BitMapFileHeader {
            bf_type,
            bf_size,
            bf_reserved1,
            bf_reserved2,
            bf_off_bits,
        })
    }
    pub fn to_byte(&self) -> Result<Vec<u8>, Error> {
        let mut byte_vec = Vec::new();
        byte_vec.write_u16::<LittleEndian>(self.bf_type)?;
        byte_vec.write_u32::<LittleEndian>(self.bf_size)?;
        byte_vec.write_u16::<LittleEndian>(self.bf_reserved1)?;
        byte_vec.write_u16::<LittleEndian>(self.bf_reserved2)?;
        byte_vec.write_u32::<LittleEndian>(self.bf_off_bits)?;
        Ok(byte_vec)
    }
}

impl BitMapInfoHeader {
    pub fn from_reader(rdr: &mut impl Read) -> io::Result<Self> {
        let bi_size = rdr.read_u32::<LittleEndian>()?;
        let bi_width = rdr.read_i32::<LittleEndian>()?;
        let bi_height = rdr.read_i32::<LittleEndian>()?;
        let bi_planes = rdr.read_u16::<LittleEndian>()?;
        let bi_bit_count = rdr.read_u16::<LittleEndian>()?;
        let bi_compression = rdr.read_u32::<LittleEndian>()?;
        let bi_size_image = rdr.read_u32::<LittleEndian>()?;
        let bi_xpels_per_meter = rdr.read_i32::<LittleEndian>()?;
        let bi_ypels_per_meter = rdr.read_i32::<LittleEndian>()?;
        let bi_clr_used = rdr.read_u32::<LittleEndian>()?;
        let bi_clr_important = rdr.read_u32::<LittleEndian>()?;

        Ok(BitMapInfoHeader {
            bi_size,
            bi_width,
            bi_height,
            bi_planes,
            bi_bit_count,
            bi_compression,
            bi_size_image,
            bi_xpels_per_meter,
            bi_ypels_per_meter,
            bi_clr_used,
            bi_clr_important,
        })
    }

    pub fn to_byte(&self) -> Result<Vec<u8>, Error> {
        let mut byte_vec = Vec::new();
        byte_vec.write_u32::<LittleEndian>(self.bi_size)?;
        byte_vec.write_i32::<LittleEndian>(self.bi_width)?;
        byte_vec.write_i32::<LittleEndian>(self.bi_height)?;
        byte_vec.write_u16::<LittleEndian>(self.bi_planes)?;
        byte_vec.write_u16::<LittleEndian>(self.bi_bit_count)?;
        byte_vec.write_u32::<LittleEndian>(self.bi_compression)?;
        byte_vec.write_u32::<LittleEndian>(self.bi_size_image)?;
        byte_vec.write_i32::<LittleEndian>(self.bi_xpels_per_meter)?;
        byte_vec.write_i32::<LittleEndian>(self.bi_ypels_per_meter)?;
        byte_vec.write_u32::<LittleEndian>(self.bi_clr_used)?;
        byte_vec.write_u32::<LittleEndian>(self.bi_clr_important)?;

        Ok(byte_vec)
    }
}

impl RGBTriple {
    pub fn from_reader(rdr: &mut impl Read) -> io::Result<Self> {
        let rgbt_blue = rdr.read_u8()?;
        let rgbt_green = rdr.read_u8()?;
        let rgbt_red = rdr.read_u8()?;

        Ok(RGBTriple {
            rgbt_blue,
            rgbt_green,
            rgbt_red,
        })
    }

    pub fn to_byte(&self) -> Result<Vec<u8>, Error> {
        let mut byte_vec = Vec::new();
        byte_vec.write_u8(self.rgbt_blue)?;
        byte_vec.write_u8(self.rgbt_green)?;
        byte_vec.write_u8(self.rgbt_red)?;
        Ok(byte_vec)
    }

    pub fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.rgbt_red = red;
        self.rgbt_green = green;
        self.rgbt_blue = blue;
    }

    pub fn get_average(&self) -> u8 {
        let sum = self.rgbt_blue as usize + self.rgbt_green as usize + self.rgbt_red as usize;
        (sum / 3) as u8
    }
}
