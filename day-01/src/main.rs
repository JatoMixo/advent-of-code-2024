use std::io::Read;
use std::collections::HashMap;

fn get_input_data() -> String {
    let mut input_data = String::new();

    std::fs::File::open("input.txt")
        .expect("ERR: Input file doesn't exist")
        .read_to_string(&mut input_data)
        .expect("ERR: Couldn't read file contents");

    input_data
}

fn main() {
    println!("============= DAY 01 =============");
    
    let data = get_input_data();

    let rows = data.split("\r\n").collect::<Vec<&str>>();

    let mut left_values = Vec::new();
    let mut right_values = Vec::new();

    rows.iter().for_each(|&row| {
        let (left, right) = row.split_once("   ").unwrap();

        left_values.push(left.parse::<u32>().unwrap());
        right_values.push(right.parse::<u32>().unwrap());
    });

    let mut times_every_number_appears: HashMap<u32, u32> = HashMap::new();

    right_values.iter().for_each(|&value| {
        times_every_number_appears
            .entry(value)
            .and_modify(|times| *times += 1)
            .or_insert(1);
    });

    let total_distance: u32 = left_values.iter().map(|&left| {
        left * times_every_number_appears.get(&left).unwrap_or(&0u32)
    })
    .sum();

    println!("{}", total_distance);
}
