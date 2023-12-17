
fn main() {


    let lines = include_str!("input.txt").lines();

    let num_strings: Vec<Vec<i64>> = lines.into_iter()
        .map(|l| l.split(" ").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .collect();

    let mut tot_sum: i64 = 0;
    for num_string in &num_strings {
        let predicted = predict_next_value(num_string);
        println!("{predicted}");
        tot_sum += predicted;
    }
    println!("part1: {tot_sum}");

    let mut tot_sum: i64 = 0;
    for num_string in &num_strings {
        let predicted = predict_prev_value(num_string);
        println!("{predicted}");
        tot_sum += predicted;
    }
    println!("part2: {tot_sum}");

}

fn predict_next_value(num_string: &Vec<i64>) -> i64 {
    let all_zero = num_string.iter().all(|n| *n == 0);
    if all_zero {
        return 0;
    }

    let last_value = num_string.last().unwrap();
    let mut delta_num_str : Vec<i64> = Vec::new();
    for i in 0..(num_string.len() -1) {
        let delta = num_string[i +1] - num_string[i];
        delta_num_str.push(delta);
    }

    return predict_next_value(&delta_num_str) + last_value;
}

fn predict_prev_value(num_string: &Vec<i64>) -> i64 {
    let all_zero = num_string.iter().all(|n| *n == 0);
    if all_zero {
        return 0;
    }

    let last_value = num_string.last().unwrap();
    let mut delta_num_str : Vec<i64> = Vec::new();
    for i in 0..(num_string.len() -1) {
        let delta = num_string[i +1] - num_string[i];
        delta_num_str.push(delta);
    }

    return num_string.first().unwrap() - predict_prev_value(&delta_num_str);
}
