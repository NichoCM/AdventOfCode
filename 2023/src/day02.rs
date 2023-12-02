use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day02.txt");

pub fn solve_part_1() -> u32 {

    let max_colors: HashMap<&str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let mut sum = 0;
    INPUT.split("\n").for_each(|line| {
        let line = String::from(line);
        let parts = line.split(": ").collect::<Vec<&str>>();
        let mut failed = false;
        let game = parts[0].replace("Game ", "").parse::<u32>().unwrap();
        
        let parts = parts[1].split("; ");
        // Split the individual games
        'outer: for part in parts {
            let parts = part.split(", ").collect::<Vec<&str>>();

            // Split the individual colors
            for part in parts {
                let parts = part.split(" ").collect::<Vec<&str>>();
                let count = parts[0].parse::<u32>().unwrap();
                let color = parts[1];
                if count > *max_colors.get(color).unwrap() {
                    failed = true;
                    break 'outer;
                }
            }
        }

        if !failed {
            sum += game;
        }
         
    });

    sum
}

pub fn solve_part_2() -> u32 {
    let mut sum = 0;
    INPUT.split("\n").for_each(|line| {
        let line = String::from(line);
        let parts = line.split(": ").collect::<Vec<&str>>();
        let game = parts[0].replace("Game ", "").parse::<u32>().unwrap();

        let mut maxes: HashMap<&str, u32> = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);

        // Split the individual games
        let parts = parts[1].split("; ");
        for part in parts {
            // Split the individual colors
            let parts = part.split(", ").collect::<Vec<&str>>();
            for part in parts {
                let parts = part.split(" ").collect::<Vec<&str>>();
                let count = parts[0].parse::<u32>().unwrap();
                let color = parts[1];
                if count > *maxes.get(color).unwrap() {
                    maxes.insert(color, count);
                }
            }
        }
        let power = maxes.into_values().reduce(|e, a| e * a).unwrap();
        println!("Game {} | Power: {}", game, power);
        sum += power
         
    });

    sum
}