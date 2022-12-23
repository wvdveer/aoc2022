use std::fs;
use std::collections::HashMap;


#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Location {
    row: i32,
    col: i32
}

#[derive(Clone,Copy, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East
}

impl Direction {
    fn move_one_offset(self: &Self) -> Location {
        match self {
            Self::North => Location { row: -1, col:  0 },
            Self::South => Location { row:  1, col:  0 },
            Self::West  => Location { row:  0, col: -1 },
            Self::East  => Location { row:  0, col:  1 },
        }
    }

    fn scan_area(self: &Self) -> [Location;3] {
        match self {
            Self::North => [ Location { row: -1, col: -1 }, Location { row: -1, col:  0 }, Location { row: -1, col:  1 } ],
            Self::South => [ Location { row:  1, col: -1 }, Location { row:  1, col:  0 }, Location { row:  1, col:  1 } ],
            Self::West  => [ Location { row: -1, col: -1 }, Location { row:  0, col: -1 }, Location { row:  1, col: -1 } ],
            Self::East  => [ Location { row: -1, col:  1 }, Location { row:  0, col:  1 }, Location { row:  1, col:  1 } ]
        }
    }    

    fn to_ord(self: &Self) -> i32 {
        let seq: [Direction;4] = [ Direction::North, Direction::South, Direction::West, Direction::East ];
        for i in 0..=3 {
            if seq[i] == *self {
                return i as i32;
            } 
        }
        panic!("not found!");
    }    

    fn from_ord(ord: i32) -> Direction {
        let seq: [Direction;4] = [ Direction::North, Direction::South, Direction::West, Direction::East ];
        return seq[(ord % 4) as usize];
    }

}

#[derive(Clone,Copy, PartialEq)]
struct Elf {
    curr_loc: Location,
    new_loc: Location
}

impl Elf {
    fn create( row: i32, col: i32 ) -> Elf {
        let loc = Location { row: row, col: col };
        Elf { curr_loc: loc, new_loc: loc }
    }
}

fn location_has_elf(elves: &Vec<Elf>, loc: &Location) -> bool {
    for elf in elves {
        if elf.curr_loc == *loc {
            return true;
        }
    }    
    return false;
}

fn process_movements(elves: & mut Vec<Elf>, start_dir_ord: i32) -> bool {
    let mut elf_moved = false;
    let mut proposed_counts: HashMap<Location, i32> = HashMap::new(); 
    // first half
    for elf_num in 0..elves.len() {
        let mut this_elf = elves[elf_num].clone();
        let mut nearby_elves: Vec<Location> = Vec::new();
        for scan_row_offset in -1..=1 {
            for scan_col_offset in -1..=1 {
                if scan_col_offset != 0 || scan_row_offset != 0 {
                    let scan_loc = Location { 
                        row: this_elf.curr_loc.row + scan_row_offset,
                        col: this_elf.curr_loc.col + scan_col_offset };
                    if location_has_elf(elves, &scan_loc) {
                        nearby_elves.push(Location { row: scan_row_offset, col: scan_col_offset });
                    }
                }
            }
        }
        let mut chosen_direction: Option<Direction> = Option::None;
        if nearby_elves.len() > 0 {
            for dir_extra in 0..4 {
                let dir: Direction = Direction::from_ord((start_dir_ord + dir_extra) % 4);
                let scan_area = dir.scan_area();
                let mut has_elf_in_scan: bool = false;
                for i in 0..=2 {
                    if nearby_elves.contains(&scan_area[i]) {
                        has_elf_in_scan = true;
                    }
                }
                if !has_elf_in_scan {
                    chosen_direction = Some(dir);
                    break;
                }
            } 
        }
        if chosen_direction != None {
            let move_offset: Location = chosen_direction.unwrap().move_one_offset();
            let proposed_loc: Location = Location { 
                row: this_elf.curr_loc.row + move_offset.row,
                col: this_elf.curr_loc.col + move_offset.col };
            if proposed_counts.contains_key(&proposed_loc) {
                let mut count = *(proposed_counts.get(&proposed_loc).unwrap());
                count = count + 1;
                proposed_counts.insert(proposed_loc.clone(), count);
            } else {
                proposed_counts.insert(proposed_loc.clone(), 1);
            }
            this_elf.new_loc = proposed_loc.clone();   
            elves[elf_num] = this_elf.clone();
            elf_moved = true;
        }
    }
    // second half
    for elf_num in 0..elves.len() {
        let mut this_elf = elves[elf_num].clone();        
        if this_elf.curr_loc != this_elf.new_loc {
            let count = *(proposed_counts.get(&this_elf.new_loc).unwrap());
            if count == 1 {
                this_elf.curr_loc = this_elf.new_loc.clone();
            } else {
                this_elf.new_loc = this_elf.curr_loc.clone();
            }
        }
        elves[elf_num] = this_elf.clone();
    }
    return elf_moved;
} 


fn get_bounding_box(elves: &Vec<Elf>) -> [Location;2] {
    let mut min_loc = elves[0].curr_loc;
    let mut max_loc = elves[0].curr_loc;

    for elf in elves {
        if elf.curr_loc.row < min_loc.row {
            min_loc.row = elf.curr_loc.row
        }
        if elf.curr_loc.row > max_loc.row {
            max_loc.row = elf.curr_loc.row
        }
        if elf.curr_loc.col < min_loc.col {
            min_loc.col = elf.curr_loc.col
        }
        if elf.curr_loc.col > max_loc.col {
            max_loc.col = elf.curr_loc.col
        }
    }

    return [ min_loc, max_loc ];
}

fn main() {
    let file_path: String = "data/input.txt".to_string();
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect(); 

    let mut elves: Vec<Elf> = Vec::new();
    
    for row in 0..lines.len() {
        let line = lines[row];
        for col in 0..line.len() {
            let ch = line.chars().nth(col).unwrap();
            if ch == '#' {
                let new_elf: Elf = Elf::create(row as i32, col as i32);
                elves.push(new_elf);
            }
        }    
    }

    let mut n = 1;
    while process_movements(&mut elves, (n - 1) % 4) {
        print!("Round {}\r\n", n);
        n = n + 1;
    }
    

    /* p1 let bbox = get_bounding_box(&elves);

    let area = (bbox[1].row - bbox[0].row + 1) * (bbox[1].col - bbox[0].col + 1);
    
    let empty_tiles = area - elves.len() as i32;

    print!("{}\r\n", empty_tiles); */

    print!("Final round {}\r\n", n);
}
