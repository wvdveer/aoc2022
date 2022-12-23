use std::fs;

fn check_cycle_count(cc : &i32, sss: i32, x: &i32) -> i32 {
    let ccm40 = (cc - 1) % 40; 
    if ccm40 >= x-1 && ccm40 <= x+1 {
        print!("#");
    } else {
        print!(".");
    }
    if ccm40 == 39 {
        print!("\r\n");
    }
    return 0;
}

fn main() {
    let file_path: String = String::from("data/input.txt");
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect();  
    let mut signal_strength_sum = 0;

    let mut cycle_count = 0;
    let mut x = 1;

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();  

        if parts[0] == "addx" {
            cycle_count = cycle_count + 1;
            signal_strength_sum = check_cycle_count(&cycle_count, signal_strength_sum, &x);
            cycle_count = cycle_count + 1;
            
            signal_strength_sum = check_cycle_count(&cycle_count, signal_strength_sum, &x);
            x += parts[1].to_string().parse::<i32>().unwrap();
        }
        if parts[0] == "noop" {
            cycle_count = cycle_count + 1;
            signal_strength_sum = check_cycle_count(&cycle_count, signal_strength_sum, &x);
        }
    }

    print!("\r\nsss: {}\r\n", signal_strength_sum);
}
