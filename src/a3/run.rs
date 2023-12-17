use std::collections::HashSet;

fn main() {

    let lines : Vec<&str> = include_str!("input.txt")
        .split("\n")
        .collect();

    println!("{:?}", lines);

    let mut numbers: HashSet<(usize, usize, u32)> = HashSet::new();
    let mut gear_ratio_sum = 0u32;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                digit if digit.is_digit(10) => (),
                cc => {
                    let mut gear_numbers: HashSet<(usize, usize, u32)> = HashSet::new();
                    for dy in -1i32..=1 {
                        for dx in -1i32..=1 {
                            let xx = ((x as i32) + dx) as usize;
                            let yy = ((y as i32) + dy) as usize;
                            if xx >= 0 && yy >=0 && xx < lines[0].len() && yy < lines.len() {
                                if lines[yy].as_bytes()[xx].is_ascii_digit() {
                                    let find_num = findNumber(&lines, xx, yy);
                                    numbers.insert(find_num);
                                    if cc == '*' {
                                        gear_numbers.insert(find_num);
                                    }
                                }
                            }
                        }
                    }
                    if gear_numbers.len() == 2 {
                        let gear_number_list = Vec::from_iter(gear_numbers);
                        let gear_ratio = gear_number_list[0].2 * gear_number_list[1].2;
                        gear_ratio_sum += gear_ratio;
                    }
                }
            }
        }
    }

    let sum: u32 = numbers.iter().map(|n| n.2).sum();
    println!("part1: {sum}");
    //148969
    //176337
    println!("part2: {gear_ratio_sum}");
}

fn findNumber(lines: &Vec<&str>, mut x: usize, y: usize) -> (usize, usize, u32) {
    let line = lines[y];
    while true {
        if x>0 && line.as_bytes()[x-1].is_ascii_digit() {
            x -= 1;
        } else {
            break;
        }
    }

    let substr = &line[x..];
    let only_number = substr.chars().take_while(|c| c.is_ascii_digit()).collect::<String>();

    let result = only_number.parse::<u32>().unwrap();

    (x, y, result)
}

