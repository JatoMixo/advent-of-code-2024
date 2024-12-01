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
    println!("============= DAY 01 =============");
    
    let data = get_input_data();

    let rows = data.split("\r\n").collect::<Vec<&str>>();

    let mut left_values = Vec::new();
    let mut right_values = Vec::new();

    rows.iter().for_each(|&row| {
        let (left, right) = row.split_once("   ").unwrap();

        left_values.push(left.parse::<i32>().unwrap());
        right_values.push(right.parse::<i32>().unwrap());
    });

    // TODO: Implement own sort function
    left_values.sort();
    right_values.sort();

    let total_distance: i32 = left_values.iter().enumerate().map(|(index, &left_value)| {
        (left_value - right_values[index]).abs()
    })
    .sum();

    println!("{}", total_distance);
}
