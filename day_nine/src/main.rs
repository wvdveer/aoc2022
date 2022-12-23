use std::fs;


#[derive(Copy, Clone)]
struct Location {
    x : i32,
    y : i32
}

impl Location {
    fn new() -> Location {
        Location{ x: 0, y:0 }
    }

    fn adjust(self: &mut Self, direction: &str) {
        match direction {
            "U" => self.y -= 1,
            "D" => self.y += 1,
            "L" => self.x -= 1,
            "R" => self.x += 1,
            _ => panic!("invalid!")
        }
    }

    fn follow(self: &mut Self, leader: &Location) -> bool {
        let mut move_tail_x = (leader.x - self.x).abs() > 1;
        let mut move_tail_y = (leader.y - self.y).abs() > 1;

        if move_tail_x && leader.y != self.y {
            move_tail_y = true;
        }

        if move_tail_y && leader.x != self.x {
            move_tail_x = true;
        }

        if move_tail_x && move_tail_y {
            // diag
            self.x += if leader.x > self.x { 1 } else { -1 };
            self.y += if leader.y > self.y { 1 } else { -1 }; 
        } else if move_tail_x {
            self.x += if leader.x > self.x { 1 } else { -1 };
        } else if move_tail_y {
            self.y += if leader.y > self.y { 1 } else { -1 }; 
        }
        return move_tail_x || move_tail_y;
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}


fn main() {
    let file_path: String = String::from("data/input.txt");
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect();   

    let mut head: Location = Location::new();
    let mut knots: [Location;8] = [Location::new();8];

    let mut tail: Location = Location::new();

    let mut tail_history: Vec<Location> = Vec::new();
    let store_tail = tail.clone();
    tail_history.push(store_tail);

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let distance = parts[1].to_string().parse::<i32>().unwrap();
        for _i in 0..distance {
            head.adjust(parts[0]);

            knots[0].follow(&head);
            for i in 1..8 {
                let leader = knots[i-1].clone();
                knots[i].follow(&leader);
            }
            
            let tail_moved = tail.follow(&knots[7]);

            //print!("{},{},{},{}\r\n", head.x, head.y, tail.x, tail.y);

            if tail_moved {
                if ! tail_history.contains(&tail) {
                    let store_tail = tail.clone();
                    tail_history.push(store_tail);
                }
            }
        }
        
    }

    print!("{} \r\n", tail_history.len());
}
