use crate::*;
use std::collections::HashMap;
use std::time::Instant;

const DAY: u8 = 10;
const CLOSING: [char; 4] = ['}', ']', ')', '>'];
const OPENING: [char; 4] = ['{', '[', '(', '<'];

fn part1(input: Vec<String>) -> u32 {
    let points: HashMap<char, u32> = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)].into_iter().collect();
    let mut sum = 0;

    for i in input {
        let mut stack: Vec<char> = Vec::new();
        for c in i.chars().collect::<Vec<char>>() {
            if OPENING.contains(&c) {
                stack.push(CLOSING[OPENING.iter().position(|&x| x == c).unwrap()]);
                continue;
            }
            if CLOSING.contains(&c) {
                if stack.pop().unwrap() != c {
                    sum += points.get(&c).unwrap();
                    break;
                }
            }      
        }
    }
    return sum;
}

fn part2(input: Vec<String>) -> u64 {
    let points: HashMap<char, u64> = [(')', 1), (']', 2), ('}', 3), ('>', 4)].into_iter().collect();
    let mut scores: Vec<u64> = Vec::new();

    for i in input {
        let mut stack: Vec<char> = Vec::new();
        for c in i.chars().collect::<Vec<char>>() {
            if OPENING.contains(&c) {
                stack.push(CLOSING[OPENING.iter().position(|&x| x == c).unwrap()]);
                continue;
            }
            if CLOSING.contains(&c) {
                if stack.pop().unwrap() != c {
                    stack.clear();
                    break;
                }
            }      
        }

        let mut sum: u64 = 0;
        while !stack.is_empty() {
            let c = stack.pop().unwrap();
            sum *= 5;
            sum += points.get(&c).unwrap();
        }
        if sum != 0 {
            scores.push(sum);
        }
    }
    scores.sort();
    return scores[scores.len() / 2];
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
    
    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    
    #[test]
    fn part1_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part1(array), 26397);
    }

    #[test]
    fn part2_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part2(array), 288957);
    }
}