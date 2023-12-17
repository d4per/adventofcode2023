use std::cmp::max;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32
}

impl CubeSet {
    fn less_than(&self, other: &CubeSet) -> bool {
        return self.red <= other.red && self.green <= other.green && self.blue <= other.blue;
    }
}

#[derive(Debug)]
struct Game {
    id: String,
    sets: Vec<CubeSet>
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"Game (\d+)").unwrap();
    static ref SET_RE: Regex = Regex::new(r"(\d+) (blue|red|green)").unwrap();
}
impl Game {
    fn new(line: &str) -> Game {
        let parts = line.split(":").collect::<Vec<&str>>();
        
        let id = RE.captures_iter(parts[0]).next().unwrap().get(1).unwrap().as_str();

        let mut game = Game { id: id.to_string(), sets: vec![]};

        let string_sets = parts[1].split(";").collect::<Vec<&str>>();
        for string_set in string_sets {
            let mut red = 0u32;
            let mut green = 0u32;
            let mut blue = 0u32;

            for (_, [number, color]) in SET_RE.captures_iter(string_set).map(|c| c.extract()) {
                match color {
                    "red" => red= number.parse::<u32>().unwrap(),
                    "green" => green= number.parse::<u32>().unwrap(),
                    "blue" => blue= number.parse::<u32>().unwrap(),
                    _ => ()

                }
            }
            let cube_set = CubeSet { red: red, green: green, blue: blue};
            game.sets.push(cube_set);
        }

        //println!("{:?}", game);
        game

    }
}

fn main() {

    let games = include_str!("input.txt")
        .split("\n")
        .map(|e| Game::new(e))
        .collect::<Vec<Game>>();



    //12 red cubes, 13 green cubes, and 14 blue cubes
    let max_set = CubeSet { red: 12, green: 13, blue: 14};

    let mut sum_id = 0;
    for game in &games {

        let mut possible = true;
        for cube_set in &game.sets {
            if !cube_set.less_than(&max_set) {
                possible = false;
            }
        }
        if possible {
            sum_id += game.id.parse::<i32>().unwrap();
        }
    }

    println!("part1: {sum_id}");

    let mut sum = 0;
    for game in games {

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for cube_set in game.sets {
            min_red = max(min_red, cube_set.red);
            min_green = max(min_green, cube_set.green);
            min_blue = max(min_blue, cube_set.blue);
        }
        let prod = min_red * min_green * min_blue;
        sum += prod;
    }
    println!("part2: {sum}");
}

