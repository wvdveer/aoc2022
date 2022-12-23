use std::fs;

struct Round {
    opp_move: char,
    my_move: char
}

impl Round {

fn calc_score(self: &Self) -> i32 {
    let mut score : i32;
    match self.my_move {
        'X' => {
            match self.opp_move {
                'A' => score = 3,
                'B' => score = 1,
                'C' => score = 2,
                _ => score = 0
            }
        },
        'Y' => {
            match self.opp_move {
                'A' => score = 4,
                'B' => score = 5,
                'C' => score = 6,
                _ => score = 0
            }
        },
        'Z' => { 
            match self.opp_move {
                'A' => score = 8,
                'B' => score = 9,
                'C' => score = 7,
                _ => score = 0
            }
        },
        _ => score = 0
    };
    return score;
}

}


fn main() {
    let file_path: String = "data/input.txt".to_string();
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect();
    let rounds: Vec<Round> = lines.iter().map(|l| {
        let parts: Vec<&str> = l.split(" ").collect();
        return Round { opp_move: parts[0].chars().next().unwrap(), my_move: parts[1].chars().next().unwrap() }
    }).collect();

    let mut total_score : i32 = 0;

    let scores: Vec<i32> = rounds.iter().map(|r| {
        let score = r.calc_score();
        total_score = total_score + score;
        return score;
    }).collect();

    print!("{}\n", &scores[0]);
    print!("{}\n", &scores[1]);
    print!("{}\n", &scores[2]);
    print!("{}\n", &scores[3]);

    print!("total: {}\n", total_score);
}
