use std::{io::{BufReader, BufRead, Seek, Read}, fs::File};

fn main() {
    let mut input_reader: BufReader<File> = advent_of_code_2022::get_reader_from_arg();

    let duplicate_assignments: usize = input_reader.by_ref().lines()
        .map(|line| ElvePair::from_line(line.unwrap()))
        .filter(|elve_pair| elve_pair.contain_each_other())
        .count();

    input_reader.rewind().unwrap();
    
    let overlapping_assignments: usize = input_reader.lines()
        .map(|line| ElvePair::from_line(line.unwrap()))
        .filter(|elve_pair| elve_pair.are_overlapping())
        .count();

    println!("Part 1: {duplicate_assignments}, Part 2: {overlapping_assignments}");
}

struct ElvePair {
    first_elve: AssignmentRange,
    second_elve: AssignmentRange
}

struct AssignmentRange {
    start: usize,
    end: usize
}

impl ElvePair {
    fn contain_each_other(&self) -> bool {
        &self.first_elve.start <= &self.second_elve.start && &self.first_elve.end >= &self.second_elve.end
        || &self.first_elve.start >= &self.second_elve.start && &self.first_elve.end <= &self.second_elve.end
    }
    
    fn are_overlapping(&self) -> bool {
        !(&self.first_elve.end < &self.second_elve.start
        || &self.first_elve.start > &self.second_elve.end)
    }

    fn from_line(line: String) -> Self {
        let assignments: Vec<usize> = line.split(",")
            .flat_map(|elve_assignment| elve_assignment.split("-"))
            .map(|range_borders_str| range_borders_str.parse().unwrap())
            .collect();
        ElvePair {
            first_elve: AssignmentRange { start: assignments[0], end: assignments[1] },
            second_elve: AssignmentRange { start: assignments[2], end: assignments[3] }
        }
    }
}