use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    part1();
    part2();
}

fn part1() {
    let input_reader: BufReader<File> = advent_of_code_2022::get_reader_from_arg();

    let priorities: usize = input_reader.lines()
        .map(|rucksack| {
            let rucksack: String = rucksack.unwrap();
            let comp_size: usize = rucksack.len() / 2;
            let first_compartment: &str = &rucksack[..comp_size];
            let second_compartment: &str = &rucksack[comp_size..];
            
            let duplicate: char = first_compartment.chars()
                .find(|char| second_compartment.chars()
                    .any(|char2| *char == char2))
                .unwrap();
            let ascii_value: usize = duplicate as usize;
            if ascii_value < 95 { ascii_value - 38 } else { ascii_value - 96 }
        })
        .sum();
        
    println!("{priorities}");
}

fn part2() {
    let mut input_reader: BufReader<File> = advent_of_code_2022::get_reader_from_arg();
    let mut buf: String = String::new();
    let mut group: [String; 3] = Default::default();
    let mut total: usize = 0;

    'outer: loop {
        for i in 0..3 {
            let read_bytes: usize = input_reader.read_line(&mut buf).unwrap();
            if read_bytes == 0 { break 'outer; }
            group[i] = buf.clone();
            buf.clear();
        }
        let duplicate = group[0].chars()
            .find(|char| group[1].chars()
                .any(|char2| *char == char2 && group[2].chars()
                    .any(|char3| char2 == char3 )))
            .unwrap();
        let ascii_value = duplicate as usize;
        total += if ascii_value < 95 { ascii_value - 38 } else { ascii_value - 96 };
    }
    
    println!("{total}");
}
