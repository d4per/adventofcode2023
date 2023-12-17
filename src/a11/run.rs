use std::collections::{BTreeSet, HashSet};
use num::abs;

fn main() {


    let lines: Vec<&str> = include_str!("input.txt").lines().collect();
    let width = lines[0].len();
    let height = lines.len();

    let mut expand_y_set: BTreeSet<usize> = lines.iter().enumerate()
        .filter(|(index, line)| line.find("#") == None)
        .map(|(index, line)| index)
        .collect();

    let mut expand_x_set: BTreeSet<usize> = BTreeSet::new();
    for x in 0..width {
        let mut is_expander_column = true;
        for line in &lines {
            if line.chars().nth(x).unwrap() == '#' {
                is_expander_column = false;
                break;
            }
        }
        if is_expander_column {
            expand_x_set.insert(x);
        }
    }

    let mut star_pos: HashSet<(usize, usize)> = HashSet::new();
    for (y, line) in lines.iter().enumerate() {
        for x in 0..width {
            if line.chars().nth(x).unwrap() == '#' {
                let extra_x = expand_x_set.range(0..x).count() * (1000000-1);
                let extra_y = expand_y_set.range(0..y).count() * (1000000-1);
                star_pos.insert((x + extra_x, y + extra_y));
            }
        }
    }

    println!("{:?}", star_pos.len());


    let mut tot_dist: usize = 0;
    for star1 in &star_pos {
        for star2 in &star_pos {
            if star1 != star2 {
                let dx = abs((star1.0 as i64) - (star2.0 as i64));
                let dy = abs((star1.1 as i64) - (star2.1 as i64));
                let dist = (dx + dy) as usize;
                tot_dist += dist;
            }
        }
        //tot_dist += shortest_path;
    }

    tot_dist = tot_dist / 2;

    println!("part1: {tot_dist}");
}

