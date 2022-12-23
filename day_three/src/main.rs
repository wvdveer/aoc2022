use std::fs;

fn getPriority(ch: char) -> u32 {
    if ('a'..'{').contains(&ch) {
        return (ch as u32) - ('a' as u32) + 1;
    } else if ('A'..'[').contains(&ch) {
        return (ch as u32) - ('A' as u32) + 27;
    }
    return 0;
}

fn in_all_three(frs: &str, srs: &str, trs: &str) -> char {
    for  fch in frs.chars() {
        for sch in srs.chars() {
            for tch in trs.chars() {
                if ( fch == sch) && (fch == tch) {
                    return fch;
                }
            }
        }
    }
    return '-';
}


fn main() {
    let file_path: String = String::from("data/input.txt");
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    //print!("{}\n", getPriority('s'));
    //print!("{}\n", getPriority('z'));

    let mut totalPriority: u32 = 0;

    let lines: Vec<&str> = contents.split("\r\n").collect();


    let mut first_elf: String = "".to_string();
    let mut second_elf: String = "".to_string();

    let rucksack_common: Vec<u32> = lines.iter().map(|l| {
        //print!("{}\n", l);
        
        if first_elf.chars().count() == 0 {
            first_elf = l.to_string();
            return 0;
        } else if second_elf.chars().count() == 0 {
            second_elf = l.to_string();
            return 0;
        }    
        let third_elf: String = l.to_string();


        

        print!("First: {}\n", first_elf);
        print!("Second: {}\n", second_elf);
        print!("Third: {}\n", third_elf);

        let iat: char = in_all_three(&first_elf, &second_elf, &third_elf);

        let priority: u32 = getPriority(iat);

        print!("{}\n", priority);

        
        totalPriority = totalPriority + priority;

        print!("{}\n", totalPriority);


        first_elf = "".to_string();
        second_elf = "".to_string();

        return priority;
    }).collect();



    print!("{}\n", totalPriority);

}
