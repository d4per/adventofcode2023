use std::collections::{HashMap, HashSet};
use std::env::current_dir;
use Location::{Outside, Path, Unknown};
use crate::Location::Inside;


/*
| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
 */
fn main() {

    let char_map: HashMap<char, ((i64, i64), (i64, i64))> = HashMap::from_iter([
        ('|', ((0,-1), (0, 1))),
        ('-', ((-1,0), (1, 0))),
        ('L', ((0,-1), (1, 0))),
        ('J', ((0,-1), (-1, 0))),
        ('7', ((0,1), (-1, 0))),
        ('F', ((0,1), (1, 0))),
    ]);

    let lines: Vec<&str> = include_str!("input.txt").lines().collect();
    let width = lines[0].len();
    let height = lines.len();

    //hitta S
    let mut spos = (0i64, 0i64);
    for (y, line) in lines.iter().enumerate() {
        match line.find("S") {
            None => {}
            Some(x) => {
                spos = (x as i64, y as i64);
            }
        }
    }

    let mut first_direction = (0,0);
    //hitta startriktning
    for dy in -1i64..=1 {
        for dx in -1i64..=1 {
            let c = lines[(spos.1 + dy) as usize].chars().nth((spos.0 + dx) as usize);
            match c {
                None => {}
                Some(cc) if char_map.contains_key(&cc) => {
                    let directions = char_map.get(&cc);
                    match directions {
                        None => {}
                        Some((d1, d2)) if *d1 == (-dx, -dy) || *d2 == (-dx, -dy) => {
                            first_direction = (dx, dy);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    let mut current_pos = spos;
    let mut current_dir = first_direction;

    println!("{:?}  {:?}", current_pos, current_dir);

    let mut count = 0;
    let mut path: HashSet<(i64, i64)> = HashSet::new();
    loop {
        count += 1;

        path.insert(current_pos.clone());

        current_pos.0 += current_dir.0;
        current_pos.1 += current_dir.1;

        let c = lines[(current_pos.1) as usize].chars().nth((current_pos.0) as usize);
        println!("{:?} {:?} {:?}",c.unwrap(), &current_pos, current_dir);
        if c.unwrap() == 'S' {
            println!("SSSSSSSSSSSS");
            break;
        }
        let dirs = char_map.get(&c.unwrap());
        let new_dir :Option<&(i64, i64)> = match dirs {
            None => {
                None
            }
            Some((d1, d2)) if *d1 == (-current_dir.0, -current_dir.1) => {
                Some(d2)
            }
            Some((d1, d2)) if *d2 == (-current_dir.0, -current_dir.1) => {
                Some(d1)
            }
            _ => {
                None
            }
        };

        current_dir.0 = new_dir.unwrap().0;
        current_dir.1 = new_dir.unwrap().1;
    }
    println!("part1: {count} {:?}", (count/2));

    let mut inside_count = 0;
    for (y,line) in lines.iter().enumerate() {
        let mut wall_count = 0;
        let mut last_char = ' ';
        for (x, c) in line.chars().enumerate() {
            if path.contains(&(x as i64, y as i64)) {
                match (last_char, c) {
                    ('F', 'J') => {
                        wall_count += 1;
                        last_char = ' ';
                    }
                    ('L', '7') => {
                        wall_count += 1;
                        last_char = ' ';
                    }
                    (_, '-') => {}
                    (_, '|') => {
                        wall_count += 1;
                        last_char = ' ';
                    }
                    (_, 'F') => {
                        last_char = 'F';
                    }
                    (_, 'L') => {
                        last_char = 'L';
                    }
                    (_, 'S') => {
                        wall_count += 1;
                        last_char = ' ';
                    }
                    _ => {
                        //println!("{last_char} {c}");
                        last_char = ' ';
                    }
                }
                print!("X");
            } else {
                if (wall_count & 1) == 1 {
                    inside_count += 1;
                    print!("I");
                } else {
                    print!(" ");
                }

            }
        }

        println!();
    }
    println!("part 2: {inside_count}");
    //
    // let search_map = SearchMap {
    //     map: &lines,
    //     path: &path,
    //     outside: HashSet::new(),
    //     inside: HashSet::new(),
    //     width,
    //     height
    // };
    //
    //
    // let mut sum = 0;
    // for y in 0i64..(height as i64) {
    //     for x in 0i64..(width as i64) {
    //         let loc = search_map.investiage(x, y);
    //         //println!("{:?} {:?} {:?}",loc, x, y);
    //         match loc {
    //             Inside => { print!("W")}
    //             Outside => { print!(" ") }
    //             Path => { print!("X")}
    //             Unknown => {
    //                 print!("I");
    //                 sum +=1;
    //             }
    //         }
    //     }
    //     println!();
    // }
    // println!("part2: {sum}");
    // //659
    // //659
    //377
    //367


}

struct SearchMap<'a> {
    map: &'a Vec<&'a str>,
    path: &'a HashSet<(i64, i64)>,
    outside: HashSet<&'a (i64, i64)>,
    inside: HashSet<&'a (i64, i64)>,
    width: usize,
    height: usize
}

#[derive(Debug)]
enum Location {
    Inside, Outside, Path, Unknown
}

impl<'a> SearchMap<'a> {

    fn investiage(&self, x: i64, y: i64) -> Location {
        let mut current_surface: HashSet<(i64, i64)> = HashSet::new();
        return self.investiage_internal(x, y, &mut current_surface).0;
    }
    pub(crate) fn investiage_internal(&self, x: i64, y: i64, current_surface: &mut HashSet<(i64, i64)>) -> (Location, HashSet<(i64, i64)>) {
        let key = (x, y);

        let mut tmp_location: HashSet<(i64, i64)> = HashSet::new();
        tmp_location.extend(current_surface.iter());

        if current_surface.contains(&key) {
            return (Unknown, tmp_location);
        }

        if self.path.contains(&key) {
            return (Path, tmp_location);
        }
        if x < 0 || y < 0 || x>=(self.width as i64) || y >= (self.height as i64) {
            return (Outside, tmp_location);
        }
        tmp_location.insert(key);

        for dy in -1i64..=1 {
            for dx in -1i64..=1 {
                let (loc, sub_set) = self.investiage_internal(x+dx, y+dy, &mut tmp_location);
                match loc {
                    Inside => {
                        tmp_location.extend(sub_set);
                        return (Inside, tmp_location);
                    }
                    Outside => {
                        tmp_location.extend(sub_set);
                        return (Outside, tmp_location);
                    }
                    Path => {
                        //println!("path");
                    }
                    Unknown => {
                        tmp_location.extend(sub_set);
                    }
                }
            }
        }

        (Unknown, tmp_location)
    }
}


