use std::fs;

#[derive(Clone, Copy, PartialEq)]
enum Instruction {
    TurnLeft,
    TurnRight,
    Move(i32)
}

#[derive(Clone,Copy, PartialEq)]
struct Location {
    row: i32,
    col: i32
}

#[derive(Clone,Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn move_one_offset(self: &Self) -> Location {
        match self {
            Self::North => Location { row: -1, col:  0 },
            Self::South => Location { row:  1, col:  0 },
            Self::East  => Location { row:  0, col:  1 },
            Self::West  => Location { row:  0, col: -1 },
        }
    }

    fn to_ord(self: &Self) -> i32 {
        let seq: [Direction;4] = [ Direction::East, Direction::South, Direction::West, Direction::North ];
        for i in 0..=3 {
            if seq[i] == *self {
                return i as i32;
            } 
        }
        panic!("not found!");
    }

    fn from_ord(ord: i32) -> Direction {
        let seq: [Direction;4] = [ Direction::East, Direction::South, Direction::West, Direction::North ];
        return seq[(ord % 4) as usize];
    }

    fn turn(self: &Self, instruction: Instruction) -> Direction {
        let mut ord_num = self.to_ord();
        if instruction == Instruction::TurnLeft {
            ord_num = (ord_num + 3) % 4;
        } else if instruction == Instruction::TurnRight {
            ord_num = (ord_num + 1) % 4;
        } else {
            panic!("not found!");
        }
        return Direction::from_ord(ord_num);
    }

}

#[derive(Clone,Copy, PartialEq)]
struct PlayerPosition {
    loc: Location,
    dir: Direction
}


fn parse_path(src: &str) -> Vec<Instruction> {
    let mut rslt: Vec<Instruction> = Vec::new();
    let mut curr_num: String = "".to_string(); 
    for i in 0..src.len() {
        let this_ch = src.chars().nth(i).unwrap();
        if ('0'..='9').contains(&this_ch) {
            curr_num.push(this_ch);
        } else {
            let num: i32 = curr_num.parse::<i32>().unwrap();
            rslt.push(Instruction::Move(num));
            curr_num = "".to_string();

            if this_ch == 'L' {
                rslt.push(Instruction::TurnLeft);
            } else {
                rslt.push(Instruction::TurnRight);
            }
        }
    }
    if curr_num.len() > 0 {
        let num: i32 = curr_num.parse::<i32>().unwrap();
        rslt.push(Instruction::Move(num));
    }
    return rslt;
}


fn get_start_position(map: &[[char;150];200]) -> Location {
    for col in 0..150 {
        if map[0][col] == '.' {
            return Location{ row: 0, col: col as i32 };
        }
    }
    panic!("cant find start");
}

fn out_of_bounds(map: &[[char;150];200], loc: &Location) -> bool {
    if loc.row < 0 || loc.row >= 200 || loc.col < 0 || loc.col >= 150 {
        return true;
    }
    //return map[loc.row as usize][loc.col as usize] == ' ';\
    return get_face(loc) == -1;
}

/* p1 fn resolve_out_of_bounds(map: &[[char;150];200], my_pos: &PlayerPosition) -> PlayerPosition {
    let offset: Location = my_pos.dir.move_one_offset();
    let mut next_loc: Location = my_pos.loc.clone();
    let mut this_loc: Location = my_pos.loc.clone();
    while ! out_of_bounds(map, &next_loc) {
        this_loc = next_loc;
        next_loc = Location { row: this_loc.row - offset.row, col: this_loc.col - offset.col };
    }
    return PlayerPosition { loc: this_loc, dir: my_pos.dir };
} */

fn get_face(loc: &Location) -> i32 {
    let face_map: [i32;12] = [-1,  0,  1,
                             -1,  2, -1, 
                              3,  4, -1,
                              5, -1, -1];
    let rs: i32 = (loc.row - (loc.row % 50)) / 50;
    let cs: i32 = (loc.col - (loc.col % 50)) / 50;
    let os: i32 = rs * 3 + cs;
    let face_num = face_map[os as usize];
    return face_num;
}

fn get_face_coords(face_num: i32) -> Location {
    let face_map: [Location; 6] = [
        Location { row:   0, col:  50 },
        Location { row:   0, col: 100 },
        Location { row:  50, col:  50 },
        Location { row: 100, col:   0 },
        Location { row: 100, col:  50 },
        Location { row: 150, col:   0 }
    ];
    if face_num < 0 || face_num > 5 {
        panic!("bad idx");
    }
    return face_map[face_num as usize];
}

fn cross_edge(face_num: i32, dir: Direction, ) -> [i32;4] { // facenum, dir_ord, swap, flip
    let mut new_face: [i32;4] = [-1;4];
    let mut new_dirs: [i32;4] = [-1;4];
    let mut swap: [i32;4] = [-1;4];
    let mut flip: [i32;4] = [-1;4];
    if face_num == 0 {
        new_face =  [ 1, 2, 3, 5 ];
        new_dirs = [ 0, 1, 0, 0 ];
        swap =      [ 0, 0, 0, 1 ];
        flip =      [ 0, 0, 1, 0 ];
    } else if face_num == 1 {
        new_face =  [ 4, 2, 0, 5 ];
        new_dirs = [ 2, 2, 2, 3 ];
        swap =      [ 0, 1, 0, 0 ];
        flip =      [ 1, 0, 0, 0 ];
    } else if face_num == 2 {
        new_face =  [ 1, 4, 3, 0 ];
        new_dirs = [ 3, 1, 1, 3 ];
        swap =      [ 1, 0, 1, 0 ];
        flip =      [ 0, 0, 0, 0 ];
    } else if face_num == 3 {
        new_face =  [ 4, 5, 0, 2 ];
        new_dirs = [ 0, 1, 0, 0 ];
        swap =      [ 0, 0, 0, 1 ];
        flip =      [ 0, 0, 1, 0 ];
    } else if face_num == 4 {
        new_face =  [ 1, 5, 3, 2 ];
        new_dirs = [ 2, 2, 2, 3 ];
        swap =      [ 0, 1, 0, 0 ];
        flip =      [ 1, 0, 0, 0 ];
    } else if face_num == 5 {
        new_face =  [ 4, 1, 0, 3 ];
        new_dirs = [ 3, 1, 1, 3 ];
        swap =      [ 1, 0, 1, 0 ];
        flip =      [ 0, 0, 0, 0 ];
    }; 
    let dir_ord: usize = dir.to_ord() as usize;
    let rslt: [i32;4] = [ new_face[dir_ord], new_dirs[dir_ord], swap[dir_ord], flip[dir_ord] ];
    return rslt;
}


fn resolve_out_of_bounds(map: &[[char;150];200], my_pos: &PlayerPosition) -> PlayerPosition {
    let loc = my_pos.loc.clone();
    let face_num = get_face(&loc);
    let face_row = loc.row % 50;
    let face_col = loc.col % 50;

    let next_info = cross_edge(face_num, my_pos.dir);
    let new_face = next_info[0];
    let new_dir = next_info[1];
    let swap = next_info[2];
    let flip = next_info[3];

    let mut new_row = if swap == 1 { face_col } else { face_row }; 
    let mut new_col = if swap == 1 { face_row } else { face_col }; 
    if flip == 1 {
        new_row = 49 - new_row;
        new_col = 49 - new_col;
    };
    if new_dir == 0 {
        new_col = 0;
    };
    if new_dir == 1 {
        new_row = 0;
    };
    if new_dir == 2 {
        new_col = 49;
    };
    if new_dir == 3 {
        new_row = 49;
    };
    let face_top_left: Location = get_face_coords(new_face);
    new_row = face_top_left.row + new_row;
    new_col = face_top_left.col + new_col;

    let new_loc: Location = Location { row: new_row, col: new_col };
    let new_pos: PlayerPosition = PlayerPosition { loc: new_loc, dir: Direction::from_ord(new_dir) };
    
    return new_pos;
}


fn follow_path(map: &[[char;150];200], path: &Vec<Instruction>, my_pos: & mut PlayerPosition) {
    for inst in path {
        if let Instruction::Move(distance) = inst {
            let mut i = 0;
            while i < *distance {
                let offset = my_pos.dir.move_one_offset();
                let new_loc = Location { row: my_pos.loc.row + offset.row, col: my_pos.loc.col + offset.col };
                let mut new_pos: PlayerPosition = PlayerPosition { loc: new_loc, dir: my_pos.dir };
                if out_of_bounds(map, &new_loc) {
                    new_pos = resolve_out_of_bounds(map, my_pos);
                }
                let ch: char = map[new_pos.loc.row as usize][new_pos.loc.col as usize];
                if ch != '#' {
                    *my_pos = new_pos;
                } else {
                    break;
                }
                i = i + 1;
            }
        } else {    
            let new_dir = my_pos.dir.turn(*inst);
            my_pos.dir = new_dir;
        }
    }
}

fn main() {
    let file_path: String = "data/input.txt".to_string();
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    


    let lines: Vec<&str> = contents.split("\r\n").collect(); 

    let mut map: [[char;150];200] = [[' ';150];200];

    let mut row = 0;
    let mut line = lines[row];
    while line.len() > 0 {
        for col in 0..line.len() {
            map[row][col] = line.chars().nth(col).unwrap();
        }
        row = row + 1;
        line = lines[row];
    }; 

    row = row + 1;
    let path: Vec<Instruction> = parse_path(lines[row]);

    let mut my_pos: PlayerPosition = PlayerPosition { loc: get_start_position(&map), dir: Direction::East };

    follow_path(&map, &path, &mut my_pos);

    print!("{}\r\n", (my_pos.loc.row + 1) * 1000 + (my_pos.loc. col + 1) * 4 + my_pos.dir.to_ord());
   

}
