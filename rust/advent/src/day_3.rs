use crate::*;

const DAY: u8 = 3;

fn part1(input: Vec<String>) -> u32 {
    let mut array: Vec<i32> = Vec::new();
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for i in 0..input[0].len() {
        array.push(0);
    }

    for i in 0..input.len() {
        let char_vec: Vec<char> = input[i].chars().collect();
        for j in 0..char_vec.len() {
            if char_vec[j].to_digit(10).unwrap() == 0 {
                array[j] -= 1
            } else {
                array[j] += 1
            }
        }
    }

    array.reverse();
    for i in 0..array.len() {
        if array[i] > 0 {
            gamma += 1 * 2_u32.pow(i as u32);
        } else {
            epsilon += 1 * 2_u32.pow(i as u32);
        }
    }

    return gamma * epsilon;
}

fn part2(input: Vec<String>) -> u32 {
    let mut array: Vec<i32> = Vec::new();

    for i in 0..input[0].len() {
        array.push(0);
    }

    let mut O2_rating: Vec<String> = input.clone();
    for  j in 0..array.len() {
        if O2_rating.len() == 1 {
            break;
        }
        let mut tmp: Vec<String> = Vec::new();
        let mut bit_criteria: i16 = 0;

        for k in 0..O2_rating.len() {
            if O2_rating[k].chars().nth(j).unwrap() == '1' {
                bit_criteria += 1;
            } else {
                bit_criteria -= 1;
            }
        }

        for i in 0..O2_rating.len() {
            let mut c = O2_rating[i].chars().nth(j).unwrap();
            if bit_criteria >= 0 {
                if c == '1' {
                    tmp.push(O2_rating[i].clone());
                }
            } else {
                if c == '0' {
                    tmp.push(O2_rating[i].clone());
                }
            }
        }
        O2_rating = tmp;
    }

    let mut CO2_rating: Vec<String> = input.clone();
    for  j in 0..array.len() {
        if CO2_rating.len() == 1 {
            break;
        }
        let mut tmp: Vec<String> = Vec::new();
        let mut bit_criteria: i16 = 0;

        for k in 0..CO2_rating.len() {
            if CO2_rating[k].chars().nth(j).unwrap() == '1' {
                bit_criteria += 1;
            } else {
                bit_criteria -= 1;
            }
        }

        for i in 0..CO2_rating.len() {
            let mut c = CO2_rating[i].chars().nth(j).unwrap();
            if bit_criteria >= 0 {
                if c == '0' {
                    tmp.push(CO2_rating[i].clone());
                }
            } else {
                if c == '1' {
                    tmp.push(CO2_rating[i].clone());
                }
            }
        }
        CO2_rating = tmp;
    }

    let mut O2_chars = O2_rating[0].chars();
    let mut O2_sum = 0;
    for i in 0..O2_rating[0].len() {
        O2_sum += O2_chars.next_back().unwrap().to_digit(10).unwrap() as i32 * 2_i32.pow(i as u32);
    }
    
    let mut CO2_chars = CO2_rating[0].chars();
    let mut CO2_sum = 0;
    for i in 0..O2_rating[0].len() {
        CO2_sum += CO2_chars.next_back().unwrap().to_digit(10).unwrap() as i32 * 2_i32.pow(i as u32);
    }

    return (O2_sum * CO2_sum).try_into().unwrap();
}

pub fn solution() {
    println!("Day {} \n", DAY);
    println!("Part 1: {}", part1(get_input_of_day(DAY)));
    println!("Part 2: {}", part2(get_input_of_day(DAY)));
    println!("-------------------------------------\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part1_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part1(array), 198);
    }

    #[test]
    fn part2_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part2(array), 230);
    }
}