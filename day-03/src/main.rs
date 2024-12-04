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
    println!("============= DAY 03 =============");
    
    let data = get_input_data();
    
    let mut split_data = data.split("mul(").collect::<Vec<&str>>();
    split_data.drain(0..1);

    let mut result = 0;

    for &data_fragment in split_data.iter() {
        let multiplied_numbers = data_fragment.split(")").collect::<Vec<&str>>()[0];

        if !multiplied_numbers.contains(",") {
            continue;
        }

        let (first, second) = multiplied_numbers.split_once(",").unwrap();
        let first = first.parse::<u32>().unwrap_or(0);
        let second = second.parse::<u32>().unwrap_or(0);

        result += first * second;
    };

    println!("{}", result);
}
