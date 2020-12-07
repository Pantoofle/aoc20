use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::result::Result;

pub fn get_input() -> Result<String, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() == 2 && args[1] != "-" {
        Some(args[1].clone())
    } else {
        None
    };

    let mut reader: Box<dyn io::Read> = match &filename {
        Some(filename) => Box::new(io::BufReader::new(File::open(&filename)?)),
        None => Box::new(io::stdin()),
    };

    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    Ok(input)
}
