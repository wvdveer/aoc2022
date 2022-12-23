use std::fs;

fn main() {
    let file_path: String = String::from("data/input.txt");
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let sections: Vec<&str> = contents.split("\r\n\r\n").collect();
    let stack_lines: Vec<&str> = sections[0].split("\r\n").collect();

    let mut stacks: Vec<String> = Vec::from(["".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),
                                            "".to_string(),"".to_string(),"".to_string(),"".to_string()]);

    for slraw in stack_lines {
        let sl: Vec<char> = String::from(slraw).chars().collect();
        if sl[1] != '1' {
            for i in 1..=9 {
                let offset: usize = i * 4 - 3;
                let ch : char = sl[offset];
                if ch != ' ' {
                    print!("{}\r\n", ch);
                    stacks[i - 1] = format!("{}{}", ch, stacks[i - 1]);
                }
            }
        }
    };

    print!("At start:\r\n"); 

    for i in 1..=9 {
        print!("{}: {}\r\n", i, stacks[i - 1]);    
    }    

    let movelines: Vec<&str> = sections[1].split("\r\n").collect(); 

    for moveline in movelines {
        let parts : Vec<&str> = moveline.split(" ").collect();
        let num_to_move :usize = parts[1].to_string().parse::<usize>().unwrap();
        let srcstack : usize = parts[3].to_string().parse::<usize>().unwrap();
        let deststack :usize = parts[5].to_string().parse::<usize>().unwrap();
        print!("{} {} {} \r\n", num_to_move, srcstack, deststack);    

        //for i in 1..=num_to_move {
            let srcs: Vec<char> = stacks[srcstack - 1].chars().collect();
            let starti = stacks[srcstack - 1].len() - num_to_move;
            let endi = stacks[srcstack - 1].len();
            let crts : &str = &(stacks[srcstack - 1])[starti..endi];

            stacks[deststack -1] = format!("{}{}", &(stacks[deststack - 1]), crts);

            print!("{}\r\n", stacks[deststack - 1]);   

            stacks[srcstack - 1] = format!("{}", &(stacks[srcstack - 1])[0..starti]);

            print!("{}\r\n", stacks[srcstack - 1]);   
        //}
    }



    print!("At end:\r\n"); 

    for i in 1..=9 {
        print!("{}: {}\r\n", i, stacks[i - 1]);    
    }   



}
