use crate::*;
use std::time::Instant;

const DAY: u8 = 11;

#[derive(Debug)]
struct Cavern {
    grid: Vec<Vec<DumboOctopus>>,
    flashs: u32
}

#[derive(Debug, Copy, Clone)]
struct DumboOctopus {
    flashed: bool,
    value: u8
}

impl Cavern {
    fn new(input: Vec<String>) -> Cavern {
        let mut grid: Vec<Vec<DumboOctopus>> = Vec::new();
        for i in 0..10 {
            grid.push(Vec::new());
            let c = input[i].chars().collect::<Vec<char>>();
            for j in 0..10 {
                grid[i].push(DumboOctopus {
                    flashed: false,
                    value: c[j].to_digit(10).unwrap() as u8
                });
            }
        }
        return Cavern {grid: grid, flashs: 0};
    }

    fn print(&self) {
        println!("\n");
        for x in 0..10 {
            for y in 0..10 {
                let v = self.grid[x][y].value;
                if v == 0 {
                    print!("\x1b[93m{}\x1b[0m ", v);
                } else {
                    print!("{} ", v);
                } 
            }
            println!("");
        }
    }

    fn step(&mut self) {
        let mut coordinates_to_update: Vec<(i8,i8)> = Vec::new();
        for x in 0..10 {
            for y in 0..10 {
                coordinates_to_update.push((x,y));
            }
        }
        while !coordinates_to_update.is_empty() {
            let mut to_add: Vec<(i8,i8)> = Vec::new();
            for (x, y) in &coordinates_to_update {
                if self.grid[*x as usize][*y as usize].update() {
                    self.flashs += 1;
                    for i in -1..2 {
                        for j in -1..2 {
                            let new_x: i8 = *x + i;
                            let new_y: i8 = *y + j;
                            if new_x >= 0 && new_x < 10 && new_y >= 0 && new_y < 10 {
                                to_add.push((new_x, new_y)); 
                            }  
                        }
                    }
                }
            } 
            coordinates_to_update.clear(); 
            for (x, y) in to_add {
                coordinates_to_update.push((x,y));
            }
        }
        //self.print();
        for x in 0..10 {
            for y in 0..10 {
                self.grid[x][y].reset_flashed();
            }
        }
    }

    fn is_all_flashing(& self) -> bool {
        for x in 0..10 {
            for y in 0..10 {
                if self.grid[x][y].value != 0 {
                    return false;
                }
            }
        }
        return true;
    }
}

impl DumboOctopus {
    fn reset_flashed(&mut self) {
        self.flashed = false;
    }

    fn update(&mut self) -> bool {
        if !self.flashed {
            self.value += 1;
            if self.value == 10 {
                self.value = 0;
                self.flashed = true;
                return true;
            }
        }
        return false;
    }   
}
fn part1(input: Vec<String>) -> u32 {
    let mut cavern = Cavern::new(input);
    for _ in 0..100 {
        cavern.step();
    }
    
    return cavern.flashs;
}

fn part2(input: Vec<String>) -> u64 {
    let mut cavern = Cavern::new(input);
    let mut cpt = 0;
    while !cavern.is_all_flashing() {
        cavern.step();
        cpt += 1;
    }

   return cpt;
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
    
    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    
    #[test]
    fn part1_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part1(array), 1656);
    }

    #[test]
    fn part2_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part2(array), 195);
    }
}