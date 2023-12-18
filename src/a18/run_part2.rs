

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
        ('3', (0,-1)),
        ('1', (0, 1)),
        ('2', (-1, 0)),
        ('0', (1, 0)),
    ]);

    for line in lines {
        let (_, [direction, steps_str, color]) = RE.captures(line).iter().next().unwrap().extract();
        let steps: i64 = i64::from_str_radix(&color[0..5], 16).unwrap();

        let dir = directions.get(&color[5..6].chars().nth(0).unwrap()).unwrap();
        for i in 0..steps {
            current_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);
            path.insert(current_pos);
        }


        println!("{}", path.len());

    }

    let min_y = path.iter().map(|c| c.1).min().unwrap();
    let max_y = path.iter().map(|c| c.1).max().unwrap();
    let min_x = path.iter().map(|c| c.0).min().unwrap();
    let max_x = path.iter().map(|c| c.0).max().unwrap();





    println!("part2: {:?}", path.len());
    //56923

}

