use crate::*;
use std::collections::HashMap;

const DAY: u8 = 5;

#[derive(Debug, Clone)]
struct HydroVentsLines {
    lines: Vec<Line>,
}

#[derive(Debug, Clone)]
struct Line {
    vector1: Vector,
    vector2: Vector,
}

#[derive(Debug, Clone)]
struct Vector {
    x: u32,
    y: u32,
}

fn update_map(hash_map: &mut HashMap<u32, HashMap<u32,u32>>, x: u32, y: u32, sum: &mut u32) {
    if !hash_map.contains_key(&x) {
        hash_map.insert(x, HashMap::<u32,u32>::new());   
    } 
    if !hash_map.get(&x).unwrap().contains_key(&y) {
        //map.get_mut(&x).insert(0);  
        hash_map.get_mut(&x).unwrap().insert(y, 0);
    } 
    let cpt = hash_map.get(&x).unwrap().get(&y).unwrap();
    match cpt {
        &1 => {
            *hash_map.get_mut(&x).unwrap().get_mut(&y).unwrap() += 1;
            *sum += 1;
        }
        &0 => {
            *hash_map.get_mut(&x).unwrap().get_mut(&y).unwrap() += 1;
        }
        _ => {}
    }
}

fn part1(input: Vec<String>) -> u32 {
    //println!("{:?}", input);
    let mut vents_lines = HydroVentsLines {
        lines: Vec::<Line>::new(),
    };

    for i in 0..input.len() {
        let tmp = input[i].split(" -> ").collect::<Vec<&str>>();
        
        let vector1: Vector = Vector {
            x: tmp[0].split(",").collect::<Vec<&str>>()[0].parse::<u32>().unwrap(),
            y: tmp[0].split(",").collect::<Vec<&str>>()[1].parse::<u32>().unwrap(),
        };

        let vector2: Vector = Vector {
            x: tmp[1].split(",").collect::<Vec<&str>>()[0].parse::<u32>().unwrap(),
            y: tmp[1].split(",").collect::<Vec<&str>>()[1].parse::<u32>().unwrap(),
        };

        let line: Line = Line {
            vector1: vector1,
            vector2: vector2,
        };
        vents_lines.lines.push(line);
    }

    // <x, <y, cpt>>
    let mut hash_map: HashMap<u32, HashMap<u32,u32>> = HashMap::new();
    let mut sum = 0;

    for line in vents_lines.lines {
        let x1 = line.vector1.x as u32;
        let y1 = line.vector1.y as u32;
        let x2 = line.vector2.x as u32;
        let y2 = line.vector2.y as u32;

        let min_x = if x1 < x2 { x1 } else { x2 };
        let min_y = if y1 < y2 { y1 } else { y2 };
        let max_x = if x1 > x2 { x1 } else { x2 };
        let max_y = if y1 > y2 { y1 } else { y2 };

        let mut x;
        let mut y;

        if x1 == x2 {
            //println!("\nHorizontal");
            //println!("{:?}", line);
            for i in min_y..max_y + 1{
                x = x1;
                y = i;  
                update_map(&mut hash_map, x, y, &mut sum);     
                //println!("x: {}, y: {}", x, y);
            }

        }
        if y1 == y2 {
            //println!("\nVertical");
            //println!("{:?}", line);
            for i in min_x..max_x + 1{
                x = i;
                y = y1;
                //println!("x: {}, y: {}", x, y);
                update_map(&mut hash_map, x, y, &mut sum);
            }
        }
    }

    return sum;
}

fn part2(input: Vec<String>) -> u32 {
    //println!("{:?}", input);
    let mut vents_lines = HydroVentsLines {
        lines: Vec::<Line>::new(),
    };

    for i in 0..input.len() {
        let tmp = input[i].split(" -> ").collect::<Vec<&str>>();
        
        let vector1: Vector = Vector {
            x: tmp[0].split(",").collect::<Vec<&str>>()[0].parse::<u32>().unwrap(),
            y: tmp[0].split(",").collect::<Vec<&str>>()[1].parse::<u32>().unwrap(),
        };

        let vector2: Vector = Vector {
            x: tmp[1].split(",").collect::<Vec<&str>>()[0].parse::<u32>().unwrap(),
            y: tmp[1].split(",").collect::<Vec<&str>>()[1].parse::<u32>().unwrap(),
        };

        let line: Line = Line {
            vector1: vector1,
            vector2: vector2,
        };
        vents_lines.lines.push(line);
    }

    // <x, <y, cpt>>
    let mut hash_map: HashMap<u32, HashMap<u32,u32>> = HashMap::new();
    let mut sum = 0;

    for line in vents_lines.lines {
        let x1 = line.vector1.x as u32;
        let y1 = line.vector1.y as u32;
        let x2 = line.vector2.x as u32;
        let y2 = line.vector2.y as u32;
        
        let x_diff = if x2 > x1 { x2 - x1 } else { x1 - x2 };
        let y_diff = if y2 > y1 { y2 - y1 } else { y1 - y2 };

        let min_x = if x1 < x2 { x1 } else { x2 };
        let min_y = if y1 < y2 { y1 } else { y2 };
        let max_x = if x1 > x2 { x1 } else { x2 };
        let max_y = if y1 > y2 { y1 } else { y2 };

        let mut x;
        let mut y;

        if x1 == x2 {
            //println!("\nHorizontal");
            //println!("{:?}", line);
            for i in min_y..max_y + 1{
                x = x1;
                y = i;  
                update_map(&mut hash_map, x, y, &mut sum);     
                //println!("x: {}, y: {}", x, y);
            }

        }
        if y1 == y2 {
            //println!("\nVertical");
            //println!("{:?}", line);
            for i in min_x..max_x + 1{
                x = i;
                y = y1;
                //println!("x: {}, y: {}", x, y);
                update_map(&mut hash_map, x, y, &mut sum);
            }
        }
        if x_diff == y_diff {
            //println!("\n45°diagonal");
            //println!("{:?}", line); 
            let diff = max_x - min_x;
            for i in 0..diff + 1 {
                if min_x == x1 {
                    x = x1 + i;
                }  else {
                    x = x1 - i;
                }

                if min_y == y1 {
                    y = y1 + i;
                } else {
                    y = y1 - i;
                }
                //println!("x: {}, y: {}", x, y);
                update_map(&mut hash_map, x, y, &mut sum);
            }
        } 
    }

    return sum;
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
    
    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part1_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part1(array), 5);
    }

    #[test]
    fn part2_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part2(array), 12);
    }
}