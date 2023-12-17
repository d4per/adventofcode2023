use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct Square {
    current_shape: char,
}

impl Square {
    fn new(c: char) -> Square {
        Square {
            current_shape: c,
        }
    }
}


fn main() {


    let lines: Vec<&str> = include_str!("input.txt").lines().collect();

    let width =  lines[0].len();
    let height = lines.len();
    let mut squares_tmp: Vec<Square> = Vec::new();
    for line in lines {
        for c in line.chars() {
            squares_tmp.push(Square::new(c));
        }
    }
    let mut squares: &mut [Square] = squares_tmp.as_mut_slice();

    let mut hash_map: HashMap<u64, u32> = HashMap::new();
    let mut weight_per_iter: Vec<usize> = Vec::new();

    for i in 0..1000000000 {
        while move_direction(&mut squares, width, height, 0, 1) {
            print!("n");
        }

        while move_direction(&mut squares, width, height, 1, 0) {
            print!("w");
        }

        while move_direction(&mut squares, width, height, 0, -1) {
            print!("s");
        }

        while move_direction(&mut squares, width, height, -1, 0) {
            print!("e");
        }
        weight_per_iter.push(tot_weight(&width, &height, &mut squares));

        println!();
        let mut hasher = DefaultHasher::new();
        squares.hash(&mut hasher);
        let hash = hasher.finish();
        //println!("{i} Hash is {:x}!", );
        if i > 300 && hash_map.contains_key(&hash) {
            let last_iter = hash_map.get(&hash).unwrap();
            let current_iter = i;

            let period = current_iter - last_iter;
            //1000000000 = n * period + last_iter + m;
            //n = (1000000000 - last_iter - m) / period
            let n = (1000000000 - 1 - last_iter) / (period as u32);
            let m = (1000000000 - 1 - last_iter) % (period as u32);

            println!("{n} {m}");
            let tot_weight_bilion = weight_per_iter[(last_iter + m) as usize];
            println!("part 2: {tot_weight_bilion}");
            break;

        } else {
            hash_map.insert(hash, i);
        }
    }


    let tot_weight = tot_weight(&width, &height, &mut squares);
    println!("part 1: {tot_weight}");
    //110090

}

fn tot_weight(width: &usize, height: &usize, squares: &mut &mut [Square]) -> usize {
    let mut tot_weight = 0;
    for y in 0..*(height) {
        let row_weight_multiple = height - y;
        let mut num_rocks = 0;
        for x in 0..*width {
            let c = &squares[x + y * width];
            if c.current_shape == 'O' {
                num_rocks += 1;
            }
        }
        tot_weight += num_rocks * row_weight_multiple;
    }
    tot_weight
}

fn move_north(squares: &mut [Square], width: usize, height: usize) -> bool {
    let mut move_done = false;
    for y in 0..(height-1) {
        for x in 0..width {
            unsafe {
                let mut c = &squares[x + y * width];
                let mut c_down = &squares[x + (y + 1) * width];
                match (c.current_shape, c_down.current_shape) {
                    ('.', 'O') => {
                        squares.swap(x + y * width, x + (y + 1) * width);
                        move_done = true;
                    }
                    _ => {}
                }
            }
        }
    }
    move_done
}

fn move_direction(squares: &mut [Square], width: usize, height: usize, dx: isize, dy: isize) -> bool {
    let mut move_done = false;
    let (ystart, ystop) = match dy {
        -1 => (1, height),
        0 => (0, height),
        1 => (0, height-1),
        _ => { todo!()}
    };
    let (xstart, xstop) = match dx {
        -1 => (1, width),
        0 => (0, width),
        1 => (0, width-1),
        _ => { todo!()}
    };

    for y in ystart..ystop {
        for x in xstart..xstop {
            let mut c = &squares[x + y * width];
            //println!("{x} {y} {width} {height}");
            let mut c_down = &squares[(x as isize + dx + (y as isize + dy) * (width as isize)) as usize];
            match (c.current_shape, c_down.current_shape) {
                ('.', 'O') => {
                    squares.swap(x + y * width, (x as isize + dx + (y as isize + dy) * (width as isize)) as usize);
                    move_done = true;
                }
                _ => {}
            }
        }
    }
    move_done
}

