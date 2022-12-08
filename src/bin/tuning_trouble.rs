use std::{io::{BufReader, Read}, fs::File};

fn main() {
    let mut input_reader: BufReader<File> = advent_of_code_2022::get_reader_from_arg();
    
    let mut line: String = Default::default();
    input_reader.read_to_string(&mut line).unwrap();
    
    let part1: usize = search_unique_sequence(&line, 4); 
    let part2: usize = search_unique_sequence(&line, 14); 

    println!("Part1: {part1}, Part2: {part2}");  
}

fn search_unique_sequence(line: &String, size: usize) -> usize {
    line.as_bytes()
        .windows(size)
        .position(|bytes| {
            bytes.iter().fold(Vec::<&u8>::with_capacity(size), |mut vec, byte| {
                if !vec.contains(&byte) {
                    vec.push(byte);
                }
                vec
            }).len() == size
        })
        .unwrap() + size
}