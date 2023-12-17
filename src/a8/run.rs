use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use num::integer::lcm;


lazy_static! {
    static ref RE: Regex = Regex::new(r#"(\w\w\w) = \((\w\w\w), (\w\w\w)\)"#).unwrap();
    static ref SET_RE: Regex = Regex::new(r"(\d+) (blue|red|green)").unwrap();
}
fn main() {

    //

    let a: Vec<&str> = include_str!("input.txt").lines().collect();
    let steps = a[0];

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for i in 2..a.len() {
        let line = a[i];
        for (_, [a, b, c]) in RE.captures_iter(line).map(|c| c.extract()) {
            let aa = (b, c);
            map.insert(a, aa);
        }
    }

    println!("{steps}");
    /*
    let mut current = "AAA";
    let last_node = "ZZZ";

    for (index, c) in steps.repeat(1000).chars().enumerate() {
        current = match c {
            'L' => map.get(current).unwrap().0,
            'R' => map.get(current).unwrap().1,
            _ => ""
        };
        if current == last_node {
            let num_chars = index +1;
            println!("part 1: {num_chars}");
            break;
        }
    }
    */
    let mut starts: Vec<&str> = map.keys().filter(|s| s.ends_with("A")).map(|s| *s).collect();

    let mut num_vec = vec![0u64; starts.len()];

    for (index, c) in steps.repeat(1000).chars().enumerate() {
        let mut all_end = true;
        for i in 0..starts.len() {
            starts[i]  = match c {
                'L' => map.get(starts[i]).unwrap().0,
                'R' => map.get(starts[i]).unwrap().1,
                _ => ""
            };
            if starts[i].ends_with("Z") {
                if num_vec[i] == 0u64 {
                    num_vec[i] = (index + 1) as u64;
                }
            }
        }
    }

    println!("{:?}", num_vec);
    let mut prod = 1u128;
    for aa in num_vec {
        prod = lcm(prod, aa as u128);
    }
    println!("{:?}", prod);



}

