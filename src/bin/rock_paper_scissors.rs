use std::{io::{BufReader, BufRead, Lines, Read, Seek}, fs::File};

fn main() {
    let mut input_reader: BufReader<File> = advent_of_code_2022::get_reader_from_arg();

    let part1: isize = get_total_score(input_reader.by_ref().lines(), play_round_part1);
    input_reader.rewind().unwrap();
    let part2: isize = get_total_score(input_reader.by_ref().lines(), play_round_part2);
    
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn get_total_score(lines: Lines<&mut BufReader<File>>, play_round: fn(&str, &str) -> isize) -> isize {
    lines.map(|round| {
        let input: Vec<String> = round.unwrap().split(" ")
            .map(|ele| String::from(ele))
            .collect();
        play_round(&input[0][..], &input[1][..])
    }).sum()
}

fn get_round_score(my_choice: isize, opponent_choice: isize) -> isize {
    let mut res: isize = my_choice - opponent_choice;
    if res.abs() > 1 {
        res /= -2;
    }
    let win_score: isize = (res + 1) * 3;
    my_choice + win_score
}

fn play_round_part1(left_column: &str, right_column: &str) -> isize {
    let opponent_choice: isize = Choice::from_opponent_choice(left_column).unwrap();
    let my_choice: isize = Choice::from_my_choice(right_column).unwrap();
    get_round_score(my_choice, opponent_choice)
}

fn play_round_part2(left_column: &str, right_column: &str) -> isize {
    let opponent_choice: isize = Choice::from_opponent_choice(left_column).unwrap();
    let my_choice: isize = Choice::from_outcome(opponent_choice, right_column).unwrap();
    get_round_score(my_choice, opponent_choice)
}

struct Choice;

impl Choice {
    pub const ROCK: isize = 1;
    pub const PAPER: isize = 2;
    pub const SCISSORS: isize = 3;

    fn from_my_choice(coded_choice: &str) -> Option<isize> {
       match coded_choice {
           "X" => Some(Self::ROCK),
           "Y" => Some(Self::PAPER),
           "Z" => Some(Self::SCISSORS),
           _ => None
       }
    }
    
    fn from_opponent_choice(coded_choice: &str) -> Option<isize> {
        match coded_choice {
            "A" => Some(Self::ROCK),
            "B" => Some(Self::PAPER),
            "C" => Some(Self::SCISSORS),
            _ => None
        }
    }
    
    fn from_outcome(opponent_choice: isize, coded_outcome: &str) -> Option<isize> {
       match coded_outcome {
           "X" => Some(if opponent_choice == Self::ROCK { Self::SCISSORS } else { opponent_choice - 1}),
           "Y" => Some(opponent_choice),
           "Z" => Some(if opponent_choice == Self::SCISSORS { Self::ROCK } else { opponent_choice + 1}),
           _ => None
       } 
    }
} 