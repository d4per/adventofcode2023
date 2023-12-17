use std::cmp::{min, Ordering};
use std::collections::binary_heap::Iter;
use counter::Counter;

struct Hand{
    cards: String,
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2

impl Hand {

    fn cmp2(&self, other: &Self) -> Ordering {
        //let order_str = "AKQJT98765432";

        let char_counts1 = self.cards.chars().collect::<Counter<_>>();
        let mut char_vect1: Vec<(&char, &usize)> = char_counts1.iter().collect();
        char_vect1.sort_by(|e1, e2| e2.1.cmp(e1.1));
        let mut char_vect1b = fix(&char_vect1);

        let char_counts2 = other.cards.chars().collect::<Counter<_>>();
        let mut char_vect2: Vec<(&char, &usize)> = char_counts2.iter().collect();
        char_vect2.sort_by(|e1, e2| e2.1.cmp(e1.1));
        let mut char_vect2b = fix(&char_vect2);

        for i in 0..(min(char_vect1.len(), char_vect2.len())) {
            let e1 = char_vect1[i];
            let e2 = char_vect2[i];
            let order = e1.1.cmp(&e2.1);
            if order.is_ne() {
                return order;
            }
        }

        self.cmp(other).reverse()
    }

}

fn fix<'a>(in_vec: &Vec<(&char, &usize)>) -> Vec<(&'a char, &'a usize)>{
    if in_vec.len() == 1 {
        if *in_vec[0].0 == 'J' {
            return vec![(&'A', &5)];
        } else {
            return in_vec.iter().collect();
        }
    }
    let j_entry: (&char, &usize) = in_vec.iter().filter(|e| e.0 == 'J').collect()[0];
    let first_non_j_entry: &(&char, &usize) = in_vec.iter().find(|e| e.0 != 'J').unwrap();

    let replace: Vec<(&char, &usize)> = in_vec.iter()
        .map(|e| if e == first_non_j_entry {(first_non_j_entry.0, first_non_j_entry.1 + j_entry.1) } else { e })
        .collect();


    replace
}

impl Ord for Hand {

    fn cmp(&self, other: &Self) -> Ordering {
        let order_str = "AKQJT98765432";

        let zip = self.cards.chars().zip(other.cards.chars());
        for (c1, c2) in zip {
            let i1 = order_str.find(c1).unwrap();
            let i2 = order_str.find(c2).unwrap();
            let order = i1.cmp(&i2);
            if order.is_ne() {
                return order;
            }
        }

        Ordering::Equal
    }
}

struct Bet {
    hand: Hand,
    bet: u64,

}

fn main() {


    let lines = include_str!("input.txt").lines();

    let mut bets: Vec<Bet> = Vec::new();
    for line in lines.into_iter() {
        let split: Vec<&str> = line.split(" ").collect();
        let hand = Hand { cards: split[0].to_string()};
        let bet = Bet { hand: hand, bet: split[1].parse::<u64>().unwrap()};
        bets.push(bet);
    }


    bets.sort_by(|a, b| a.hand.cmp2(&b.hand));
    let mut sum = 0u64;
    for (index, bet) in bets.iter().enumerate() {
        let rank = index +1;
        println!("{} {rank}", bet.hand.cards);
        sum += (rank as u64) * (bet.bet as u64);
    }
    println!("part1: {sum}");
}

