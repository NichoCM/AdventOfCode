use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day01.txt");

pub fn solve_part_1() -> u32 {
    let mut sum: u32 = 0;

    // There are no zeros in the data
    let pattern = ('1' ..= '9').collect::<Vec<char>>();

    INPUT.split("\n").for_each(|line| {
        let chars = line.chars();
        let rev = chars.clone().rev();
        let first = chars.clone().nth(chars.collect::<String>().find(&*pattern).unwrap()).unwrap();
        let last = rev.clone().nth(rev.collect::<String>().find(&*pattern).unwrap()).unwrap();
        let mut number = String::from(first);
        number.push(last);
        let value = number.parse::<u32>().unwrap();
        sum = sum + value;
    });
    sum
}

pub fn solve_part_2() -> u32 {
    let mut sum: u32 = 0;

    // There are no 0s or "zero"s in the data
    let pattern = ('0' ..= '9').collect::<Vec<char>>();

    INPUT.split("\n").enumerate().for_each(|(index, line)| {
        // Replace the text numbers to numeric numbers
        let mut start_result = replace_numeric_value(String::from(line), true);
        while start_result.1 {
            start_result = replace_numeric_value(start_result.0, true);
        }
    
        let mut end_result = replace_numeric_value(String::from(line), false);
        while end_result.1 {
            end_result = replace_numeric_value(end_result.0, false)
        }

        let start_chars = start_result.0.chars();
        let end_chars = end_result.0.chars().rev();
        let first = start_chars.clone().nth(start_chars.collect::<String>().find(&*pattern).unwrap()).unwrap();
        let last = end_chars.clone().nth(end_chars.collect::<String>().find(&*pattern).unwrap()).unwrap();
        let mut number = String::from(first);
        println!("{}: {} {} {}", index, line, first, last);
        number.push(last);
        let value = number.parse::<u32>().unwrap();

        sum = sum + value;
    });
    sum
}

/**
 * Replace the first numeric value found with it's number representation
 */
fn replace_numeric_value(value: String, from_start: bool) -> (String, bool) {
    let dict = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let mut position: Option<(usize, (&str, &str))> = None;
    for (k, v) in dict {

        position = match if from_start { value.find(k) } else { value.rfind(k) } {
            Some(pos) => {
                match position {
                    None => Some((pos, (k, v))),
                    Some(current) => {
                        if if from_start { pos < current.0 } else { pos > current.0 } {
                            Some((pos, (k, v)))
                        } else {
                            position
                        }
                    }
                }
            }
            None => position
        }
    };

    match position {
        Some(pos) => (value.replacen(pos.1.0, pos.1.1, 1), true),
        None => (value, false)
    }
}