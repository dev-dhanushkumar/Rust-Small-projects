use std::{fs::File, io::Read};

use nom::{bytes::complete::{tag, take}, multi::many0, number::complete::{le_u16, le_u32}, IResult};
use serde_derive::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Header {
    magic_number: i32,
    version: u16,
    file_size: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct DataChunk {
    chunk_id: u16,
    chunck_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct FileFormat {
    header: Header,
    data_chunks: Vec<DataChunk>,
}



fn parser_header(input: &[u8]) -> IResult<&[u8], Header> {
    let (input, _) = tag("MAGIC")(input)?;
    let (input, version) = le_u16(input)?;
    let (input, file_size) = le_u32(input)?;

    Ok((
        input,
        Header {
            magic_number: 0x47414D,
            version,
            file_size,
        },
    ))
}

fn parse_data_chunk(input: &[u8]) -> IResult<&[u8], DataChunk> {
    let(input, chunk_id) = le_u16(input)?;
    let (input, chunk_size) = le_u16(input)?;
    let (input, chunk_data) = take(chunk_size)(input)?;

    Ok((
        input,
        DataChunk {
            chunk_id,
            chunck_data: chunk_data.to_vec(),
        },
    ))
}


fn parse_file_format(input: &[u8]) -> IResult<&[u8], FileFormat> {
    let (input, header) = parser_header(input)?;
    let (input, data_chunks) = many0(parse_data_chunk)(input)?;

    Ok((input, FileFormat {header, data_chunks}))
}

fn main()  -> std::io::Result<()>{
    println!("Welcome to file Parser!");

    let mut  file = File::open("D:/dev-dhanushkumar/Rust-Small-projects/file_format_parser/input.dat")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    match parse_file_format(&buffer) {
        Ok((_, file_format)) => {
            println!("File Format: {:?}", file_format);
        }
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
        }

    }

    Ok(())
}
