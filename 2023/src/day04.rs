use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day04.txt");

pub fn solve_part_1() -> u32 {
    let mut sum = 0;
    build_card_list().iter().for_each(|(left, right)| {
        sum += calculate_card_points(left, right)
    });
    sum
}

pub fn solve_part_2() -> u32 {
    let mut sum = 0;
    let cards = build_card_list();
    let cache: &mut HashMap<usize, usize> = &mut HashMap::new();
    for i in 0 .. cards.len() {
        sum += process_cards(&cards, i, cache);
    }
    sum
}

/**
 * Build the card list from the input
 */
fn build_card_list() -> Vec<(Vec<String>, Vec<String>)> {
    INPUT.split("\n").map(|e| {
        let split = e.split(": ").collect::<Vec<&str>>()[1]
            .split(" | ").collect::<Vec<&str>>()
            .iter().map(|e| String::from(*e)).collect::<Vec<String>>();
        let left = split.get(0).unwrap().split(" ")
            .filter(|e| !e.is_empty()).map(|e| String::from(e)).collect::<Vec<String>>();
        let right = split.get(1).unwrap().split(" ")
            .filter(|e| !e.is_empty()).map(|e| String::from(e)).collect::<Vec<String>>();
        (left, right)
    }).collect::<Vec<(Vec<String>, Vec<String>)>>()
}

fn process_cards(cards: &Vec<(Vec<String>, Vec<String>)>, i: usize, cache: &mut HashMap<usize, usize>) -> u32 {
    let mut sum = 0;
    match cards.get(i) {
        Some((left, right)) => {
            sum += 1;
            let win_count = match cache.get(&i) {
                Some(v) => *v,
                None => {
                    let count = left.iter().filter(|e| right.contains(e)).count();
                    cache.insert(i, count);
                    count
                }
            };
            for o in 1 ..= win_count {
                sum += process_cards(cards, i + o, cache)
            }
        },
        _ => {}
    }
    sum
}

fn calculate_card_points(left: &Vec<String>, right: &Vec<String>) -> u32 {
    let matching = left.iter().filter(|e| right.contains(e));
    let c = matching.count();
    let points = if c > 0 {
        1 * i32::pow(2, (c - 1) as u32) as u32
    } else {
        0
    };
    points
}
