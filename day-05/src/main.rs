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
    println!("============= DAY 05 =============");

    // Approaches:
    // * Grab the rules, and store them in a hashmap that contains (number, vec of numbers that need to fo before) <==
    // then iter the array in rever order, adding the numbers that need to appear in an array and removing them as they appear
    // ================
    // * Create an ordered vec with the order of all the numbers by the correct rules and use that to check that everything is appearing in the right order
    
    let data = get_input_data();
    /*let data = String::from("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47");*/

    let (rules, updates) = data.split_once("\r\n\r\n").unwrap();

    let rules = rules.split("\r\n").collect::<Vec<&str>>();
    let updates = updates
        .split("\r\n")
        .map(|update| {
            update.split(",").map(|number| {
                number.parse::<u8>().unwrap()
            }).collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let rules_map = get_rules_map(rules);

    let result: u32 = updates.iter().map(|update| {
        match is_update_correct(&rules_map, update) {
            false => {
                let sorted_update = sort_update(&rules_map, &update);
                sorted_update[sorted_update.len() / 2] as u32
            },
            true => 0,
        }
    }).sum();

    println!("{}", result);
}

fn sort_update(rules_map: &HashMap<u8, Vec<u8>>, update: &Vec<u8>) -> Vec<u8> {

    if update.len() == 1 {
        return update.clone();
    }
    
    let mut last_element_index: usize = 0;
    
    for (page_index, &page) in update.iter().enumerate() {
        let next_pages = rules_map.get(&page);
        if next_pages.is_none() {
            last_element_index = page_index;
            break;
        }

        if next_pages.unwrap().iter().filter(|&next_page| {
            update.contains(&next_page)
        }).collect::<Vec<&u8>>().len() == 0 {
            last_element_index = page_index;
            break;
        }
    }

    let mut update_last_removed = update.clone();
    update_last_removed.remove(last_element_index);

    let mut result = sort_update(rules_map, &update_last_removed);
    result.push(update[last_element_index]);

    result
}

fn is_update_correct(rules_map: &HashMap<u8, Vec<u8>>, update: &Vec<u8>) -> bool {
    for (index, &page) in update.iter().enumerate() {
        let next_pages_in_rules = rules_map.get(&page);
        if next_pages_in_rules.is_none() {
            continue;
        }

        let previous_update_pages = &update[0..index];
        for &next in next_pages_in_rules.unwrap() {
            if previous_update_pages.contains(&next) {
                return false;
            }
        }
    }

    true
}

fn get_rules_map(rules: Vec<&str>) -> HashMap<u8, Vec<u8>> {
    
    let mut map = HashMap::new();

    rules.iter().for_each(|&rule| {
        let (first, second) = rule.split_once("|").unwrap();
        let first = first.parse::<u8>().unwrap();
        let second = second.parse::<u8>().unwrap();

        map
            .entry(first)
            .and_modify(|previous_nums: &mut Vec<u8>| previous_nums.push(second))
            .or_insert(vec![second]);
    });

    map
}
