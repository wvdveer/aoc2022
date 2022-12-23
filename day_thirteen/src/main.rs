use std::{fs, cmp::Ordering};

#[derive(Clone,Eq)]
enum Item {
    Single(i32),
    List(Vec<Item>)
}

impl Item {

    fn compare(self: &Self, rhs: &Self) -> i32 {
        let mut left_is_list: bool = true;
        let mut right_is_list: bool = true;
        let mut li: i32 = 0;
        let mut ri: i32 = 0;
        
        if let Item::Single(lv) = self {
            li = *lv;
            left_is_list = false;
        }
        if let Item::Single(rv) = rhs {
            ri = *rv;
            right_is_list = false;
        }
        if !left_is_list && !right_is_list {
            return li - ri;
        } 

        let ll: Vec<Item> = if let Item::List(lv) = self {
            let tl = lv.clone();
            tl
        } else {
            let mut tl: Vec<Item> = Vec::new();
            tl.push(Item::Single(li));
            tl
        };
        let rl: Vec<Item> = if let Item::List(rv) = rhs {
            let tl = rv.clone();
            tl
        } else {
            let mut tl: Vec<Item> = Vec::new();
            tl.push(Item::Single(ri));
            tl
        };

        let mut idx = 0;
        let shortest_len = if ll.len() < rl.len() { ll.len() } else { rl.len() };
        while idx < shortest_len {
            let rslt = ll[idx].compare(&rl[idx]);
            if rslt != 0 {
                return rslt;
            }
            idx = idx + 1;
        };
        if ll.len() < rl.len() {
            return -1; 
        } else if ll.len() > rl.len() {
            return 1;
        } else {
            return 0;
        }    
    }

}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        let rslt = self.compare(other);
        if rslt < 0 {
            Ordering::Less
        } else if rslt > 0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[derive(Clone)]
struct StringReader {
    src: String,
    offset: usize
}

impl StringReader {

    fn char_at(self: &Self, idx: usize) -> char {
        self.src.chars().nth(idx).unwrap()
    }


    fn this_ch(self: &Self) -> char {
        return self.char_at(self.offset);
    }
   
    fn next_ch(self: & mut Self) {
        self.offset = self.offset + 1;
    }

    fn read_item_from_string(self: & mut Self) -> Item {
        if self.this_ch() == '[' {
            let mut result: Vec<Item> = Vec::new();
            self.next_ch();
            while self.this_ch() != ']' {
                let item_in_list = self.read_item_from_string();
                result.push(item_in_list);
                if self.this_ch() == ',' {
                    self.next_ch();
                } 
            }
            self.next_ch();
            return Item::List(result);
        } else {
            let mut new_offset = self.offset + 1;
            while ('0'..='9').contains(&self.char_at(new_offset)) {
                new_offset = new_offset + 1;
            }
            let the_str: &str = self.src.as_str();
            let valu: i32 = the_str[self.offset..new_offset].parse::<i32>().unwrap();
            let result = Item::Single(valu);
            self.offset = new_offset;
            return result;
        }
    } 
}

fn main() {
    let file_path: String = String::from("data/input.txt");
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let linepairs: Vec<&str> = contents.split("\r\n\r\n").collect(); 
    
    let mut pairindex = 1;
    let mut rightsum = 0;

    let mut sigs: Vec<Item> = Vec::new();

    for linepair in linepairs {
        let lines: Vec<&str> = linepair.split("\r\n").collect();
        let mut left_str: StringReader = StringReader { src: lines[0].to_string(), offset: 0 };
        let left: Item = left_str.read_item_from_string();
        let mut right_str: StringReader = StringReader { src: lines[1].to_string(), offset: 0 };
        let right: Item = right_str.read_item_from_string();


        if left.compare(&right) < 0 {
            rightsum = rightsum + pairindex;
        }
        pairindex = pairindex + 1;

        sigs.push(left);
        sigs.push(right);
    } 
    
    print!("{}\r\n", rightsum);

    let s2 = Item::Single(2);
    let s6 = Item::Single(6);

    sigs.push(s2.clone());
    sigs.push(s6.clone());

    sigs.sort();

    let mut packetindex = 1;
    for sig in sigs {
        if s2.compare(&sig) == 0 {
            print!("{}\r\n", packetindex);
        }      
        if s6.compare(&sig) == 0 {
            print!("{}\r\n", packetindex);
        } 
        packetindex = packetindex + 1;
    }

}
