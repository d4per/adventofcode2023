use std::collections::{BTreeMap, VecDeque};

fn main() {
    let lines: Vec<&str> = include_str!("input.txt").lines().collect();

    let mut row_sections: Vec<Vec<&str>> = Vec::new();
    let mut current_row_section: Vec<&str> = Vec::new();
    for line in lines {
        if line.is_empty() {
            row_sections.push(current_row_section);
            current_row_section = Vec::new();
        } else {
            current_row_section.push(line);
        }
    }
    row_sections.push(current_row_section);

    let mut col_sections: Vec<Vec<String>> = Vec::new();
    for row_section in &row_sections {
        let width = row_section[0].len();
        let mut col_section: Vec<String> = Vec::new();

        for x in 0..width {
            col_section.push(String::new());
            for y in 0..row_section.len() {
                let c = row_section[y].chars().nth(x).unwrap();
                col_section[x].push(c);
            }
        }
        col_sections.push(col_section);
    }



    let row_sections_string: Vec<Vec<String>> = row_sections.iter().map(|s| s.iter().map(|s2| s2.to_string()).collect()).collect();

    print(&row_sections_string[0]);
    print(&col_sections[0]);

    let debug1: Vec<(&Vec<String>, usize)> = row_sections_string.iter().map(|section| (section, find_symetry_line(section))).collect();
    let debug2: Vec<(&Vec<String>, usize)> = col_sections.iter().map(|section| (section, find_symetry_line(section))).collect();

    let debug1_count = debug1.iter().filter(|e| e.1 > 0).count();
    let debug2_count = debug2.iter().filter(|e| e.1 > 0).count();

    println!("{debug1_count} {debug2_count}");

    for ((aa, acount), (bb, bcount)) in debug1.iter().zip(debug2.iter()) {
        if *acount == 0 && *bcount == 0 {
            println!("FEEEEEL ");
            print(aa);
            print(bb);
            find_symetry_line(aa);
        }
    }

    let row_sum: usize = row_sections_string.iter().map(|section| find_symetry_line(section)).sum();
    let col_sum: usize = col_sections.iter().map(|section| find_symetry_line(section)).sum();

    let tot_sum = 100 * row_sum + col_sum;

    println!("part1:  {tot_sum}");
    //26761
    //30808
    //25960
    //30911
    //32035




}

fn print(section: &Vec<String>) {
    for line in section {
        println!("{line}");
    }
    println!();
}

fn find_symetry_line(section: &Vec<String>) -> usize {

    let mut result = BTreeMap::<String, Vec<usize>>::new();
    for (index, s) in section.iter().enumerate() {
        result.entry(s.to_string()).or_default().push(index);
    }

    let mut potential_symetry_indexes: Vec<usize> = Vec::new();
    for (s, v) in &result {
        if v.len() > 1 {
            for i in 0..(v.len() -1) {
                if v[i + 1] - v[i] == 1 {
                    let potential_symetry_index = v[i];
                    potential_symetry_indexes.push(potential_symetry_index);
                }
            }
        }
    }
    if potential_symetry_indexes.is_empty() {
        return 0;
    }
    let mut ok_symetry_indexes: Vec<usize> = Vec::new();
    for potential_symetry_index in &potential_symetry_indexes {
        let mut is_ok = true;
        for i in 0..*potential_symetry_index {
            let l1 = &section[i];
            let l2index = potential_symetry_index + (potential_symetry_index - i + 1);
            if l2index >= section.len() {
                continue;
            }
            if l1 != &section[l2index] {
                is_ok = false;
                break;
            }
        }
        if is_ok {
            ok_symetry_indexes.push(*potential_symetry_index);
        }
    }
    if ok_symetry_indexes.len() > 1 {
        println!("");
    }

    if ok_symetry_indexes.is_empty() {
        0
    } else {
        ok_symetry_indexes.iter().map(|n| n + 1).sum()
    }
}

