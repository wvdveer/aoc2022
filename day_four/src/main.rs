use std::{fs, ops::RangeInclusive};

fn parse_range(in_range: &str) -> RangeInclusive<i32> {
    let bounds: Vec<&str> = in_range.split("-").collect();
    let lower: i32 = bounds[0].to_string().parse::<i32>().unwrap();
    let upper: i32 = bounds[1].to_string().parse::<i32>().unwrap();
    return lower..=upper;
}

fn range_size(r: &RangeInclusive<i32>) -> i32 {
    return r.end() - r.start() + 1;
}

fn overlap_count(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> i32 {
    let mut oc : i32 = 0;
    let s: i32 = *r1.start();
    let e: i32 = *r1.end();

    for i in s..=e {
        if r2.contains(&i) {
            oc = oc + 1;
        }
    }

    return oc;
}



fn main() {
    let file_path: String = String::from("data/input.txt");
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect();

    let mut total_enc: i32 = 0;

    let pairs_inc: Vec<i32> = lines.iter().map(|l|{
        let pair: Vec<&str> = l.split(",").collect();
        let r1: RangeInclusive<i32> = parse_range(pair[0]);
        let r2: RangeInclusive<i32> = parse_range(pair[1]);
        let r1s: i32 = range_size(&r1);
        let r2s: i32 = range_size(&r2);
        let oc: i32 = overlap_count(&r1, &r2);

        if (oc > 0) {
            total_enc = total_enc + 1;
        }

        print!("r1: {} {} {} r2: {} {} {} oc: {} \r\n", r1.start(), r1.end(), r1s, r2.start(), r2.end(), r2s, oc);

        return 0;
    }).collect();

    print!("te: {}\r\n", total_enc);

}        
