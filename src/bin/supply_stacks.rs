use std::{io::{BufReader, BufRead, Read}, fs::File};

fn main() {
    let mut input_reader: BufReader<File> = advent_of_code_2022::get_reader_from_arg();

    let cargo_setup: Vec<String> = input_reader.by_ref().lines()
        .take_while(|line| !line.as_ref().unwrap().is_empty())
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let stack_count: usize = cargo_setup.iter()
        .rev()
        .next().unwrap()
        .chars()
        .nth_back(1).unwrap()
        .to_digit(10).unwrap() as usize;
    
    let max_crates_per_stack: usize = stack_count * (cargo_setup.len() - 1);
    let mut cargo_part1: Vec<Vec<char>> = vec![Vec::with_capacity(max_crates_per_stack); stack_count];
    
    cargo_setup.iter()
        .rev()
        .skip(1)
        .for_each(|line | line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(index, char)| {
                if char != ' ' {
                    cargo_part1[index].push(char);
                }
            })
        );
    
    let mut cargo_part2: Vec<Vec<char>> = cargo_part1.clone();
    
    for line in input_reader.lines() {
        let line: String = line.unwrap();
        let segments: Vec<&str> = line.split(" ").collect();
        
        let move_count: usize = segments[1].parse().unwrap();
        let move_from_index: usize = segments[3].parse::<usize>().unwrap() - 1;
        let move_to_index: usize = segments[5].parse::<usize>().unwrap() - 1;
        
        for _ in 0..move_count {
            let moved_crate = cargo_part1[move_from_index].pop().unwrap();
            cargo_part1[move_to_index].push(moved_crate);
        }

        let index: usize= cargo_part2[move_from_index].len() - move_count;
        let mut moved_crates: Vec<char> = cargo_part2[move_from_index].drain(index..).collect();
        cargo_part2[move_to_index].append(&mut moved_crates);
    }
    
    let message_part1: String = cargo_part1.iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect();
    
    let message_part2: String = cargo_part2.iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect();

    println!("Part1: '{message_part1}', Part2: '{message_part2}'");
}