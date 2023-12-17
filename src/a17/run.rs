use std::collections::{HashMap, VecDeque};
use std::ops::Index;

struct Square {
    cost: u8,
    best_dist: HashMap<((i8, i8),u8), usize>,
}

impl Square {
    fn new(cost: u8) -> Square {
        Square {
            cost,
            best_dist: HashMap::new()
        }
    }
}

struct Move {
    x: i32,
    y: i32,
    direction: (i8, i8),
    current_step: u8,
    current_cost: usize
}

fn main() {


    let lines: Vec<&str> = include_str!("input.txt").lines().collect();
    let width: i32 = lines[0].len() as i32;
    let height: i32 = lines.len() as i32;
    let mut map: Vec<Square> = Vec::with_capacity((width * height) as usize);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let sq = Square::new(c.to_digit(10).unwrap() as u8);
            map.push(sq);
        }
    }

    let first_move = Move {
        x: 0,
        y: 0,
        direction: (1, 0),
        current_step: 0,
        current_cost: 0,
    };

    let mut moves: VecDeque<Move> = VecDeque::new();

    moves.push_back(first_move);

    let directions: [(i8, i8); 4] = [(-1, 0), (0,-1), (1,0), (0,1)];
    let mut directions_index: HashMap<&(i8, i8), usize> = HashMap::new();
    for (index, d) in directions.iter().enumerate() {
        directions_index.insert(&d, index);
    }

    while !moves.is_empty() {

        if moves.len() % 10000 == 0 {
            println!("{}", moves.len());
        }


        let mv = moves.pop_front().unwrap();
        if mv.current_step > 10 || mv.x < 0 || mv.x >= width || mv.y < 0 || mv.y >= height {
            //dont use this move
        } else {
            let square: &mut Square = &mut map[(mv.x + mv.y*width) as usize];
            let best_so_far = square.best_dist.entry((mv.direction, mv.current_step)).or_insert(usize::MAX);
            if *best_so_far > mv.current_cost {
                square.best_dist.insert((mv.direction, mv.current_step), mv.current_cost);

                let current_cost = mv.current_cost + (square.cost as usize);

                if mv.current_step >= 4 {
                    //create new moves
                    let di = directions_index.get(&mv.direction).unwrap();
                    let left_dir = directions[(di + directions.len() - 1) % directions.len()];
                    let left_move = Move {
                        x: mv.x + (left_dir.0 as i32),
                        y: mv.y + (left_dir.1 as i32),
                        direction: left_dir,
                        current_step: 1,
                        current_cost,
                    };
                    moves.push_back(left_move);

                    let right_dir = directions[(di + 1 + directions.len()) % directions.len()];
                    let right_move = Move {
                        x: mv.x + (right_dir.0 as i32),
                        y: mv.y + (right_dir.1 as i32),
                        direction: right_dir,
                        current_step: 1,
                        current_cost,
                    };
                    moves.push_back(right_move);
                }

                if mv.current_step < 10 {
                    let straight_dir = mv.direction;
                    let straight_move = Move {
                        x: mv.x + (straight_dir.0 as i32),
                        y: mv.y + (straight_dir.1 as i32),
                        direction: straight_dir,
                        current_step: mv.current_step + 1,
                        current_cost,
                    };
                    moves.push_back(straight_move);
                }
            }
        }
    }

    println!("part 1:  {:?}", map.last().unwrap().best_dist);

    //part2
    //889
    //894,
}

