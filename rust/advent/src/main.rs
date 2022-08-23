use std::fs;

pub mod day_1;
pub mod day_2;
pub mod day_3;

fn main() {
    day_1::solution();
    day_2::solution();
    day_3::solution();
}

fn get_input_of_day(nb: u8) -> Vec<String> {
    let file_path: String = "input/day_".to_owned() + &nb.to_string() + ".txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    return string_to_array_of_strings(contents);
}

fn string_to_array_of_strings(contents: String) -> Vec<String> {
    return contents.split('\n').map(|s| s.to_string()).collect();
}