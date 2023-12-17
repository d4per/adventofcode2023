use std::cmp::{max, min};
use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"Card\s*?(\d+):").unwrap();
    static ref SET_RE: Regex = Regex::new(r"(\d+) (blue|red|green)").unwrap();
}


struct Card {
    id: u32,
    facit: HashSet<u32>,
    numbers: HashSet<u32>,
    win_part1: u32,
    matches: u32,

}

impl Card {
    fn new(line: &str) -> Card {
        let parts = line.split("|").collect::<Vec<&str>>();

        let id = RE.captures_iter(parts[0]).next().unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();

        let parts2 = parts[0].split(":").collect::<Vec<&str>>();

        let facit: HashSet<u32> = HashSet::from_iter(parts2[1].split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u32>().unwrap()));
        let numbs: HashSet<u32> = HashSet::from_iter(parts[1].split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u32>().unwrap()));


        let mut win = 0;
        let mut matches = 0;
        for n in &numbs {
            if facit.contains(&n) {
                match win {
                    0 => win = 1,
                    _ => win = win *2,
                }
                matches += 1;
            }
        }
        Card {
            id: id,
            facit: facit,
            numbers: numbs,
            win_part1: win,
            matches: matches,
        }

    }
}

fn main() {


    let lines = include_str!("input.txt").lines();

    let mut sum_win = 0;

    let mut cards: Vec<Card> = Vec::new();

    for line in lines {
        let card = Card::new(line);
        sum_win += card.win_part1;
        cards.push(card);
    }

    let mut win_ids: Vec<u32> = vec![1; cards.len()];

    for (index, card) in cards.iter().enumerate() {

        let matches = card.matches;

        if matches > 0 {
            let mut end_index = (index + matches as usize) as usize;
            end_index = min(end_index + 1, win_ids.len());
            for nid in (index+1)..end_index {
                win_ids[nid] += win_ids[index];
            }
        }
    }
    println!("list {:?}", &win_ids);


    println!("part1: {sum_win}");

    let sum_list: u32 = win_ids.iter().sum();
    println!("part2: {sum_list}");

    //6227972
}


