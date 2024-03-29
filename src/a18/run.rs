

//(\w) (\d) \(#(\w\w\w\w\w\w)\)

use std::collections::{HashMap, HashSet, VecDeque};
use lazy_static::lazy_static;
use regex::Regex;
lazy_static! {
    static ref RE: Regex = Regex::new(r"(\w) (\d*) \(#(\w\w\w\w\w\w)\)").unwrap();
}

fn main() {


    let lines = include_str!("input.txt").lines();

    //let id = RE.captures_iter(parts[0]).next().unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();

    let mut current_pos = (0, 0);
    let mut path: HashSet<(i32, i32)> = HashSet::new();
    path.insert(current_pos);

    let directions: HashMap<char, (i32, i32)> = HashMap::from_iter([
        ('U', (0,-1)),
        ('D', (0, 1)),
        ('L', (-1, 0)),
        ('R', (1, 0)),
    ]);

    for line in lines {
        let (_, [direction, steps_str, color]) = RE.captures(line).iter().next().unwrap().extract();
        let steps: i32 = steps_str.parse().unwrap();

        let dir = directions.get(&direction.chars().nth(0).unwrap()).unwrap();
        for i in 0..steps {
            current_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);
            path.insert(current_pos);
        }
    }

    let inside = (1, 1);
    let mut fill_queue: VecDeque<(i32, i32)> = VecDeque::new();
    fill_queue.push_back(inside);

    while !fill_queue.is_empty() {
        let coord = fill_queue.pop_front().unwrap();
        if !path.contains(&coord) {
            path.insert(coord);

            fill_queue.push_back((coord.0+1, coord.1));
            fill_queue.push_back((coord.0-1, coord.1));
            fill_queue.push_back((coord.0, coord.1+1));
            fill_queue.push_back((coord.0, coord.1-1));
        }
    }

    let min_y = path.iter().map(|c| c.1).min().unwrap();
    let max_y = path.iter().map(|c| c.1).max().unwrap();
    let min_x = path.iter().map(|c| c.0).min().unwrap();
    let max_x = path.iter().map(|c| c.0).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=(max_x/2) {
            let c = (x,y);
            if path.contains(&c) {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!();
    }




    println!("part1: {:?}", path.len());
    //56923

}

