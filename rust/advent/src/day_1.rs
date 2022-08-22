use crate::*;

fn part1(input: Vec<String>) -> u32 {
    let array: Vec<u32> = input.iter().map(|s| s.parse::<u32>().unwrap()).collect();
    let mut tmp: u32 = array[0];
    let mut sum:u32 = 0;

    for i in 1..array.len() {
        if tmp < array[i] {
            sum += 1;
        }
        tmp = array[i];
    }
    return sum;
}

fn part2(input: Vec<String>) -> u32 {
    let array: Vec<u32> = input.iter().map(|s| s.parse::<u32>().unwrap()).collect();
    let mut sum: u32 = 0;
    let mut tmp: u32 = array[0] + array[1] + array[2];

    for i in 1..array.len() - 2 {
        if tmp < array[i] + array[i + 1] + array[i + 2] {
            sum += 1;
        }
        tmp = array[i] + array[i + 1] + array[i + 2];
    }

    return sum;
}

pub fn solution() {
    println!("Part 1: {}", part1(get_input_of_day(1)));
    println!("Part 2: {}", part2(get_input_of_day(1)));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1_test() {
        let input: String = "199
200
208
210
200
207
240
269
260
263".to_string();
        println!("{}", input);
        let array: Vec<String> = string_to_array_of_strings(input);
        assert_eq!(part1(array), 7);
    }

    #[test]
    fn part2_test() {
        let input: String = "199
200
208
210
200
207
240
269
260
263".to_string();
        let array: Vec<String> = string_to_array_of_strings(input);
        assert_eq!(part2(array), 5);
    }
}