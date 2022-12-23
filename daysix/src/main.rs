use std::fs;


fn main() {
    let file_path: String = String::from("data/input.txt");
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //print!("{}\r\n", contents);  

    let msg: Vec<char> = String::from(contents).chars().collect();

    let mut i = 0;

    loop {
        let mut has_match : bool = false;
        for j in 0..=13 {
            for k in 0..=13 {
                if j != k {
                    if msg[i+j] == msg[i+k] {
                        has_match = true;
                    }
                }
            }
        }
        if ! has_match {
            print!("{}{}{}{}  {}\r\n", msg[i], msg[i+1], msg[i+2], msg[i+3], i+14 );
            break;
        }
        i = i + 1;
        if i > msg.len() - 14 {
            break;
        }
    }
}
