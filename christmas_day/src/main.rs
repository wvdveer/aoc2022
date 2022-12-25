use std::fs;

fn snafu_to_int(src: &str) -> i64 {
    let digits = "=-012".to_string();
    let s = src.to_string();
    let mut rslt = 0;
    let mut base = 1;
    for n in (0..s.len()).rev() {
        let digit = s.chars().nth(n).unwrap();
        let mut digit_value = -2;
        while digits.chars().nth((digit_value + 2) as usize).unwrap() != digit {
            digit_value = digit_value + 1;
        } 
        rslt = rslt + (digit_value * base);
        base = base * 5;
    }
    return rslt;
}

fn int_to_snafu(num: i64) -> String {
    let digits = "=-012".to_string();
    let mut base_5: [i64;500] = [0;500];
    let mut n = 0;
    let mut rem = num;
    while rem > 0 {
        let mod5 = rem % 5;
        rem = (rem - mod5) / 5;
        base_5[n] = mod5;
        n = n + 1;
    }
    let places_count = n;
    for n in 0..places_count {
        if base_5[n] > 2 {
            base_5[n+1] = base_5[n+1] + 1;
            base_5[n] = base_5[n] - 5;
        }
    }
    let mut rslt: String = "".to_string();
    for n in (0..places_count).rev() {
        let digit = digits.chars().nth((base_5[n] + 2) as usize).unwrap();
        rslt.push(digit);
    }
    return rslt;
}


fn main() {
    let file_path: String = "data/input.txt".to_string();
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect();   

    let mut sum: i64 = 0;
    for line in lines {
        let val = snafu_to_int(line);
        print!("{} -> {}\r\n", line, val);
        sum = sum + val;
    }

    let sum_as_snafu = int_to_snafu(sum);

    print!("Total: {} -> {}\r\n", sum_as_snafu, sum);

    print!("Check: {} = {}", sum, snafu_to_int(&sum_as_snafu) );
}
