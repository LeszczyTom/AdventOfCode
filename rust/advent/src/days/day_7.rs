use crate::*;
use std::time::Instant;

const DAY: u8 = 7;

#[derive(Debug, Clone)]
struct Crab {
    horizontal_position: u32,
    fuel_used: u64,
}

impl Crab {
    fn new(h_pos: u32) -> Self {
        Self {
            horizontal_position: h_pos,
            fuel_used: 0,
        }
    }

    fn move_to(&mut self, h_pos: u32) {
        self.fuel_used += if self.horizontal_position > h_pos {(self.horizontal_position - h_pos) as u64} else {(h_pos - self.horizontal_position) as u64};
        self.horizontal_position = h_pos;
    }

    fn move_to_part_2(&mut self, h_pos: u32) {
        let i = if self.horizontal_position > h_pos {self.horizontal_position - h_pos} else {h_pos - self.horizontal_position};

        self.fuel_used += (i * (i + 1) / 2) as u64;
        self.horizontal_position = h_pos;
    }
}

fn part1(input: Vec<String>) -> u32 {
    let mut crabs: Vec<Crab> = Vec::new();
    let mut highest_pos = 0;
    let mut lowest_score = 1000000;
    for crab in input[0].split(',').collect::<Vec<&str>>() {
        let pos = crab.parse::<u32>().unwrap();
        crabs.push(Crab::new(pos));
        if pos > highest_pos {
            highest_pos = pos;
        }
    }

    for hpos in 0..highest_pos {
        let mut crabs_copy = crabs.clone();
        let mut tmp_score = 0;
        for i in 0..crabs_copy.len() {
            if tmp_score > lowest_score {
                break;
            }
            crabs_copy[i].move_to(hpos);
            tmp_score += crabs_copy[i].fuel_used;
        }
        if tmp_score < lowest_score {
            lowest_score = tmp_score;
        }
    }
    
    return lowest_score.try_into().unwrap();
}

fn part2(input: Vec<String>) -> u64 {
    let mut crabs: Vec<Crab> = Vec::new();
    let mut highest_pos = 0;
    let mut lowest_score = u64::MAX;
    for crab in input[0].split(',').collect::<Vec<&str>>() {
        let pos = crab.parse::<u32>().unwrap();
        crabs.push(Crab::new(pos));
        if pos > highest_pos {
            highest_pos = pos;
        }
    }

    for hpos in 0..highest_pos {
        let mut crabs_copy = crabs.clone();
        let mut tmp_score = 0;
        for i in 0..crabs_copy.len() {
            if tmp_score > lowest_score {
                break;
            }
            crabs_copy[i].move_to_part_2(hpos);
            tmp_score += crabs_copy[i].fuel_used;
        }
        if tmp_score < lowest_score {
            lowest_score = tmp_score;
        }
    }
    
    return lowest_score.into();
}

pub fn solution() {
    let mut now = Instant::now();
    println!("Day {} \n", DAY);
    println!("Part 1: {} in {:.2?}", part1(get_input_of_day(DAY)), now.elapsed());
    now = Instant::now();
    println!("Part 2: {} in {:.2?}", part2(get_input_of_day(DAY)), now.elapsed());
    println!("-------------------------------------\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part1_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part1(array), 37);
    }

    #[test]
    fn part2_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part2(array), 168);
    }
}