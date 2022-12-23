use std::fs;

#[derive(Clone, PartialEq)]
struct Location {
    x: usize,
    y: usize
}

impl Location {
    fn read(src: &str) -> Location {
        let coord_nums: Vec<&str> = src.split(",").collect();
        let x = coord_nums[0].parse::<usize>().unwrap();
        let y = coord_nums[1].parse::<usize>().unwrap();
        Location{ x: x, y: y}
    }

    fn next_towards(self: &Self, other: &Location) -> Option<Location> {
        if self.x == other.x && self.y == other.y {
            return Option::None;
        } else if self.x != other.x {
            let new_x = if self.x < other.x { self.x + 1 } else { self.x - 1 };
            return Option::Some(Location{ x: new_x, y: self.y }); 
        } else {
            let new_y = if self.y < other.y { self.y + 1 } else { self.y - 1 };
            return Option::Some(Location{ x: self.x, y: new_y }); 
        }
    }

}


fn main() {
    let file_path: String = "data/input.txt".to_string();
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut map: [[char;164];1000] = [['.';164];1000];

    let lines: Vec<&str> = contents.split("\r\n").collect(); 

    for line in lines {
        let coords: Vec<&str> = line.split(" -> ").collect();
        let mut start: Location = Location::read(coords[0]);
        let mut i = 1;
        while i < coords.len() {
            let end: Location = Location::read(coords[i]);
            let mut next_loc_opt = Option::Some(start);
            while next_loc_opt != Option::None {
                let curr_loc: Location = next_loc_opt.unwrap();
                map[curr_loc.x][curr_loc.y] = '#';
                next_loc_opt = curr_loc.next_towards(&end);
            }
            start = end;
            i = i + 1;
        }
    }

    for x in 0..1000 {
        map[x][163]= '#';
    }

    for y in 0..164 {
        for x in 0..1000 {
            print!("{}", map[x][y]);
        }
        print!("\r\n"); 
    }

    let sandpipe: Location = Location { x: 500, y: 0 };
    let mut sand_counter: i32 = 0;
    let mut sand_reached_top: bool = false;

    while ! sand_reached_top {
        let mut sand_stopped: bool = false;
               
        let mut sand_loc = sandpipe.clone();

        while ! sand_stopped {
            if map[sand_loc.x][sand_loc.y + 1] == '.' {
                sand_loc = Location { x: sand_loc.x, y: sand_loc.y + 1 };
            } else if map[sand_loc.x - 1][sand_loc.y + 1] == '.' {
                sand_loc = Location { x: sand_loc.x - 1, y: sand_loc.y + 1 };
            } else if map[sand_loc.x + 1][sand_loc.y + 1] == '.' {
                sand_loc = Location { x: sand_loc.x + 1, y: sand_loc.y + 1 };
            } else {
                sand_stopped = true;
                //print!("{},{} {} \r\n", sand_loc.x, sand_loc.y, map[sand_loc.x][sand_loc.y]);
            }
        }
        if sand_loc == sandpipe {
            sand_reached_top = true;
            sand_counter = sand_counter + 1; 
        } else {
            map[sand_loc.x][sand_loc.y] = 'o';
            sand_counter = sand_counter + 1; 
        }
    }

    for y in 0..164 {
        for x in 0..1000 {
            print!("{}", map[x][y]);
        }
        print!("\r\n"); 
    }

    print!("{sand_counter}\r\n");
}
