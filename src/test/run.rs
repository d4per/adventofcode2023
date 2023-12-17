use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Debug, Clone)]
struct Square {
    flag1: bool,
    count: u32
}

impl Square {
    fn new() -> Square {
        Square {
            flag1: false,
            count: 0
        }
    }

    fn inc_count(&mut self) {
        self.count += 1;
    }
}

struct Map {
    squares: Vec<Square>,
    width: usize,
    height: usize
}

impl Map {
    fn new(width: usize, height: usize) -> Map {
        Map {
            width: width,
            height: height,
            squares: vec![Square::new(); (width * height) as usize],
        }
    }
    pub(crate) fn get_square(&mut self, x: usize, y: usize) -> &mut Square {
        &mut self.squares[x+y*self.width]
    }
}

lazy_static! {
    static ref MAP_X:HashMap<(u32, u32), Square> = HashMap::new();
}

fn main() {
    let mut map:HashMap<(u32, u32), Box<Square>> = HashMap::new();
    let mut squares= vec![Square::new(); 10000000];

    squares[4].count = 3232;

    let square = Square::new();
    map.insert((3, 4), Box::new(square));

    for y in 1..10 {
        map.insert((y,y), Box::new(Square::new()));
    }

    for i in 1..10 {
        for (pos, square) in map.iter_mut() {
            println!("{:?}", square);
            square.inc_count();
            square.count = 55;
        }
    }

    let mut rmap = Map::new(1000, 1000);
    let mut s1 = rmap.get_square(3, 32);

    s1.count = 3322;
    s1.flag1 = true;

    println!("Banan1: {:?}", rmap.get_square(42, 32));
    println!("Banan2: {:?}", rmap.get_square(3, 32));
}
