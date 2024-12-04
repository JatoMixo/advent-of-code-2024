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

    let result: u32 = get_fragments_between_separators(data, "do()", "don't()")
        .iter()
        .map(|fragment| {
            calculate_fragment_amount(fragment)
        })
        .sum();

    println!("{}", result);
}

fn calculate_fragment_amount(fragment: &String) -> u32 {
    let mut split_fragment = fragment.split("mul(").collect::<Vec<&str>>();
    split_fragment.drain(0..1);

    let mut result = 0;

    for &fragment_section in split_fragment.iter() {
        let multiplied_numbers = fragment_section.split(")").collect::<Vec<&str>>()[0];

        if !multiplied_numbers.contains(",") {
            continue;
        }

        let (first, second) = multiplied_numbers.split_once(",").unwrap();
        let first = first.parse::<u32>().unwrap_or(0);
        let second = second.parse::<u32>().unwrap_or(0);

        result += first * second;
    };

    result
}

// In a string, gets the fragments between two defined separators, to get fragments between don't's and do's
fn get_fragments_between_separators(string: String, start: &str, end: &str) -> Vec<String> {
    let mut split_by_end = string.split(end).collect::<Vec<&str>>();
    let mut result = vec![split_by_end[0].to_string()];

    split_by_end.drain(0..1);

    split_by_end.iter().for_each(|&fragment| {
        match fragment.split_once(start) {
            None => {}
            Some((_, final_fragment)) => {
                result.push(final_fragment.to_string());
            }
        }
    });

    result
}
