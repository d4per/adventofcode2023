use std::collections::LinkedList;

struct PM {
    pattern: String,
    index_of_q: Vec<usize>,
    max_q: usize,
}

impl PM {
    fn new(pattern: &str) -> PM {
        let mut index_of_q = Vec::new();
        for (i, c) in pattern.chars().enumerate() {
            if c == '?' {
                index_of_q.push(i);
            }
        }
        let maxq = index_of_q.len();

        PM {
            pattern: pattern.to_string(),
            index_of_q: index_of_q,
            max_q: maxq,
        }
    }

    fn generate_candidates(&self) -> Vec<String> {
        let max: usize = (1 << self.max_q);
        let mut result: Vec<String> = Vec::new();
        for i in 0..max {
            let mut pattern_copy = self.pattern.clone();
            for (bit_num, q_index) in self.index_of_q.iter().enumerate() {
                let bit = ((i >> bit_num) & 1) == 1;
                let c = if bit {
                    '.'
                } else {
                    '#'
                };
                let q_index_1 = q_index + 1;
                pattern_copy.replace_range(q_index..(&q_index_1), String::from(c).as_str());
            }
            result.push(pattern_copy);
        }

        result
    }

}


fn main() {
    let lines = include_str!("input.txt").lines();

    let entries: Vec<(&str, Vec<usize>)> = lines.map(|line|line.split(" ").collect::<Vec<&str>>())
        .map(|split| (split[0], split[1].split(",").into_iter().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>()))
        .collect();

    println!("{:?}", entries);

    let mut totsum: usize = 0;
    for entry in entries.iter() {
        let (pattern, numbers) = entry;
        let part2pattern = pattern.repeat(5);

        count_possibilieties(part2pattern, numbers, 0, 0);

        let pm = PM::new(part2pattern.as_str());
        let candidates = pm.generate_candidates();
        let filtered: Vec<&String> = candidates.iter().filter(|s| get_numbers(s) == *numbers).collect();


        println!("{:?}", &filtered);

        totsum += filtered.len();

    }

    println!("part1: {totsum}");
    //7047
}

fn count_possibilieties(pattern: String, numbers: &Vec<usize>, pattern_pos: usize, number_index: usize) -> usize {
    if numbers.len() >= number_index {
        return 0;
    }
    let current_num = numbers[number_index];

    let mut max_index = 0;
    for i in (number_index)..numbers.len() {
        max_index += numbers[i] + 1;
    }
    max_index = numbers.len() - max_index;

    let mut ok_count = 0usize;
    for i in pattern_pos..max_index {

        let mut ok = 1;
        for j in 0..current_num {
            let c = pattern.chars().nth(i + j).unwrap();
            if c == '?' || c == '#' {

            } else {
                ok = 0;
            }
        }
        if ok == 1 {

        }
    }

    3
}

fn get_numbers(s: &String) -> Vec<usize> {
    let mut found_sharp = true;
    let mut sharp_len = 0;
    let mut found_number: Vec<usize> = Vec::new();
    for c in s.chars() {
        match (c, found_sharp) {
            ('.', true) => {
                found_sharp = false;
                if sharp_len > 0 {
                    found_number.push(sharp_len);
                    sharp_len = 0;
                }
            }
            ('.', false) => {
            }
            ('#', true) => {
                sharp_len += 1;
            }
            ('#', false) => {
                found_sharp = true;
                sharp_len = 1;
            }

            _ => {todo!()}
        }
    }
    if sharp_len > 0 {
        found_number.push(sharp_len);
        sharp_len = 0;
    }

    found_number
}

// fn aaa<'a,'b>(current_str: &'a str, numbers: &Vec<usize>, current_number_index: usize, out_strings: &mut Vec<String>) {
//     if current_number_index == numbers.len() {
//         out_strings.push(current_str.to_string());
//         return;
//     }
//
//     let current_number = numbers[current_number_index];
//     let max_start_pos: usize = numbers.iter().skip(current_number_index + 1).sum::<usize>() + (numbers.len() - 1);
//
//     let max_dots = max_start_pos - current_str.len();
//     for dot_count in 0..max_dots {
//         let mut net_str = current_str.to_string();
//         let dots = ".".repeat(dot_count);
//         net_str.push_str(&dots);
//         let hashes = "#".repeat(current_number);
//         net_str.push_str(&hashes);
//         let net_str_str = net_str.as_str();
//         aaa(net_str_str, numbers, current_number_index + 1, out_strings);
//     }
// }


fn is_match(pattern: &str, test_string: &str) -> bool {
    let mut is_match = true;
    for (p, c) in pattern.chars().zip(test_string.chars()) {
        match (p, c) {
            ('#', '#') => {

            }
            ('#', _) => {
                is_match = false;
            }
            ('?', _) => {

            }
            ('.', '.') => {

            }
            _ => {
                is_match = false;
            }
        }
        if !is_match {
            break;
        }
    }

    is_match
}

