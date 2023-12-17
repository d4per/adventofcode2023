use std::cmp::{min, Ordering};
use std::cmp::Ordering::Equal;
use std::collections::binary_heap::Iter;
use counter::Counter;

struct Hand {
    cards: String,
    fixed_cards: String,
}




//A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2

impl Hand {

    fn pairs_compare(&self, other: &Self) -> Ordering {
        let char_counts1 = self.cards.chars().collect::<Counter<_>>();
        let mut char_vect1: Vec<(&char, &usize)> = char_counts1.iter().collect();
        char_vect1.sort_by(|e1, e2| e2.1.cmp(e1.1));

        let char_counts2 = other.cards.chars().collect::<Counter<_>>();
        let mut char_vect2: Vec<(&char, &usize)> = char_counts2.iter().collect();
        char_vect2.sort_by(|e1, e2| e2.1.cmp(e1.1));

        for i in 0..(min(char_vect1.len(), char_vect2.len())) {
            let e1 = char_vect1[i];
            let e2 = char_vect2[i];
            let order = e1.1.cmp(&e2.1);
            if order.is_ne() {
                return order;
            }
        }
        return Equal;
    }

}


impl Hand {

    fn cmp(&self, other: &Self) -> Ordering {
        let order_str = "AKQT98765432J";

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
    let mut bets_hold : Vec<&Bet> = Vec::new();
    for line in lines.into_iter() {
        let split: Vec<&str> = line.split(" ").collect();
        let hand = Hand { cards: split[0].to_string(), fixed_cards: String::new()};
        let bet = Bet { hand: hand, bet: split[1].parse::<u64>().unwrap()};


        bets.push(bet);
    }
    for bet in &bets {
        bets_hold.push(&bet);
    }

    let order_str = "AKQT98765432J";

    //let char_counts2 = other.cards.chars().collect::<Counter<_>>();
    let mut sort_vec: Vec<Bet> = Vec::new();

    for mut bet in bets {
        let mut chars: Vec<char> = bet.hand.cards.chars().collect();
        let counter = bet.hand.cards.chars().collect::<Counter<_>>();
        let mut char_counts: Vec<(&char, &usize)> = counter.iter().collect();
        char_counts.sort_by_key(|e| order_str.find(*e.0).unwrap());
        char_counts.sort_by_key(|e| 100-e.1);
        let best_char_entry = char_counts.first().unwrap();
        let replace_char = match best_char_entry {
            ('J', 5) => 'J',
            ('J', n) => *char_counts[1].0,
            (c, n) => **c,
        };

        let ibet = bet.hand.cards.replace('J', replace_char.to_string().as_str());

        println!("{:?} {:?}",bet.hand.cards, ibet);
        bet.hand.fixed_cards.clear();
        bet.hand.fixed_cards.push_str(&ibet);
        sort_vec.push(bet);
    }

    sort_vec.sort_by(|a, b| cmp1(&b.hand.cards, &a.hand.cards));
    for a in sort_vec.iter() {
        println!("fs {}", a.hand.cards);
    }

    sort_vec.sort_by(|a, b| cmp2(&a.hand.fixed_cards, &b.hand.fixed_cards));

    println!("aaa   {:?}", cmp2("T5555", "32T3K"));

    let mut sum = 0u64;
    for (index, bet) in sort_vec.iter().enumerate() {
        let rank = index +1;
        println!("{} {rank} {}", bet.hand.cards, bet.hand.fixed_cards);
        sum += (rank as u64) * (bet.bet as u64);
    }
    println!("part2: {sum}");
    //250743953
    //5905
}


fn cmp1(h1: &str, h2: &str) -> Ordering {
    let order_str = "AKQT98765432J";

    let zip = h1.chars().zip(h2.chars());

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

fn cmp2(h1: &str, h2: &str) -> Ordering {
    //let order_str = "AKQJT98765432";

    let char_counts1 = h1.chars().collect::<Counter<_>>();
    let mut char_vect1: Vec<(&char, &usize)> = char_counts1.iter().collect();
    char_vect1.sort_by(|e1, e2| e2.1.cmp(e1.1));

    let char_counts2 = h2.chars().collect::<Counter<_>>();
    let mut char_vect2: Vec<(&char, &usize)> = char_counts2.iter().collect();
    char_vect2.sort_by(|e1, e2| e2.1.cmp(e1.1));

    for i in 0..(min(char_vect1.len(), char_vect2.len())) {
        let e1 = char_vect1[i];
        let e2 = char_vect2[i];
        let order = e1.1.cmp(&e2.1);
        if order.is_ne() {
            return order;
        }
    }

    Ordering::Equal
}
