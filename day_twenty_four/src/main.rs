use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Location {
    row: i32,
    col: i32
}

impl Location {
    fn apply(self: &Self, dir: Direction) -> Location {
        let delta = dir.move_one_offset();
        Location { row: self.row + delta.row, col: self.col + delta.col }
    }
}

#[derive(Clone,Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
    Wait
}

impl Direction {
    fn move_one_offset(self: &Self) -> Location {
        match self {
            Self::North => Location { row: -1, col:  0 },
            Self::East  => Location { row:  0, col:  1 },
            Self::South => Location { row:  1, col:  0 },
            Self::West  => Location { row:  0, col: -1 },
            Self::Wait  => Location { row:  0, col:  0 }
        }
    }

    fn to_ord(self: &Self) -> i32 {
        let seq: [Direction;5] = [ Direction::North, Direction::East, Direction::South, Direction::West, Direction::Wait ];
        for i in 0..=4 {
            if seq[i] == *self {
                return i as i32;
            } 
        }
        panic!("not found!");
    }    

    fn from_ord(ord: i32) -> Direction {
        let seq: [Direction;5] = [ Direction::North, Direction::East, Direction::South, Direction::West, Direction::Wait ];
        return seq[ord as usize];
    }

    fn from_ch(ch: char) -> Direction {
        let dirs = "^>v<".to_string();
        for i in 0..=3 {
            if ch == dirs.chars().nth(i).unwrap() {
                return Direction::from_ord(i as i32);
            }
        } 
        panic!("not found!");
    }

}

#[derive(Clone, Copy)]
struct Blizzard {
    loc: Location,
    dir: Direction
}

fn fix_bounds(loc: Location) -> Location {
    let mut new_loc = loc.clone();
    new_loc.row = ((new_loc.row + 19) % 20) + 1;
    new_loc.col = ((new_loc.col + 149) % 150) + 1;
    new_loc
}

fn in_bounds_for_player(loc: Location) -> bool {
    if loc.row == 0 && loc.col == 1 {
        return true;
    }
    if loc.row == 21 && loc.col == 150 {
        return true;
    }
    let new_loc = fix_bounds(loc);
    return new_loc == loc;
}

fn move_blizzards(blizzards: & mut Vec<Blizzard>) {
    for blizz_num in 0..blizzards.len() {
        let mut blizz = blizzards[blizz_num];
        let mut new_loc = blizz.loc.apply(blizz.dir);
        new_loc = fix_bounds(new_loc);
        blizz.loc = new_loc;
        blizzards[blizz_num].loc = new_loc;
    }
}

fn free_from_blizzards(blizzards: &Vec<Blizzard>, new_loc: &Location) -> bool {
    for blizz in blizzards {
        if blizz.loc == *new_loc {
            return false;
        }
    }
    return true;
}

fn get_possible_moves(blizzards: &Vec<Blizzard>, ploc: &Location) -> Vec<Location> {
    let mut possibles: Vec<Location> = Vec::new(); 
    for dir in 0..=4 {
        let action = Direction::from_ord(dir);
        let new_loc = ploc.apply(action);
        if in_bounds_for_player(new_loc) && free_from_blizzards(blizzards, &new_loc) {
            possibles.push(new_loc);
        }
    }
    return possibles;
}

fn solve_maze(blizzards: & mut Vec<Blizzard>, start_loc: Location, end_loc: Location) -> i32 {
    let mut minute = 0;
    let mut possible_player_locations: HashSet<Location> = HashSet::new();
    
    possible_player_locations.insert(start_loc);
    
    while ! possible_player_locations.contains(&end_loc) {
        minute = minute + 1;
        move_blizzards(blizzards);
        let mut new_player_poss : HashSet<Location> = HashSet::new();
        for ploc in possible_player_locations {
            let from_here: Vec<Location> = get_possible_moves(blizzards, &ploc);
            for loc in from_here {
                new_player_poss.insert(loc);
            }    
        }
        possible_player_locations = new_player_poss;
        print!("{} {} \r\n", minute, possible_player_locations.len());

        /*for row in 0..=21 {
            for col in 0..=151 {
                let mut ch = '.';
                let loc = Location { row: row, col: col };
                if ! free_from_blizzards(blizzards, &loc) {
                    ch = '#';
                }
                if possible_player_locations.contains(&loc) {
                    ch = 'E';
                }
                
                print!("{}", ch);
            }
            print!("\r\n");
        }*/
    } 

    return minute;
}

fn main() {
    let file_path: String = "data/input.txt".to_string();
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect();     
    let mut blizzards: Vec<Blizzard> = Vec::new();

    for row in 1..=20 {
        let line = lines[row].to_string();
        for col in 1..=150 {
            let ch = line.chars().nth(col).unwrap();
            if ch != '.' {
                let new_loc: Location = Location { row: row as i32, col: col as i32 };
                let new_dir: Direction = Direction::from_ch(ch);
                let new_blizz = Blizzard { loc: new_loc, dir: new_dir };
                blizzards.push(new_blizz);
            }
        }
    }

    let start_loc = Location { row: 0, col: 1 };
    let end_loc = Location { row: 21, col: 150 };

    let minutes1 = solve_maze(&mut blizzards, start_loc, end_loc);
    let minutes2 = solve_maze(&mut blizzards, end_loc, start_loc);
    let minutes3 = solve_maze(&mut blizzards, start_loc, end_loc);

    print!("{}\r\n", minutes1);
    print!("{}\r\n", minutes1 + minutes2 + minutes3);
}
