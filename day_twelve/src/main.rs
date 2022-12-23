use std::fs;

#[derive(Clone)]
struct Location {
    r: usize,
    c: usize
}


fn get_check_locations(row: i32, col: i32) -> Vec<Location> {
    let mut check_locs = Vec::new();
    if row > 0 {
        check_locs.push(Location { r: (row - 1) as usize, c: col as usize});
    }
    if row < 40 {
        check_locs.push(Location { r: (row + 1) as usize, c: col as usize});
    }
    if col > 0 {
        check_locs.push(Location { r: row as usize, c: (col - 1) as usize});
    }
    if col < 166 {
        check_locs.push(Location { r: row as usize, c: (col + 1) as usize});
    }
    check_locs
}

fn main() {
    let file_path: String = String::from("data/input.txt");
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect(); 
    
    let mut start_location: Location = Location { r: 0, c: 0};
    let mut end_location: Location = Location { r: 0, c: 0};

    let mut heights: [[i32;167]; 41] = [[0;167]; 41];

    for row in 0..41 {
        let line = lines[row];
        for col in 0..167 {
            let mut ch: char = line.chars().nth(col).unwrap();
            if ch == 'S' {
                start_location = Location { r: row, c: col};
                ch = 'a';
            } 
            if ch == 'E' {
                end_location = Location { r: row, c: col};
                ch = 'z';
            } 
            let height = ((ch as u32) - ('a' as u32)) as i32;
            heights[row][col] = height;
        }
    }

    let mut path_dist: [[Option<i32>;167]; 41] = [[None;167]; 41];
    path_dist[end_location.r][end_location.c] = Some(0);

    let mut nones_count = 41 * 167 - 1;
    let mut last_nones_count = 41 * 167;
    let mut shortest: i32 = last_nones_count;

    while nones_count < last_nones_count {
        last_nones_count = nones_count;
        nones_count = 0;
        for row in 0..41 {
            for col in 0..167 {
                let my_height = heights[row][col];
                let mut check_locs: Vec<Location> = get_check_locations(row as i32, col as i32);
                for check_loc in check_locs {
                    if path_dist[check_loc.r][check_loc.c] != None {
                        let cl_height = heights[check_loc.r][check_loc.c];
                        if cl_height <= my_height + 1 {
                            match path_dist[check_loc.r][check_loc.c] {
                                Some(pd) => {
                                    if path_dist[row][col] == None || pd + 1 < path_dist[row][col].unwrap() {
                                        path_dist[row][col] = Some(pd + 1);      
                                    }
                                },
                                _ => {}
                            }
                        }
                    }
                }
                if path_dist[row][col] == None {
                    nones_count = nones_count + 1;
                } /*else if row == start_location.r && col == start_location.c {
                    print!("dist: {}\r\n", path_dist[row][col].unwrap());
                } */
                if my_height == 0 && path_dist[row][col] != None{
                    let pd = path_dist[row][col].unwrap();
                    if pd < shortest {
                        shortest = pd;
                    }
                }
            }    
        }
        //print!("{}\r\n", nones_count);
    }

    print!("dist: {}\r\n", shortest);
}
