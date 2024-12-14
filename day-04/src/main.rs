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
    
    let data = get_input_data();

    let split_data = data.split("\n").collect::<Vec<&str>>();

    let result: u32 = split_data.iter().enumerate().map(|(row_index, &row)| {
        row.chars().enumerate().map(|(character_index, character)| {
            match character {
                'A' => {
                    check_position(&split_data, row_index, character_index)
                },
                _ => 0,
            }
        }).sum::<u32>()
    }).sum();

    println!("{}", result);
}

fn check_position(split_data: &Vec<&str>, row_index: usize, character_index: usize) -> u32 {
    let left_top = check_left_top(&split_data, row_index, character_index);
    let left_bottom = check_left_bottom(&split_data, row_index, character_index);
    let right_top = check_right_top(&split_data, row_index, character_index);
    let right_bottom = check_right_bottom(&split_data, row_index, character_index);

    // Check left diagonal
    if !((left_top == 'M' && right_bottom == 'S') ||
       (left_top == 'S' && right_bottom == 'M')) {
        return 0;
    }

    // Check right diagonal
    if !((right_top == 'M' && left_bottom == 'S') ||
       (right_top == 'S' && left_bottom == 'M')) {
        return 0;
    }

    1
}

fn check_left_top(split_data: &Vec<&str>, row_index: usize, character_index: usize) -> char {
    if row_index < 1 || character_index < 1 {
        return '.';
    }

    split_data[row_index - 1].chars().nth(character_index - 1).unwrap()
}

fn check_right_top(split_data: &Vec<&str>, row_index: usize, character_index: usize) -> char {
    if row_index < 1 || character_index > split_data[0].len() - 2 {
        return '.';
    }

    split_data[row_index - 1].chars().nth(character_index + 1).unwrap()
}

fn check_left_bottom(split_data: &Vec<&str>, row_index: usize, character_index: usize) -> char {
    if row_index > split_data.len() - 2 || character_index < 1 {
        return '.';
    }

    split_data[row_index + 1].chars().nth(character_index - 1).unwrap()
}

fn check_right_bottom(split_data: &Vec<&str>, row_index: usize, character_index: usize) -> char {
    if row_index > split_data.len() - 2 || character_index > split_data[0].len() - 2 {
        return '.';
    }

    split_data[row_index + 1].chars().nth(character_index + 1).unwrap()
}
