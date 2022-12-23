use std::fs;


#[derive(Clone, Copy)]
struct Cavern {
    h: i64,
    base: i64,
    s: [i8;50000] 
}

#[derive(Clone, Copy)]
struct Location {
    x: i32,
    y: i64
}

impl Cavern {
    fn new() -> Cavern {
        let s: [i8;50000] = [0;50000];
        Cavern { h: 0, base: 0, s: s }
    }

    fn has_rock(self: &Self, loc: &Location) -> i32 {
        let cy = loc.y - self.base;
        if cy < 0 {
            return 2;
        }
        let row = self.s[cy as usize];
        let bit  = 1 << loc.x;
        return if (row & bit) == bit { 1 } else { 0 };
    }

    fn place_part_rock(self: & mut Self, loc: &Location) {
        let cy = loc.y - self.base;
        let existing_row = self.s[cy as usize];
        let bit  = 1 << loc.x; 
        let new_row = existing_row | bit;
        self.s[cy as usize] = new_row;
    }

    fn place_rock(self: & mut Self, loc: &Location, rock_type: &RockType) {
        for dy in 0..rock_type.h {
            for dx in 0..rock_type.w {
                let sp = dy * rock_type.w + dx;
                if rock_type.s[sp as usize] {
                    let rp = Location { x: loc.x + dx, y: loc.y + (dy as i64) };
                    self.place_part_rock(&rp);
                }
            }
        }
        if loc.y + (rock_type.h as i64) > self.h {
            self.h = loc.y + (rock_type.h as i64);
        }     

        if self.h - self.base > 30000 {
            for i in 0..25000 {
                self.s[i] = self.s[i+25000];
            }
            for i in 25000..50000 {
                self.s[i] = 0;
            }
            self.base = self.base + 25000;
        }
    }

}


#[derive(Clone, Copy)]
struct RockType {
    w: i32,
    h: i32,
    s: [bool;9] 
}

impl RockType {
    fn build(w: i32, h: i32, pattern: String) -> RockType {
        let mut s: [bool;9] = [false;9];
        for i in 0..pattern.len() {
            s[i] = pattern.chars().nth(i).unwrap() == '#';
        }
        return RockType { w: w, h: h, s: s};
    }

    fn is_valid_loc(self: &Self, loc: &Location, cavern: &Cavern) -> i32 {
        if loc.x < 0 {
            return 1;
        }
        if loc.x + self.w > 7 {
            return 1;
        }
        if loc.y < 0 {
            return 2;
        }
        for dy in 0..self.h {
            for dx in 0..self.w {
                let sp = dy * self.w + dx;
                if self.s[sp as usize] {
                    let rp = Location { x: loc.x + dx, y: loc.y + (dy as i64) };
                    let has_rock = cavern.has_rock(&rp);
                    if has_rock == 1 {
                        return 2;
                    }
                    if has_rock == 2 {
                        return 3;
                    }
                }
            }
        }
        return 0;
    }

    fn get_start(self: &Self, cavern: &Cavern) -> Location {
        return Location { x: 2, y: cavern.h + 3 };
    }

}

fn get_rock_types() -> [RockType;5] {
    return [ 
        RockType::build( 4, 1, "####".to_string() ),
        RockType::build( 3, 3, ".#.###.#.".to_string() ),
        RockType::build( 3, 3, "###..#..#".to_string() ),
        RockType::build( 1, 4, "####".to_string() ),
        RockType::build( 2, 2, "####".to_string() ),
    ];
}

fn main() {
    let file_path: String = "data/input.txt".to_string();
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let rock_types: [RockType; 5] = get_rock_types();
    let mut cavern: Cavern = Cavern::new();
    let mut last_cavern = 0;
    let mut last_occured: [i32;20000] = [0;20000];

    let mut next_rock_type: usize = 0;
    let mut next_jet_idx: usize = 0;

    let mut required_rocks: usize = 1000000000000;
    let mut skipped_height = 0;

    while required_rocks > 5000 {
        required_rocks = required_rocks - 1710;
        skipped_height = skipped_height + 2647;
    }
    
    for rock_count in 1..=required_rocks {
        let curr_rock_type = rock_types[next_rock_type];
        let mut rock_loc = curr_rock_type.get_start(&cavern);
        let mut rock_landed = false;

        while ! rock_landed {
            let jet = contents.chars().nth(next_jet_idx).unwrap();
            let dx = if jet == '<' { -1 } else { 1 };
            let new_loc_x = Location { x: rock_loc.x + dx, y: rock_loc.y };
            if curr_rock_type.is_valid_loc(&new_loc_x, &cavern) == 0 {
                rock_loc = new_loc_x;
            }
            let new_loc_y = Location{ x: rock_loc.x, y: rock_loc.y - 1 };
            let is_valid = curr_rock_type.is_valid_loc(&new_loc_y, &cavern);
            if is_valid == 0 {
                rock_loc = new_loc_y;
            } else {
                if is_valid < 3 {
                    cavern.place_rock(&rock_loc, &curr_rock_type);
                }
                rock_landed = true;
            }
            next_jet_idx = next_jet_idx + 1;
            if next_jet_idx == contents.len() {
                next_jet_idx = 0;
            }

        }

        next_rock_type = next_rock_type + 1;
        if next_rock_type == rock_types.len() {
            next_rock_type = 0;
        }

        if rock_count % (1710) == 0 {
            print!("{} {} {} {} {} {} \r\n", rock_count, cavern.h, cavern.h - last_cavern, cavern.s[(cavern.h - cavern.base -1) as usize], 
                next_jet_idx, rock_count as i32 - last_occured[next_jet_idx]);
            last_cavern = cavern.h;
            last_occured[next_jet_idx] = rock_count as i32;
        }
    }

    print!("{}\r\n", cavern.h + skipped_height);
}

// Firt run through results:

// 2170 3368 13 8 2654 1710
// 2180 3383 15 12 2706 1710 
// 2190 3404 21 48 2751 1710
// 2200 3423 19 56 2800 1710 
// 2210 3436 13 16 2857 1710 
// 2220 3448 12 4 2924 1710
// 2230 3467 19 16 2976 1710 
// 2240 3484 17 12 3026 1710 
// 2250 3501 17 8 3079 1710 
// 2260 3521 20 4 3130 1710

// Second run through results:

//1710 2652 2652 16 10057 1710 
//3420 5299 2647 16 10057 1710 
//5130 7946 2647 16 10057 1710 
//6840 10593 2647 16 10057 1710 


