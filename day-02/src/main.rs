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
    let reports = data.split("\r\n").collect::<Vec<&str>>();

    let safe_reports_count: i32 = reports.iter().map(|report| {

        let levels = report
            .split(" ")
            .map(|level_str| {
                level_str.parse::<i32>().unwrap()
            }).collect::<Vec<i32>>();


        let report_safety = check_report(levels);
        match report_safety {
            false => 0,
            true => 1,
        }
    })
    .sum();

    println!("{}", safe_reports_count);
}

fn check_report(mut report: Vec<i32>) -> bool {

    let mut direction: Option<i32> = None;

    for level_index in 0..report.len() - 1 {
        if check_level(&mut report, &mut direction, level_index) {
            continue;
        }

        // Check if invalid would be solved with dampener

        let mut first_removed = report.clone();
        first_removed.remove(level_index);

        let mut second_removed = report.clone();
        second_removed.remove(level_index + 1);

        if check_report_no_dampener(first_removed) || check_report_no_dampener(second_removed) {
            return true;
        }

        return false;
    }

    true
}

fn check_report_no_dampener(report: Vec<i32>) -> bool {

    let mut direction = None;

    for level_index in 0..report.len() - 1 {
        if !check_level(&report, &mut direction, level_index) {
            return false;
        }
    }

    true
}

fn check_level(levels: &Vec<i32>, direction: &mut Option<i32>, index: usize) -> bool {
    let distance = (levels[index] - levels[index + 1]).abs();

    if distance < 1 || distance > 3 {
        return false;
    }

    let current_direction = (levels[index] - levels[index + 1]) / distance;

    match direction {
        Some(direction) => {
            if *direction != current_direction {
                return false;
            }

            true
        },
        None => {
            *direction = Some(current_direction);
            true
        }
    }
}
