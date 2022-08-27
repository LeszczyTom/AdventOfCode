use crate::*;
use std::collections::HashMap;
use std::time::Instant;

const DAY: u8 = 8;

/*
len -> digits
2 -> 1
3 -> 7
4 -> 4
5 -> 2, 3 ,5
6 -> 0, 6, 9
7 -> 8
*/

fn part1(input: Vec<String>) -> u32 {
    let mut sum = 0;
    
    for ind in 0..input.len() {
        let outputs: Vec<&str> = input[ind].split(" | ").collect::<Vec<&str>>()[1].split(' ').collect::<Vec<&str>>();
        for i in outputs {
            if i.len() == 2 || i.len() == 3 || i.len() == 4 || i.len() == 7 {
                sum += 1;
            }
        }
    }
    return sum;
}

fn part2(input: Vec<String>) -> u64 {
    let mut res:u64 = 0;
    for inp in 0..input.len() {
        let outputs: Vec<&str> = input[inp].split(" | ").collect::<Vec<&str>>()[1].split(' ').collect::<Vec<&str>>();
        let inputs: Vec<&str> = input[inp].split(" | ").collect::<Vec<&str>>()[0].split(' ').collect::<Vec<&str>>();
    
        let mut map: HashMap<u8, Vec<char>> = HashMap::new();
        for i in 0..10 {
            map.insert(i, Vec::new());
        }

        let mut z = [false; 10];
        let mut cpy = inputs.clone();
        while cpy.len() != 0 {
            for i in 0..cpy.len() {
                let tmp = cpy[i].chars().collect::<Vec<char>>();
                match tmp.len() {
                    2 => {
                        // Pattern of 1
                        *map.get_mut(&1).unwrap() = tmp;
                        cpy.remove(i);
                        z[1] = true;
                        break;
                    },
                    3 => {
                        // Pattern of 7
                        *map.get_mut(&7).unwrap() = tmp;
                        cpy.remove(i);
                        z[7] = true;
                        break;
                    },
                    4 => {
                        // Pattern of 4
                        *map.get_mut(&4).unwrap() = tmp;
                        cpy.remove(i);
                        z[4] = true;
                        break;
                    },
                    7 => {
                        // Pattern of 8
                        *map.get_mut(&8).unwrap() = tmp;
                        cpy.remove(i);
                        z[8] = true;
                        break;
                    },
                    6 => {
                        let one = map.get(&1).unwrap();
                        if !z[6] && z[1] && (!tmp.contains(&one[0]) || !tmp.contains(&one[1])) {
                            *map.get_mut(&6).unwrap() = tmp;
                            cpy.remove(i);
                            z[6] = true;
                            break;
                        } 
                        if z[6] && z[4] {
                            let four = map.get(&4).unwrap();
                            let mut comm = 0;
                            for j in four {
                                if tmp.contains(j) {
                                    comm += 1;
                                }
                            }
                            if comm == 4 && !z[9] {
                                *map.get_mut(&9).unwrap() = tmp;
                                cpy.remove(i);
                                z[9] = true;
                                break;
                            }
                            if comm == 3 && !z[0] {
                                *map.get_mut(&0).unwrap() = tmp;
                                cpy.remove(i);
                                z[0] = true;
                                break;
                            }
                        }
                    },
                    5 => {
                        let one = map.get(&1).unwrap();
                        if !z[3] && z[1] && tmp.contains(&one[0]) && tmp.contains(&one[1]) {
                            *map.get_mut(&3).unwrap() = tmp;
                            cpy.remove(i);
                            z[3] = true;
                            break;
                        } 
                        if z[3] && z[4] {
                            let four = map.get(&4).unwrap();
                            let mut comm = 0;
                            for j in four {
                                if tmp.contains(j) {
                                    comm += 1;
                                }
                            }
                            if comm == 3 && !z[5] {
                                *map.get_mut(&5).unwrap() = tmp;
                                cpy.remove(i);
                                z[5] = true;
                                break;
                            }
                            if comm == 2 && !z[2] {
                                *map.get_mut(&2).unwrap() = tmp;
                                cpy.remove(i);
                                z[2] = true;
                                break;
                            }
                        }
                    }
                    _ => {
                    
                    }
                }
            }
        }

        let mut sum = 0;
        for i in 0..outputs.len() {
            match outputs[i].len() {
                2 => {
                    sum += 1 * 10_u32.pow((3 - i).try_into().unwrap());
                },
                3 => {
                    sum += 7 * 10_u32.pow((3 - i).try_into().unwrap());
                },
                4 => {
                    sum += 4 * 10_u32.pow((3 - i).try_into().unwrap());
                },
                7 => {
                    sum += 8 * 10_u32.pow((3 - i).try_into().unwrap());
                },
                6 => {
                    let tmp = outputs[i].chars().collect::<Vec<char>>();
                    if is_value(&map, &tmp, 0) {

                    } else if is_value(&map, &tmp, 6) {
                        sum += 6 * 10_u32.pow((3 - i).try_into().unwrap());
                    } else if is_value(&map, &tmp, 9) {
                        sum += 9 * 10_u32.pow((3 - i).try_into().unwrap());
                    }
                },
                5 => {
                    let tmp = outputs[i].chars().collect::<Vec<char>>();
                    if is_value(&map, &tmp, 2) {
                        sum += 2 * 10_u32.pow((3 - i).try_into().unwrap());
                    } else if is_value(&map, &tmp, 3) {
                        sum += 3 * 10_u32.pow((3 - i).try_into().unwrap());
                    } else if is_value(&map, &tmp, 5) {
                        sum += 5 * 10_u32.pow((3 - i).try_into().unwrap());
                    }
                },
                _ => {panic!("error");},
            }
        }
        res += sum as u64;
    }

    return res;
}

fn is_value(map: & HashMap<u8, Vec<char>>, input: &Vec<char>, p: u8) -> bool {
    for i in map.get(&p).unwrap() {
        if !input.contains(i) {
            return false;
        }
    }
    return true;
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
    
    const INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
//const INPUT: &str = "ef cafeb gbcfea agcbdf agbfc gfae bfe cgfbeda bdefcg dceab | eafg dgcbef cfgba bcagedf";
    
    #[test]
    fn part1_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part1(array), 26);
    }

    #[test]
    fn part2_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part2(array), 61229);
    }
}