use std::collections::VecDeque;
#[derive(Hash, Eq, PartialEq, Clone)]
struct Lens {
    label: String,
    focal_length: u64
}

impl Lens {
    fn create(label: String, focal_length: u64) -> Lens {
        Lens {
            label,
            focal_length,
        }
    }
}


struct LensBox {
    lens_queue: VecDeque<Lens>,

}

impl LensBox {
    fn new() -> LensBox {
        LensBox {
            lens_queue: VecDeque::new(),
        }
    }
}


fn main() {
    //get_hash("HASH".to_string());

    let line = include_str!("input.txt");
    let tokens: Vec<String> = line.split(",").map(|s| s.to_string()).collect();

    let mut hash_sum = 0;
    for token in tokens.iter() {
        let hash = get_hash(token);
        hash_sum += hash;
    }

    println!("part 1: {hash_sum}");

    let mut boxes : Vec<LensBox> = Vec::new();
    for i in 0..256 {
        boxes.push(LensBox::new());
    }

    for token in tokens.iter() {
        let (split, is_dash) = if token.contains("-") {
            (token.split("-"), true)
        } else {
            (token.split("="), false)
        };

        let split_vec: Vec<&str> = split.collect();
        let label = split_vec[0].to_string();
        let focal_length: u64 = if split_vec.len() > 1 && !split_vec[1].is_empty() {
            split_vec[1].parse::<u64>().unwrap()
        } else {
            0
        };
        let box_num = get_hash(&label);

        if is_dash {
            let b = &mut boxes[box_num as usize];
            for (index, bb) in b.lens_queue.iter().enumerate() {
                if bb.label == label.to_string() {
                    b.lens_queue.remove(index);
                    break;
                }
            }
        } else {
            let b = &mut boxes[box_num as usize];
            if b.lens_queue.is_empty() {
                b.lens_queue.push_back(Lens::create(label.to_string(), focal_length));
            } else {
               let new_lens =  Lens::create(label.to_string(), focal_length);
                let mut lenses_to_add: Vec<Lens> = Vec::new();
                let mut found = false;
                for lens in b.lens_queue.iter_mut() {
                    if lens.label == label {
                        lens.focal_length = new_lens.focal_length;
                        found = true;
                    }
                }
                if !found {
                    b.lens_queue.push_back(new_lens);
                }
            }
        }
    }

    let mut sum = 0;
    for i in 0..256 {
        let bo = &boxes[i];
        for (ii, lens) in bo.lens_queue.iter().enumerate() {
            sum += (i+1) * (ii + 1) * (lens.focal_length as usize);
        }
    }

    println!("part 2:  {sum}");
    //1753

}

fn get_hash(s: &String) -> u64 {
    let mut hash = 0;
    for c in s.chars() {
        hash += c as u64;
        hash = hash * 17;
        hash = hash & 0xff;
    }
    hash
}

