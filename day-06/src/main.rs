use std::io::Read;
use std::collections::{HashMap, HashSet};

fn get_input_data() -> String {
    let mut input_data = String::new();

    std::fs::File::open("input.txt")
        .expect("ERR: Input file doesn't exist")
        .read_to_string(&mut input_data)
        .expect("ERR: Couldn't read file contents");

    input_data
}

fn main() {
    println!("============= DAY 06 =============");
    
    let data = get_input_data();
    /*let data = String::from("....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...");*/

    let mut obstacles_in_rows: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut obstacles_in_columns: HashMap<u8, Vec<u8>> = HashMap::new();

    let mut current_position: (u8, u8) = (0, 0);
    let mut current_rotation = Rotation::Up;

    let mut grid_size: u8 = 0;

    data.split("\r\n")
        .collect::<Vec<&str>>()
        .iter()
        .enumerate()
        .for_each(|(row_index, &row)| {

            grid_size = row.len() as u8;

            row.chars().enumerate().for_each(|(column_index, character)| {

                match character {
                    '^' => current_position = (row_index as u8, column_index as u8),
                    '#' => {

                        // ///////////////////////
                        // TODO: Insert the obstacles in order, so that they don't need to be ordered later and several times
                        // ///////////////////////

                        // Update obstacles in rows
                        obstacles_in_rows
                            .entry(row_index as u8)
                            .and_modify(|obstacles| obstacles.push(column_index as u8))
                            .or_insert(vec![column_index as u8]);

                        // Update obstacles in columns
                        obstacles_in_columns
                            .entry(column_index as u8)
                            .and_modify(|obstacles| obstacles.push(row_index as u8))
                            .or_insert(vec![row_index as u8]);
                    },
                    _ => {},
                }
            });
        });

    let mut visited_locations: HashSet<(u8, u8)> = HashSet::new();

    loop {
        match current_rotation {
            Rotation::Up => {

                // TODO: Make it so that obstacles are already ordered from smallest to biggest
                let mut next_obstacles = obstacles_in_columns.get(&current_position.1).unwrap().iter().filter(|&obstacle| {
                    obstacle < &current_position.0
                }).collect::<Vec<&u8>>();

                if next_obstacles.is_empty() {
                    for row in 0..current_position.0 {
                        visited_locations.insert((row, current_position.1));
                    }

                    break;
                }

                next_obstacles.sort();

                let next_obstacle_row = **next_obstacles.last().unwrap();

                for row in (next_obstacle_row + 1..current_position.0).rev() {
                    visited_locations.insert((row, current_position.1));
                }

                current_position = (next_obstacle_row + 1, current_position.1);
                current_rotation = current_rotation.next();
            },
            Rotation::Down => {

                // TODO: Make it so that obstacles are already ordered from smallest to biggest
                let mut next_obstacles = obstacles_in_columns.get(&current_position.1).unwrap().iter().filter(|&obstacle| {
                    obstacle > &current_position.0
                }).collect::<Vec<&u8>>();

                if next_obstacles.is_empty() {
                    for row in current_position.0..grid_size {
                        visited_locations.insert((row, current_position.1));
                    }

                    break;
                }

                next_obstacles.sort();

                let next_obstacle_row = *next_obstacles[0];

                for row in current_position.0..next_obstacle_row {
                    visited_locations.insert((row, current_position.1));
                }

                current_position = (next_obstacle_row - 1, current_position.1);
                current_rotation = current_rotation.next();
            },
            Rotation::Right => {

                // TODO: Make it so that obstacles are already ordered from smallest to biggest
                let mut next_obstacles = obstacles_in_rows.get(&current_position.0).unwrap().iter().filter(|&obstacle| {
                    obstacle > &current_position.1
                }).collect::<Vec<&u8>>();

                if next_obstacles.is_empty() {
                    for column in current_position.1..grid_size {
                        visited_locations.insert((current_position.0, column));
                    }

                    break;
                }

                next_obstacles.sort();

                let next_obstacle_column = *next_obstacles[0];

                for column in current_position.1..next_obstacle_column {
                    visited_locations.insert((current_position.0, column));
                }

                current_position = (current_position.0, next_obstacle_column - 1);
                current_rotation = current_rotation.next();
            },
            Rotation::Left => {

                // TODO: Make it so that obstacles are already ordered from smallest to biggest
                let mut next_obstacles = obstacles_in_rows.get(&current_position.0).unwrap().iter().filter(|&obstacle| {
                    obstacle < &current_position.1
                }).collect::<Vec<&u8>>();

                if next_obstacles.is_empty() {
                    for column in 0..current_position.1 {
                        visited_locations.insert((current_position.0, column));
                    }

                    break;
                }

                next_obstacles.sort();

                let next_obstacle_column = **next_obstacles.last().unwrap();

                for column in (next_obstacle_column + 1..current_position.1).rev() {
                    visited_locations.insert((current_position.0, column));
                }

                current_position = (current_position.0, next_obstacle_column + 1);
                current_rotation = current_rotation.next();
            },
        };
    }

    let result = visited_locations.len();
    println!("{}", result);

    // 5330 ==> Too low

    // Approaches:
    // * Store in a hashmap by (row, column)
    // * Use 2 different hashmaps, one for obstacles in rows and one for columns
    // * Use a hashmap based on columns, that points to a vec of rows with obstacles
    // ^ Use 2 hashmaps, one by rows and one by columns for this? <====
}

enum Rotation {
    Up,
    Down,
    Right,
    Left,
}

impl Rotation {
    pub fn next(&self) -> Self {
        match self {
            Rotation::Up => Rotation::Right,
            Rotation::Right => Rotation::Down,
            Rotation::Down => Rotation::Left,
            Rotation::Left => Rotation::Up,
        }
    }
}
