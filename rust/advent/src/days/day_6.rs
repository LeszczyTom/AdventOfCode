use crate::*;
use std::collections::HashMap;
use std::time::Instant;

const DAY: u8 = 6;

#[derive(Debug)]
struct LanternFish {
    timer: u8,
}

impl LanternFish {
    fn new() -> Self {
        Self { timer: 8 }
    }

    fn new_timer(timer: u8) -> Self {
        Self { timer }
    }

    fn tick(&mut self) -> bool {
        if self.timer == 0 {
            self.timer = 6;
            return true;
        } else {
            self.timer -= 1;
            return false;
        }
    }
}

fn part1(input: Vec<String>) -> u32 {
    let mut fishs: Vec<LanternFish> = Vec::new();

    for time in input[0].split(',').collect::<Vec<&str>>() {
        fishs.push(LanternFish::new_timer(time.parse::<u8>().unwrap()));
    }
    
    for _ in 0..80 {
        let mut new_fishs: u32 = 0;
        for fish in 0..fishs.len() {
            if fishs[fish].tick() {
                new_fishs += 1;
            }
        }
        for _ in 0..new_fishs {
            fishs.push(LanternFish::new());
        }
    }
    return fishs.len() as u32;
}

fn part2(input: Vec<String>) -> u64 {
    let mut timers: HashMap<u8, u64> = HashMap::new(); 
    for i in 0..9 {
        timers.insert(i, 0);
    }

    for time in input[0].split(',').collect::<Vec<&str>>() {
        *timers.get_mut(&time.parse::<u8>().unwrap()).unwrap() += 1;
    }

    for _ in 0..256 {
        let zero_timer_sum: u64 = *timers.get(&0).unwrap();
        for i in 0..9 {
            match i {
                8 => {
                    *timers.get_mut(&i).unwrap() = zero_timer_sum;
                    *timers.get_mut(&6).unwrap() += zero_timer_sum;
                }
                _ => {
                    *timers.get_mut(&i).unwrap() = *timers.get(&(i+1)).unwrap();
                }
            }
        }
    }

    let mut sum: u64 = 0;
    for i in timers.values() {
        sum += i;
    }
    return sum;
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
    
    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn part1_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part1(array), 5934);
    }

    #[test]
    fn part2_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part2(array), 26984457539);
    }
}