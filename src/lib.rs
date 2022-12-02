use std::env;
use std::fs::File;
use std::io::BufReader;

pub fn get_reader_from_arg() -> BufReader<File> {
    let last_arg: String = env::args().last().expect("No arg supplied.");
    if !last_arg.ends_with(".txt") {
        panic!("Input has to be a TXT file.");
    }
    let input_file: File = File::open(last_arg).expect("File could not be opened.");
    BufReader::new(input_file)
}