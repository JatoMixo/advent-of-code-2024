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
    println!("============= DAY 02 =============");

    let data = get_input_data();
    let mut reports = data.split("\n").collect::<Vec<&str>>();
    reports.pop(); // Ignore last line jump.

    let safe_reports_count: i32 = reports.iter().map(|report| {

        let levels = report
            .split(" ")
            .map(|level_str| {
                level_str.parse::<i32>().unwrap()
            }).collect::<Vec<i32>>();

        if levels[0] - levels[1] == 0 {
            return 0;
        }

        let direction = (levels[0] - levels[1]) / (levels[0] - levels[1]).abs();

        for level_index in 0..levels.len() - 1 {
            let distance = (levels[level_index] - levels[level_index + 1]).abs();

            if distance < 1 || distance > 3 {
                return 0;
            }

            let current_direction = (levels[level_index] - levels[level_index + 1]) / distance;

            if current_direction != direction {
                return 0;
            }
        }

        1
    })
    .sum();

    println!("{}", safe_reports_count);
}
