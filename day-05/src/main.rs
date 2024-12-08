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
    println!("============= DAY 05 =============");
    
    let data = get_input_data();

    let (rules, updates) = data.split_once("\r\n\r\n").unwrap();

    let split_rules = rules.split("\r\n").collect::<Vec<&str>>();
    let split_updates = updates.split("\r\n").collect::<Vec<&str>>();

    let ordered_rules = get_ordered_rules(rules);
}

fn get_ordered_rules(rules: Vec<&str>) -> Vec<u8> {
    let rules = rules.iter().map(|&rule| {

        let (first, second) = rule.split_once("|").unwrap();
        (first.parse::<u8>().unwrap(), second.parse::<u8>().unwrap())
    }).collect::<Vec<(u8, u8)>>();

    
}
