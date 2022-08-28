use crate::*;
use std::time::Instant;

const DAY: u8 = 9;

fn part1(input: Vec<String>) -> u32 {
    let mut matrix_of_floor: Vec<Vec<char>> = Vec::new();
    let mut sum: u32 = 0;
    for i in input {
        matrix_of_floor.push(i.chars().collect::<Vec<char>>());
    }
    for x in 0..matrix_of_floor.len() {
        for y in 0..matrix_of_floor[x].len() {
            if is_low_point(&matrix_of_floor, x, y) {
                sum += 1 + matrix_of_floor[x][y].to_digit(10).unwrap();
                //println!("{}", matrix_of_floor[x][y] );
            }
        }
    }
    return sum;
}

fn part2(input: Vec<String>) -> u32 {
    let mut matrix_of_floor: Vec<Vec<char>> = Vec::new();
    let mut sums : Vec<u32> = Vec::new();
    for i in input {
        matrix_of_floor.push(i.chars().collect::<Vec<char>>());
    }

    for x in 0..matrix_of_floor.len() {
        for y in 0..matrix_of_floor[x].len() {
            if is_low_point(&matrix_of_floor, x, y) {
                let seen: Vec<(usize, usize)> = Vec::new();
                sums.push(get_value(&matrix_of_floor, x, y, 4, seen).len().try_into().unwrap());
            }
        }
    }
    let mut sum = 1;
    for _ in 0..3 {
        let mut max = (0,0);
        for (i, sum) in sums.iter().enumerate() {
            if max == (0,0) {
                max = (i, *sum);
            }  
            if *sum > max.1 {
                max = (i, *sum);
            }  
        }
        sum *= max.1;
        sums.remove(max.0);
    }
    
    return sum;
}

fn get_to_check(length: usize, width: usize, x: usize, y: usize) -> [bool; 4]{
    // Left - Top - Right - Bottom
    let mut to_check = [true; 4];

    if x == 0 {
        to_check[1] = false;
    }
    if y == 0 {
        to_check[0] = false;
    }
    if x == length - 1 {
        to_check[3] = false;
    }
    if y == width - 1 {
        to_check[2] = false;
    }

    return to_check;
}

fn is_low_point(matrix_of_floor: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let to_check = get_to_check(matrix_of_floor.len(), matrix_of_floor[0].len(), x, y);

    let current = matrix_of_floor[x][y];
    for (i, b) in to_check.iter().enumerate() {
        if *b {
            match i {
                0 => {
                    if matrix_of_floor[x][y - 1] <= current {
                        return false;
                    }
                },
                1 => {
                    if matrix_of_floor[x - 1][y] <= current {
                        return false;
                    }
                },
                2 => {
                    if matrix_of_floor[x][y + 1] <= current {
                        return false;
                    }
                },
                3 => {
                    if matrix_of_floor[x + 1][y] <= current {
                        return false;
                    }
                }
                _ => {
                    panic!("Error");
                }
            }
        }
    }
    return true;
}

// from -> 0: left, 1: top, 2: right, 3: bottom, 4: root
fn get_value(matrix_of_floor: &Vec<Vec<char>>, x: usize, y: usize, from: usize, mut seen: Vec<(usize, usize)>) -> Vec<(usize, usize)>{
    if matrix_of_floor[x][y] == '9' || seen.contains(&(x, y)) {
        return seen;
    }
    seen.push((x, y));
    let mut to_check = get_to_check(matrix_of_floor.len(), matrix_of_floor[0].len(), x, y);
    if from != 4 {
        to_check[from] = false;
    }
    for (i,b) in to_check.iter().enumerate() {
        if *b {
            match i {
                0 => {
                    seen = get_value(matrix_of_floor, x, y - 1, 2, seen.clone());
                },
                1 => {
                    seen = get_value(matrix_of_floor, x - 1, y, 3, seen.clone());
                },
                2 => {
                    seen = get_value(matrix_of_floor, x, y + 1, 0, seen.clone());
                },
                3 => {
                    seen = get_value(matrix_of_floor, x + 1, y, 1, seen.clone());
                }
                _ => {
                    panic!("Error");
                }
            }
        }
    }
    return seen;
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
    
    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";
    
    #[test]
    fn part1_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part1(array), 15);
    }

    #[test]
    fn part2_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part2(array), 1134);
    }
}