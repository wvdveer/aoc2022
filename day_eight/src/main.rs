use std::fs;




fn main() {
    let file_path: String = String::from("data/input.txt");
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect();    

    let mut tree_heights: [[i32;99];99] = [[0;99];99];

    for i in 0..99 {
        let row = lines[i];
        for j in 0..99 {
            let ch: char = row.chars().nth(j).unwrap();
            tree_heights[i][j] = ((ch as u32) - ('0' as u32)) as i32;
        }
    }

    let mut max_scenic_score : i32 = 0;

    for i in 0..99 {
        for j in 0..99 {
            let target_tree_hgt = tree_heights[i][j];
            
            // up
            let mut up_dist : i32 = 0;
            if i > 0 {
                up_dist = 1;
                let mut k = i - 1;
                while k > 0 && tree_heights[k][j] < target_tree_hgt {
                    k = k - 1;
                    up_dist = up_dist + 1;
                }
            }

            // down
            let mut down_dist : i32 = 0;
            if i < 98 {
                down_dist = 1;
                let mut k = i + 1;
                while k < 98 && tree_heights[k][j] < target_tree_hgt {
                    k = k + 1;
                    down_dist = down_dist + 1;
                }
            }

            // left
            let mut left_dist : i32 = 0;
            if j > 0 {
                left_dist = 1;
                let mut k = j - 1;
                while k > 0 && tree_heights[i][k] < target_tree_hgt {
                    k = k - 1;
                    left_dist = left_dist + 1;
                }
            }

            // right
            let mut right_dist : i32 = 0;
            if j < 98 {
                right_dist = 1;
                let mut k = j + 1;
                while k < 98 && tree_heights[i][k] < target_tree_hgt {
                    k = k + 1;
                    right_dist = right_dist + 1;
                }
            }

            let scenic_score = up_dist * down_dist * left_dist * right_dist;

            print!("{},{} u{} d{} l{} r{} s{}\r\n", i, j, up_dist, down_dist, left_dist, right_dist, scenic_score);


            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    print!("max_scenic_score: {}\r\n", max_scenic_score);
}        
