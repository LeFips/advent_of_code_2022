use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    part1();
    part2();
}

fn part1() {
    let reader: BufReader<File> = advent_of_code_2022::get_reader_from_arg();
    
    let mut fattest_elve_calories: u32 = 0;
    let mut elve_calories: u32 = 0;

    for line in reader.lines() {
        if let Ok(calories) = line.unwrap().parse::<u32>() {
            elve_calories += calories;
        } else {
            if elve_calories > fattest_elve_calories {
                fattest_elve_calories = elve_calories;
            }
            elve_calories = 0;
        }
    }
    println!("{}", fattest_elve_calories);
}

fn part2() {
    let reader: BufReader<File> = advent_of_code_2022::get_reader_from_arg();

    let mut elves: Vec<u32> = reader.lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap_or_default())
        .fold(vec![0u32], |mut build_elves, calorie| {
            if calorie == 0 {
                build_elves.push(0);
            } else {
                let index = build_elves.len() - 1;
                build_elves[index] += calorie;
            }
            build_elves
        });
    elves.sort();
    let sum_top_3: u32 = elves.iter()
        .rev()
        .take(3)
        .sum();
    
    println!("{}", sum_top_3);
}