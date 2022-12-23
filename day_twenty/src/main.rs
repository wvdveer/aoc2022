use std::fs;

#[derive(Copy, Clone)]
struct Item {
    value: i64,
    orig_pos: i32
}

impl Item {
    fn parse(src: &str, original_position: i32) -> Item {
        let mut value: i64 = src.parse::<i64>().unwrap();
        value = value * 811589153;
        Item { value: value, orig_pos: original_position }
    }
}

fn find_by_orig(list: &Vec<Item>, orig: i32) -> usize {
    let mut now_pos = 0;
    while list[now_pos].orig_pos != orig {
        now_pos = now_pos + 1;
    } 
    return now_pos;
}

fn mix_one(list: & mut Vec<Item>, from_index: usize) {
    
    let delta = list[from_index].value;

    //let dest_index_orig = list[from_index].orig_pos;
    let temp = list[from_index].clone();  
    list.remove(from_index);

    let mut dest_index_to_wrap: i64 = (from_index as i64) + delta;
    while dest_index_to_wrap < 0 {
        dest_index_to_wrap = dest_index_to_wrap + ( list.len() as i64 * 811589153);
    }
    if dest_index_to_wrap >= list.len() as i64 {
        dest_index_to_wrap = dest_index_to_wrap % (list.len() as i64);
    }
    let mut dest_index: usize = dest_index_to_wrap as usize;

    list.insert(dest_index,temp);
}

fn mix(list: & mut Vec<Item>) {
    for orig in 1..=list.len() {
        let from_index = find_by_orig(list, orig as i32);
        mix_one(list, from_index);
    }
}

fn find_coords(list: &Vec<Item>) -> i64 {
    let mut sum = 0;
    let mut idx = 0;
    while list[idx].value != 0 {
        idx = idx + 1;
    }
    idx = (idx + 1000) % list.len();
    sum = sum + list[idx].value;
    idx = (idx + 1000) % list.len();
    sum = sum + list[idx].value;
    idx = (idx + 1000) % list.len();
    sum = sum + list[idx].value;
    return sum;
}



fn main() {
    let file_path: String = "data/input.txt".to_string();
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect(); 

    let mut count = 0;
    let mut numbers: Vec<Item> = lines.iter().map(|l|{
        count = count + 1;
        Item::parse( l, count )
    }).collect();

    for i in 0..10 {
        mix(&mut numbers);
    };

    let sum = find_coords(&numbers);

    print!("{}\r\n", sum);
}
