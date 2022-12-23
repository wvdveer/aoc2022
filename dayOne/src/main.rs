use std::fs;


fn main() {
    let file_path: String = "data/input.txt".to_string();
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let elves: Vec<&str> = contents.split("\r\n\r\n").collect();

    let mut max_cals1: i32 = 0;
    let mut max_cals2: i32 = 0;
    let mut max_cals3: i32 = 0;

    for elf in &elves {
        let lines: Vec<&str> = elf.split("\r\n").collect();
        let mut count_cals: i32 = 0;
        for line in lines {
            let cals = line.to_string().parse::<i32>().unwrap();
            count_cals = count_cals + cals;
        } 
        print!("{}\r\n", count_cals);

        if (count_cals > max_cals1) {
            max_cals3 = max_cals2;
            max_cals2 = max_cals1;
            max_cals1 = count_cals;
        } else if (count_cals > max_cals2) {
            max_cals3 = max_cals2;
            max_cals2 = count_cals;
        } else if (count_cals > max_cals3) {
            max_cals3 = count_cals;
        }
    }
       
    print!("max1: {}\r\n", max_cals1);
    print!("max2: {}\r\n", max_cals2);
    print!("max3: {}\r\n", max_cals3);

    print!("top 3 total: {}\r\n", max_cals1 + max_cals2 + max_cals3);
}
