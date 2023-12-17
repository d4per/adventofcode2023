use std::cmp::max;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Beam {
    dx: i64,
    dy: i64,
    current_x: i64,
    current_y: i64,
}

impl Beam {
    fn new(dx: i64, dy: i64, current_x: i64, current_y: i64) -> Beam {
        Beam {
            dx, dy, current_x, current_y
        }
    }
}

fn main() {


    let lines: &Vec<&str> = &include_str!("input.txt").lines().collect();
    let width = lines[0].len() as i64;
    let height = lines.len() as i64;
    let mut beams: Vec<Beam> = Vec::new();
    let start_beam = Beam::new(1,0, -1, 0);
    beams.push(start_beam);

    let visitid_squares = find_visited_squares(lines, width, height, &mut beams);
    println!("Part 1: {}", visitid_squares.len());
    //7728

    let mut max_squares = 0;
    for x in 0..width {
        let mut beams: Vec<Beam> = Vec::new();
        let start_beam = Beam::new(0,1, x, -1);
        beams.push(start_beam);
        let visitid_squares = find_visited_squares(lines, width, height, &mut beams);
        max_squares = max(max_squares, visitid_squares.len());
    }
    for x in 0..width {
        let mut beams: Vec<Beam> = Vec::new();
        let start_beam = Beam::new(0,-1, x, height);
        beams.push(start_beam);
        let visitid_squares = find_visited_squares(lines, width, height, &mut beams);
        max_squares = max(max_squares, visitid_squares.len());
    }
    for y in 0..height {
        let mut beams: Vec<Beam> = Vec::new();
        let start_beam = Beam::new(1,0, -1, y);
        beams.push(start_beam);
        let visitid_squares = find_visited_squares(lines, width, height, &mut beams);
        max_squares = max(max_squares, visitid_squares.len());
    }
    for y in 0..height {
        let mut beams: Vec<Beam> = Vec::new();
        let start_beam = Beam::new(-1,0, width, y);
        beams.push(start_beam);
        let visitid_squares = find_visited_squares(lines, width, height, &mut beams);
        max_squares = max(max_squares, visitid_squares.len());
    }

    println!("Part 2: {max_squares}");


}

fn find_visited_squares(lines: &Vec<&str>, width: i64, height: i64, beams: &mut Vec<Beam>) -> HashSet<(i64, i64)> {
    let mut visitid_squares: HashSet<(i64, i64)> = HashSet::new();
    let mut found_beams: HashSet<Beam> = HashSet::new();

    loop {
        if (beams.is_empty()) {
            break;
        }

        let mut new_beams: Vec<Beam> = Vec::new();
        let mut remove_indexes: Vec<usize> = Vec::new();
        for (index, beam) in beams.iter_mut().enumerate() {
            let new_x = beam.current_x + beam.dx;
            let new_y = beam.current_y + beam.dy;

            if new_x >= 0 && new_y >= 0 && new_x < width && new_y < height && !found_beams.contains(beam) {
                found_beams.insert(beam.clone());
                visitid_squares.insert((new_x, new_y));
                let c = lines[new_y as usize].chars().nth(new_x as usize).unwrap();

                match c {
                    '.' => {}
                    '/' => {
                        (beam.dx, beam.dy) = match (beam.dx, beam.dy) {
                            (-1, 0) => (0, 1),
                            (1, 0) => (0, -1),
                            (0, -1) => (1, 0),
                            (0, 1) => (-1, 0),
                            _ => { unimplemented!() }
                        }
                    }

                    '\\' => {
                        (beam.dx, beam.dy) = match (beam.dx, beam.dy) {
                            (-1, 0) => (0, -1),
                            (1, 0) => (0, 1),
                            (0, -1) => (-1, 0),
                            (0, 1) => (1, 0),
                            _ => { unimplemented!() }
                        }
                    }
                    '-' => {
                        match (beam.dx, beam.dy) {
                            (-1, 0) => {},
                            (1, 0) => {},
                            (0, -1) | (0, 1) => {
                                beam.dx = -1;
                                beam.dy = 0;
                                let new_beam = Beam::new(1, 0, new_x, new_y);
                                new_beams.push(new_beam);
                            }
                            _ => { unimplemented!() }
                        }
                    }
                    '|' => {
                        match (beam.dx, beam.dy) {
                            (-1, 0) | (1, 0) => {
                                beam.dx = 0;
                                beam.dy = -1;
                                let new_beam = Beam::new(0, 1, new_x, new_y);
                                new_beams.push(new_beam);
                            },
                            (0, -1) | (0, 1) => {}
                            _ => { unimplemented!() }
                        }
                    }
                    _ => { unimplemented!() }
                }
                beam.current_x = new_x;
                beam.current_y = new_y;
            } else {
                //remove beam
                remove_indexes.push(index);
            }
        }

        for (ri, remove_index) in remove_indexes.iter().enumerate() {
            beams.remove(remove_index - ri);
        }
        beams.extend(new_beams);
    }
    visitid_squares
}

