use std::io::Read;

fn get_input_data() -> String {
    let mut input_data = String::new();

    std::fs::File::open("input.txt")
        .expect("ERR: Input file doesn't exist")
        .read_to_string(&mut input_data)
        .expect("ERR: Couldn't read file contents");

    input_data
}

fn main() {
    println!("============= DAY 04 =============");
    
    // let data = get_input_data();
    let data = String::from("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX");

    let split_data = data.split("\n").collect::<Vec<&str>>();

    let result: u32 = split_data.iter().enumerate().map(|(row_index, &row)| {
        row.chars().enumerate().map(|(character_index, character)| {
            match character {
                'X' => {
                    check_position(&split_data, row_index, character_index)
                },
                _ => 0,
            }
        }).sum::<u32>()
    }).sum();

    println!("{}", result);
}

fn check_position(split_data: &Vec<&str>, row_index: usize, character_index: usize) -> u32 {
    let valid_positions = vec![check_left_top(&split_data, row_index, character_index),
                      check_center_top(&split_data, row_index, character_index),
                      check_right_top(&split_data, row_index, character_index),
                      check_left_middle(&split_data, row_index, character_index),
                      check_right_middle(&split_data, row_index, character_index),
                      check_left_bottom(&split_data, row_index, character_index),
                      check_center_bottom(&split_data, row_index, character_index),
                      check_right_bottom(&split_data, row_index, character_index)];
    
    valid_positions.iter().map(|&valid| {
        match valid {
            false => 0,
            true => 1,
        }
    }).sum()
}

const CHARACTERS_TO_MATCH: &str = "XMAS";

fn check_left_top(split_data: &Vec<&str>, row_index: usize, character_index: usize) -> bool {
    if row_index < 3 || character_index < 3 {
        return false;
    }

    !CHARACTERS_TO_MATCH.chars().enumerate().map(|(offset, character)| {
        character == split_data[row_index - offset].chars().nth(character_index - offset).unwrap()
    }).any(|is_valid_char| !is_valid_char)
}

fn check_center_top(split_data: &Vec<&str>, row_index: usize, character_index: usize) -> bool {
    if row_index < 3 {
        return false;
    }

    !CHARACTERS_TO_MATCH.chars().enumerate().map(|(offset, character)| {
        character == split_data[row_index - offset].chars().nth(character_index).unwrap()
    }).any(|is_valid_char| !is_valid_char)
}

fn check_right_top(split_data: &Vec<&str>, row_index: usize, character_index: usize) -> bool {
    if row_index < 3 || character_index > split_data[0].len() - 4 {
        return false;
    }

    !CHARACTERS_TO_MATCH.chars().enumerate().map(|(offset, character)| {
        character == split_data[row_index - offset].chars().nth(character_index + offset).unwrap()
    }).any(|is_valid_char| !is_valid_char)
}

fn check_left_middle(split_data: &Vec<&str>, row_index: usize, character_index: usize) -> bool {
    if character_index < 3 {
        return false;
    }

    !CHARACTERS_TO_MATCH.chars().enumerate().map(|(offset, character)| {
        character == split_data[row_index].chars().nth(character_index - offset).unwrap()
    }).any(|is_valid_char| !is_valid_char)
}

fn check_right_middle(split_data: &Vec<&str>, row_index: usize, character_index: usize) -> bool {
    if character_index > split_data[0].len() - 4 {
        return false;
    }

    !CHARACTERS_TO_MATCH.chars().enumerate().map(|(offset, character)| {
        character == split_data[row_index].chars().nth(character_index + offset).unwrap()
    }).any(|is_valid_char| !is_valid_char)
}

fn check_left_bottom(split_data: &Vec<&str>, row_index: usize, character_index: usize) -> bool {
    if row_index > split_data.len() - 4 || character_index < 3 {
        return false;
    }

    !CHARACTERS_TO_MATCH.chars().enumerate().map(|(offset, character)| {
        character == split_data[row_index + offset].chars().nth(character_index - offset).unwrap()
    }).any(|is_valid_char| !is_valid_char)
}

fn check_center_bottom(split_data: &Vec<&str>, row_index: usize, character_index: usize) -> bool {
    if row_index > split_data.len() - 4 {
        return false;
    }

    !CHARACTERS_TO_MATCH.chars().enumerate().map(|(offset, character)| {
        character == split_data[row_index + offset].chars().nth(character_index).unwrap()
    }).any(|is_valid_char| !is_valid_char)
}

fn check_right_bottom(split_data: &Vec<&str>, row_index: usize, character_index: usize) -> bool {
    if row_index > split_data.len() - 4 || character_index > split_data[0].len() - 4 {
        return false;
    }

    !CHARACTERS_TO_MATCH.chars().enumerate().map(|(offset, character)| {
        character == split_data[row_index + offset].chars().nth(character_index + offset).unwrap()
    }).any(|is_valid_char| !is_valid_char)
}
