use std::cmp::Ordering;
use std::fs;
use std::ops::RangeInclusive;

#[derive(Clone, Copy, PartialEq)]
struct Location {
    x: i32,
    y: i32
}


impl Location {
    fn new() -> Location {
       Location{ x: 0, y: 0}
    }

    fn parse_loc(src: &str) -> Location {
        let parts: Vec<&str> = src.split("at x=").collect();
        let coords: Vec<&str> = parts[1].split(", y=").collect();
        let x = coords[0].parse::<i32>().unwrap();
        let y = coords[1].parse::<i32>().unwrap();
        Location { x: x, y: y}
    }

    fn manhattan_dist(self: &Self, other: &Self) -> i32 {
        return (self.x - other.x).abs() + (self.y - other.y).abs();
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Sensor {
    loc: Location,
    near_b: Location,
    m_dist: i32
}

impl Sensor {
    fn new() -> Sensor {
        Sensor{ loc: Location::new(), near_b: Location::new(), m_dist: 0 }
    }

    fn parse_sensor(src: &str) -> Sensor {
        let parts: Vec<&str> = src.split(": closest beacon is").collect();
        let loc = Location::parse_loc(parts[0]);
        let nb = Location::parse_loc(parts[1]);
        let md = loc.manhattan_dist(&nb);
        Sensor { loc: loc, near_b: nb, m_dist: md }
    }

    fn get_exclusion_range_for_row(self: &Self, excl_y: i32) -> Option<RangeInclusive<i32>> {
        let perp_loc: Location = Location { x: self.loc.x, y: excl_y };
        let perp_dist = self.loc.manhattan_dist(&perp_loc);
        if perp_dist > self.m_dist {
            return Option::None;
        } 
        let half_width: i32 = self.m_dist - perp_dist;
        return Some((self.loc.x - half_width)..=(self.loc.x+half_width));
    }
}

fn compare_ranges(ra: &RangeInclusive<i32>, rb: &RangeInclusive<i32>) -> Ordering {
    let a = ra.clone();
    let b = rb.clone();
    let a_min: i32 = *a.start();
    let b_min: i32 = *b.start();
    let a = ra.clone();
    let b = rb.clone();
    let a_max: i32 = *a.end();
    let b_max: i32 = *b.end();
    if a_min < b_min {
        return Ordering::Less;
    } else if a_min > b_min {
        return Ordering::Greater;
    } else if a_max < b_max {
        return Ordering::Less;
    } else if a_max > b_max {
        return Ordering::Greater;
    } else {
        return Ordering::Equal;
    }
}

fn overlaps(ra: &RangeInclusive<i32>, rb: &RangeInclusive<i32>) -> bool {
    let a = ra.clone();
    let b = rb.clone();
    let a_min: i32 = *a.start();
    let b_min: i32 = *b.start();
    let a = ra.clone();
    let b = rb.clone();
    let a_max: i32 = *a.end();
    let b_max: i32 = *b.end();
    
    return a_max >= b_min && a_min <= b_max;
}

fn cover(ra: &RangeInclusive<i32>, rb: &RangeInclusive<i32>) -> RangeInclusive<i32> {
    let a = ra.clone();
    let b = rb.clone();
    let a_min: i32 = *a.start();
    let b_min: i32 = *b.start();
    let a = ra.clone();
    let b = rb.clone();
    let a_max: i32 = *a.end();
    let b_max: i32 = *b.end();
    
    let r_min: i32 = if a_min < b_min { a_min } else { b_min };
    let r_max: i32 = if a_max > b_max { a_max } else { b_max };

    return r_min..=r_max;
}

fn range_count(ra: &RangeInclusive<i32>) -> i32 {
    let a = ra.clone();
    let a_min = a.start();
    let a = ra.clone();
    let a_max = a.end();    

    return a_max - a_min + 1;
} 

fn main() {
    let file_path: String = "data/input.txt".to_string();
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect(); 
    let mut sensors: [Sensor; 32] = [Sensor::new(); 32];

    for i in 0..32 {
        sensors[i] = Sensor::parse_sensor(lines[i]);
    }
    
    for excl_y in 0..=4000000 {
        let mut excl_ranges: Vec<RangeInclusive<i32>> = Vec::new();

        for i in 0..32 {
            let er : Option<RangeInclusive<i32>> = sensors[i].get_exclusion_range_for_row(excl_y);
            if er != None {
                excl_ranges.push(er.unwrap());
            }
        }
        excl_ranges.sort_by(|a, b| compare_ranges(a, b));

        let mut rslt:  Vec<RangeInclusive<i32>> = Vec::new();

        let mut curr_range: RangeInclusive<i32> = excl_ranges[0].clone();

        for i in 1..excl_ranges.len() {
            if overlaps(&curr_range, &excl_ranges[i]) {
                curr_range = cover(&curr_range, &excl_ranges[i]);
            } else {
                rslt.push(curr_range);
                curr_range = excl_ranges[i].clone();
            }
        }
        rslt.push(curr_range);


        if rslt.len() > 1 {
            print!("{}-{},{}", rslt[0].end(), rslt[1].start(), excl_y ); 
        }
    }
    //let mut c = 0;
    //for r in rslt {
    //    c = c + range_count(&r);
    //}

    //print!("{}\r\n", c ); 
}
