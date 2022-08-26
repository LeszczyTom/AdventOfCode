use crate::*;
use std::time::Instant;

const DAY: u8 = 2;

fn part1(input: Vec<String>) -> u32 {
    let mut horizontal_sum = 0;
    let mut depth_sum = 0;

    for i in 0..input.len() {
        let line: Vec<String> = input[i].split(' ').map(|s| s.to_string()).collect();
        if line[0] == "forward".to_string() {
            horizontal_sum += line[1].parse::<u32>().unwrap();
        }
        if line[0] == "down".to_string() {
            depth_sum += line[1].parse::<u32>().unwrap();
        }
        if line[0] == "up".to_string() {
            depth_sum -= line[1].parse::<u32>().unwrap();
        }
    }
    return horizontal_sum * depth_sum;
}

fn part2(input: Vec<String>) -> u32 {
    let mut horizontal_sum = 0;
    let mut depth_sum = 0;
    let mut aim_sum = 0;

    for i in 0..input.len() {
        let line: Vec<String> = input[i].split(' ').map(|s| s.to_string()).collect();
        if line[0] == "forward".to_string() {
            horizontal_sum += line[1].parse::<u32>().unwrap();
            depth_sum += line[1].parse::<u32>().unwrap() * aim_sum;
        }
        if line[0] == "down".to_string() {
            aim_sum += line[1].parse::<u32>().unwrap();
        }
        if line[0] == "up".to_string() {
            aim_sum -= line[1].parse::<u32>().unwrap();
        }
    }
    return horizontal_sum * depth_sum;
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
    
    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part1_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part1(array), 150);
    }

    #[test]
    fn part2_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part2(array), 900);
    }
}