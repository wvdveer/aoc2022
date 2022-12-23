use std::fs;

#[derive(Copy, Clone)]
struct BackpackItem {
    worry_level: i128,
    next_monkey: usize
}

struct Monkey {
    num: usize,
    items: Vec<BackpackItem>,
    rule_operation: String,
    rule_second_term: String,
    rule_test_divisor: i32,
    true_monkey: usize,
    false_monkey: usize,
    inspections: i32
}

impl Monkey {
    fn new() -> Monkey {
        Monkey{ 
            num: 0,
            items: Vec::new(), 
            rule_operation: "".to_string(), 
            rule_second_term:  "".to_string(), 
            rule_test_divisor: 0, true_monkey: 0, false_monkey: 0, inspections: 0 }
    }

    fn create_from_def(mkdef: &str) -> Monkey {
        let lines: Vec<&str> = mkdef.split("\r\n").collect();
        let num: usize = lines[0][7..8].to_string().parse::<usize>().unwrap();
        let itemlist: Vec<&str> = lines[1][18..].split(", ").collect();
        let items: Vec<BackpackItem> = itemlist.iter().map(|wlt| {
            let wl = wlt.parse::<i128>().unwrap();
            BackpackItem{ worry_level: wl, next_monkey: num }
        }).collect();
        let op: String = lines[2][23..24].to_string();
        let st: String = lines[2][25..].to_string();
        let td: i32 = lines[3][21..].parse::<i32>().unwrap();
        let tm: usize = lines[4][29..].parse::<usize>().unwrap();
        let fm: usize = lines[5][30..].parse::<usize>().unwrap();

        Monkey{ num: num, items: items, rule_operation: op, rule_second_term: st, rule_test_divisor: td, true_monkey: tm, false_monkey: fm, inspections: 0 }
    }

    fn process_round(self: & mut Self) {
        //print!("\r\nMonkey {}:\r\n", self.num);
        let new_items : Vec<BackpackItem> = self.items.iter().map(|old_bpi|{
            let st: i128 = if self.rule_second_term == "old".to_string() { old_bpi.worry_level } else { self.rule_second_term.parse::<i128>().unwrap() }; 
            let mut new_wl: i128 = if self.rule_operation == "*".to_string() {
                old_bpi.worry_level * st
            } else {
                old_bpi.worry_level + st
            };
            let btd :i128 = self.rule_test_divisor as i128;
            let action_flag: bool = new_wl % btd == 0;
            let nm: usize = if action_flag { self.true_monkey } else { self.false_monkey };
            new_wl = new_wl % 9699690;
            self.inspections = self.inspections + 1;
            //print!("old_wl: {} newwl: {} action: {} dest: {} \r\n", old_bpi.worry_level, new_wl, action_flag, nm );
            BackpackItem{ worry_level: new_wl, next_monkey: nm }
        }).collect();
        self.items = new_items;
    }

    fn take_item(self: & mut Self, item: &BackpackItem) {
        let new_item = item.clone();
        self.items.push(new_item);
    }
}



/*fn move_items(old_monkeys: &[Monkey;8]) -> [Monkey;8] {
    let mut new_items: Vec<Vec<BackpackItem>> = Vec::new();
    for i in 0..8 {
        new_items.push(Vec::new());
    }
    for old_monkey in old_monkeys {
        let old_monkey_items: Vec<&BackpackItem> = old_monkey.items.iter().map(|i|{i}).collect();
        for old_bpi in old_monkey_items {
            let new_bpi = BackpackItem { worry_level: old_bpi.worry_level, next_monkey: usize::MAX};
            new_items[old_bpi.next_monkey].push(new_bpi);
        }
    }  
    let mut new_monkeys: [Monkey;8] = Default::default();
    for i in 0..8 {
        new_monkeys[i] = old_monkeys[i].clone();
        new_monkeys[i].items = copy_bpi_list(&new_items[i]);
    };
    new_monkeys
}*/

fn copy_bpi_list(src: &Vec<BackpackItem>) -> Vec<BackpackItem> {
    let mut new_items: Vec<BackpackItem> = Vec::new();
    for src_item in src {
        new_items.push(src_item.clone());
    }
    new_items
}

impl Clone for Monkey {
    fn clone(&self) -> Self {
        Monkey { 
            num: self.num,
            items: self.items.clone(),
            rule_operation: format!("{}", self.rule_operation),
            rule_second_term: format!("{}", self.rule_second_term),
            rule_test_divisor: self.rule_test_divisor,
            true_monkey: self.true_monkey,
            false_monkey: self.false_monkey,
            inspections: self.inspections
        }
    }
}

impl Default for Monkey {
    fn default() -> Self {
        Monkey::new()
    }
}

fn main() {
    let file_path: String = String::from("data/input.txt");
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut monkeys: [Monkey;8] = Default::default();

    let monkey_defs: Vec<&str> = contents.split("\r\n\r\n").collect(); 

    let mut multout = 1;

    for i in 0..8 {
        monkeys[i] = Monkey::create_from_def(monkey_defs[i]);
        multout = multout * monkeys[i].rule_test_divisor;
    }


    print!("{}\r\n", multout);

    for round in 0..10000 {
        for i in 0..8 {
            monkeys[i].process_round();
            let old_item_list = copy_bpi_list(&monkeys[i].items);
            for item in old_item_list {
                let new_item = item.clone();
                monkeys[item.next_monkey].take_item(&new_item);
            }
            monkeys[i].items = Vec::new();
        }
        //monkeys = move_items(&monkeys);
        print!("\r\n\r\nRound {}", round + 1);
        //for i in 0..8 {
        //    print!("\r\nMonkey {}: ", i);
        //    for item in &monkeys[i].items {
        //        print!(" {},", item.worry_level);
        //    }
        //}
    }

    print!("\r\n\r\n");

    for i in 0..8 {
        print!("{} {} \r\n", monkeys[i].num, monkeys[i].inspections);
    }

}
