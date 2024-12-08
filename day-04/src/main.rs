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
    let data = String::from("..X...
.SAMX.
.A..A.
XMAS.S
.X....");

    let split_data = data.split("\n").collect::<Vec<&str>>();

    let mut result = 0;

    split_data.iter().enumerate().for_each(|(index, &current_row)| {
        current_row.chars().enumerate().for_each(|(char_index, character)| {
            if character == 'X' {
                if check_index(&split_data, index, char_index) {
                    result += 1;
                }
            }
        });
    });

    println!("{}", result);
}

fn check_index(split_data: &Vec<&str>, row_index: usize, column_index: usize) -> bool {
    const NEXT_PART: [char; 3] = ['M', 'A', 'S'];

    for (checking_char_index, &checking_char) in NEXT_PART.iter().enumerate() {
        let checking_positions = get_checking_positions_for_indices(row_index, column_index, checking_char_index);

        for position in checking_positions {

            let character = match split_data.get(position.0) {
                None => {
                    continue;
                },
                Some(&row) => {
                    match row.chars().nth(position.1) {
                        None => {
                            continue;
                        },
                        Some(character) => character,
                    }
                }
            };

            if checking_char == character {

                if &checking_char == NEXT_PART.last().unwrap() {
                    return true;
                }

                continue;
            }
        }
    }

    false
}

fn get_checking_positions_for_indices(row: usize, column: usize, checking_char: usize) -> Vec<(usize, usize)> {
    Vec::new()
}
