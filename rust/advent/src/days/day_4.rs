use crate::*;

const DAY: u8 = 4;

#[derive(Debug, Clone)]
struct Bingo<'a> {
    numbers: Vec<&'a str>,
    boards: Vec<Board<'a>>,
}

#[derive(Debug, Clone)]
struct Board<'a> {
    tiles: Vec<Vec<&'a str>>,
    checked_tiles: Vec<Vec<bool>>,
}

fn isBingo(board: &Board) -> bool {
    for i in 0..5 {
        if board.checked_tiles[i][0] && 
            board.checked_tiles[i][1] &&
            board.checked_tiles[i][2] &&
            board.checked_tiles[i][3] &&
            board.checked_tiles[i][4] {
                return true;
            }
        if board.checked_tiles[0][i] && 
            board.checked_tiles[1][i] &&
            board.checked_tiles[2][i] &&
            board.checked_tiles[3][i] &&
            board.checked_tiles[4][i] {
                return true;
            }
    }
    return false;
}

fn sum_of_unmarked(board: &Board) -> u32 {
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !board.checked_tiles[i][j] {
                sum += board.tiles[i][j].parse::<u32>().unwrap();
            }
        }
    }
    return sum;
}

fn part1(input: Vec<String>) -> u32 {
    let mut bingo: Bingo = Bingo {
        numbers: input[0].split(',').collect::<Vec<&str>>(),
        boards: Vec::<Board>::new(),
    };

    let mut board: Board = Board {
        tiles: Vec::<Vec<&str>>::new(),
        checked_tiles: Vec::<Vec<bool>>::new(),
    };

    for i in 2..input.len() {
        if input[i] == "" {
            bingo.boards.push(board.clone());
            board = Board {
                tiles: Vec::<Vec<&str>>::new(),
                checked_tiles: Vec::<Vec<bool>>::new(),
            };
            continue;
        }
        let mut row: Vec<&str> = input[i].split(' ').collect::<Vec<&str>>();
        row.retain(|&x| x != "");
        board.checked_tiles.push(vec![false; row.len()]);
        board.tiles.push(row);
    }

    for i in &bingo.numbers {
        for j in 0..bingo.boards.len() {
            for k in 0..bingo.boards[j].tiles.len() {
                for l in 0..bingo.boards[j].tiles[k].len() {
                    if &bingo.boards[j].tiles[k][l] == i {
                        bingo.boards[j].checked_tiles[k][l] = true;
                    }
                    if isBingo(&bingo.boards[j]) {
                        return sum_of_unmarked(&bingo.boards[j]) * i.parse::<u32>().unwrap();
                    }  
                }
            }
        }
    }
    return 0;
}

// Ashamed of what i have written, nevertheless I won't modify it.
fn part2(input: Vec<String>) -> u32 {
    let mut bingo: Bingo = Bingo {
        numbers: input[0].split(',').collect::<Vec<&str>>(),
        boards: Vec::<Board>::new(),
    };

    let mut board: Board = Board {
        tiles: Vec::<Vec<&str>>::new(),
        checked_tiles: Vec::<Vec<bool>>::new(),
    };

    for i in 2..input.len() {
        if input[i] == "" {
            bingo.boards.push(board.clone());
            board = Board {
                tiles: Vec::<Vec<&str>>::new(),
                checked_tiles: Vec::<Vec<bool>>::new(),
            };
            continue;
        }
        let mut row: Vec<&str> = input[i].split(' ').collect::<Vec<&str>>();
        row.retain(|&x| x != "");
        board.checked_tiles.push(vec![false; row.len()]);
        board.tiles.push(row);
    }

    let mut last_score = 0;
    while true {
        
        if bingo.boards.len() == 0 || bingo.numbers.len() == 1 {
            return last_score;
        }
        
        let tmp = bingo.clone();
        let res = te(tmp);
        bingo = res.bingo;
        if res.score != 0 {
            last_score = res.score;
        }
    }
    return 0;
}

struct Res<'a> {
    bingo: Bingo<'a>,
    score: u32,
}

fn te(mut bingo: Bingo) -> Res {
    let mut tmp = bingo.clone();
    for i in 0..bingo.numbers.len() {
        for j in 0..bingo.boards.len() {
            for k in 0..bingo.boards[j].tiles.len() {
                for l in 0..bingo.boards[j].tiles[k].len() {
                    if bingo.boards[j].tiles[k][l] == bingo.numbers[i] {
                        bingo.boards[j].checked_tiles[k][l] = true;
                        tmp.boards[j].checked_tiles[k][l] = true;
                    }       
                }
            }
        }
        let mut res = Res {
            bingo: Bingo{
                numbers: Vec::<&str>::new(),
                boards: Vec::<Board>::new(),
            },
            score: 0,
        };
        let mut offset = 0;
        for j in 0..bingo.boards.len() {
            if isBingo(&bingo.boards[j]) {
                let sc = sum_of_unmarked(&tmp.boards[j - offset]) * bingo.numbers[i].parse::<u32>().unwrap();
                tmp.boards.remove(j - offset);
                offset += 1;
                if tmp.numbers[0] == bingo.numbers[i] {
                    tmp.numbers.remove(0);
                } 
                res = Res {bingo: tmp.clone(), score: sc};
            }  
        }
        if res.score != 0 {
            return res;
        }
        tmp.numbers.remove(0);
    }
    return Res {bingo: tmp, score: 0};
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
    
    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7
";

    #[test]
    fn part1_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part1(array), 4512);
    }

    #[test]
    fn part2_test() {
        let array: Vec<String> = string_to_array_of_strings(INPUT.to_string());
        assert_eq!(part2(array), 1924);
    }
}