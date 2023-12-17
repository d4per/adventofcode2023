use std::arch::x86_64::_mm_stream_si128;
use std::collections::HashMap;
use std::ops::Index;

fn toDigit(str: &str) -> u32 {
    let onlyDigits = str.chars()
        .filter(|c| c.is_digit(10)).collect::<Vec<char>>();
    let first = onlyDigits[0];
    let last = onlyDigits[onlyDigits.len()-1];
    let number = (first.to_digit(10).unwrap()) * 10 + last.to_digit(10).unwrap();
    number

}

//one, two, three, four, five, six, seven, eight, and nine
fn replace(stra: &str)  -> String {
    let numbers: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut str2: String = stra.to_string();
    print!("{} ",str2);
    let mut startIndex = 0;
    let mut bestIndex = 10000;
    let mut bestNumber = None::<&str>;
    let mut bestNumberDigit = 0;

    while startIndex < str2.len() {
        for (ii, number) in numbers.iter().enumerate() {
            let index = str2[startIndex..].find(number);
            let subsrtrdebug: &str = str2[startIndex..].as_ref();
            match index {
                Some(i) if i < bestIndex => {
                    bestIndex = i;
                    bestNumber = Some(number);
                    bestNumberDigit = (ii + 1);
                }
                _ => (),
            }

        }
        match bestNumber {
            Some(nn) => {
                str2 = str2.replacen(nn, bestNumberDigit.to_string().as_str(), 1);
                startIndex = 0;
                bestNumber = None;
                bestIndex = 1000;
            },
            _ => (),
        }
        startIndex += 1;


    }
    println!("{}", str2);
    str2.to_string()
}



fn findFirstDigit(stra: &str) -> u32 {
    let searchWords = Vec::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut bestIndex: usize = 1000;
    let mut bestNumber = 0;
    for (numberStr, number) in searchWords {
        let findResult = stra.find(numberStr);
        match findResult {
            Some(pos) => {
                if pos < bestIndex {
                    bestIndex = pos;
                    bestNumber = number;
                }
            }
            _ => ()
        }
    }
    bestNumber
}

fn findLastDigit(stra: &str) -> u32 {
    let searchWords = Vec::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut bestIndex: usize = 0;
    let mut bestNumber = 0;
    for (numberStr, number) in searchWords {
        let findResult = stra.rfind(numberStr);
        match findResult {
            Some(pos) => {
                if pos >= bestIndex {
                    bestIndex = pos;
                    bestNumber = number;
                }
            }
            _ => ()
        }
    }
    bestNumber
}

fn reverseString(stra: &str) -> String {
    stra.chars().rev().collect()
}

fn main() {

    println!("{}", findFirstDigit("nine3148oneeight"));
    println!("{}", findLastDigit("nine3148oneeight"));

    let list = include_str!("input.txt")
        .split("\n")
        .map(|e| 10*findFirstDigit(e) + findLastDigit(e))
        .collect::<Vec<u32>>();

    println!("list {:?}",list);

    let sum: u32 = list.iter().sum();

    println!("{}",sum)

    //55262
    //55358



}
