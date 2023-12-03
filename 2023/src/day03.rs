
const INPUT: &str = include_str!("../inputs/day03.txt");

pub fn solve_part_1() -> u32 {
    let lines = INPUT.split("\n").map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut sum = 0;
    let h = lines.len();
    for y in 0 .. h {
        let line = lines.get(y).unwrap();
        let w = line.len();

        let mut last_value: u32 = 0;
        let mut last_numeric = false;
        let mut start_index = 0;

        for x in 0 .. w {
            let c = line.get(x).unwrap();

            if c.is_numeric() {
                if !last_numeric {
                    start_index = x;
                }
                last_numeric = true;
                last_value = last_value * 10 + c.to_digit(10).unwrap();
            } 

            // If a number may have ended
            if !c.is_numeric() || x == w - 1 {
                if last_numeric {

                    let mut is_adj = false;
                                        
                    // Search for the adjacent symbols
                    'outer: for sy in i32::max(y as i32 - 1, 0) as usize ..= i32::min(y as i32 + 1, h as i32 - 1) as usize {
                        for sx in i32::max(start_index as i32 - 1, 0) as usize ..= i32::min(x as i32, w as i32 - 1) as usize {
                            let sc = lines.get(sy).unwrap().get(sx).unwrap();
                            if !sc.is_numeric() && *sc != '.' {
                                sum += last_value;
                                is_adj = true;
                                break 'outer;
                            }
                        }
                    }

                    start_index = 0;
                    last_value = 0;
                    last_numeric = false;
                }
            }
        }
    }

    sum
}

pub fn solve_part_2() -> u32 {
    let lines = INPUT.split("\n").map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut sum = 0;
    let h = lines.len();
    for y in 0 .. h {
        let line = lines.get(y).unwrap();
        let w = line.len();

        for x in 0 .. w {
            let c = line.get(x).unwrap();
            if *c == '*' {

                let mut values: Vec<u32> = Vec::new();
                for sy in i32::max(y as i32 - 1, 0) as usize ..= i32::min(y as i32 + 1, h as i32 - 1) as usize {
                    
                    let mut skip_until = 0;
                    
                    for sx in i32::max(x as i32 - 1, 0) as usize ..= i32::min(x as i32 + 1, w as i32 - 1) as usize {
                                              
                        // Skip until flag prevents us from double reading numbers
                        if sx < skip_until {
                            continue;
                        }
                        
                        let sc = lines.get(sy).unwrap().get(sx).unwrap();
                        if sc.is_numeric() {
                            // Go back to start of number
                            let mut dx = sx;
                            while dx > 0 {
                                if lines.get(sy).unwrap().get(dx - 1).unwrap().is_numeric() {
                                    dx -= 1;
                                } else {
                                    break;
                                }
                            }

                            // Now build the whole number
                            let mut value = 0;
                            while dx < w && lines.get(sy).unwrap().get(dx).unwrap().is_numeric() {
                                value = value * 10 + lines.get(sy).unwrap().get(dx).unwrap().to_digit(10).unwrap();
                                dx += 1;
                            }

                            values.push(value);
                            skip_until = dx;
                        }

                    }
                }

                if values.len() == 2 {
                    sum += values[0] * values[1];
                }

            }
        }
    }

    sum
}

// T0O LOW: 525479
// Correct: 527364