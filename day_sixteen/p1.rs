use std::fs;

#[derive(Clone, Default)]
struct Valve {
    name: String,
    flow: i32,
    tunnels: Vec<String>
}

impl Valve {
    fn new() -> Valve {
        Valve { name: "".to_string(), flow: 0, tunnels: Vec::new() }
    }

    fn parse(src: &str) -> Valve {
        let parts: Vec<&str> = src.split("; tunnel").collect();
        let name = parts[0][6..8].to_string();
        let flow_str: &str = &parts[0][23..];
        let flow = flow_str.parse::<i32>().unwrap();
        let tunnels_str: Vec<&str> = parts[1].split(",").collect();
        let mut tunnels: Vec<String> = Vec::new();
        for ts in tunnels_str {
            tunnels.push(ts[ts.len()-2..].to_string());
        };
        Valve { name: name, flow: flow, tunnels: tunnels }
    }
}


fn get_best_pressure(valves: &Vec<Valve>, distance: &[[i32;60];60], current: usize, visited: Vec<usize>, time_so_far: i32, pr_so_far: i32) -> i32 {
    let mut best_pr = pr_so_far;
    for i in 0..valves.len() {
        if !visited.contains(&i) && valves[i].flow > 0 {
            let new_time = time_so_far + distance[current][i] + 1;
            if new_time <= 30 {
                let mut new_visited = visited.clone();
                new_visited.push(i);
                let extra_pr = (30 - new_time) * valves[i].flow;
                let pr_plan = get_best_pressure(valves, distance, i, new_visited, new_time, pr_so_far + extra_pr);
                if pr_plan > best_pr {
                    best_pr = pr_plan;
                }
            }
        }    
    } 

    return best_pr;
}


fn main() {
    let file_path: String = "data/input.txt".to_string();
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect(); 

    let mut valves: Vec<Valve> = Vec::new();

    for line in &lines {
        valves.push(Valve::parse(line));
    }

    let mut distance: [[i32;60];60] = [[1000;60];60];

    let mut improved_route = true;
    while improved_route {
        improved_route = false;
        for i in 0..valves.len() {       
            for j in 0..valves.len() {
                let mut best_route = distance[i][j];
                if i == j {
                    best_route = 0;
                } else {
                    for tunnel in &valves[i].tunnels {
                        if tunnel == &valves[j].name {
                            best_route = 1;
                        } 
                    }
                }
                for k in 0..valves.len() {
                    if distance[i][k] + distance[k][j] < best_route {
                        best_route = distance[i][k] + distance[k][j];
                    }
                }
                if best_route < distance[i][j] {
                    distance[i][j] = best_route;
                    improved_route = true;
                }
            }
        }
    }

    let best_pressure_released = get_best_pressure(&valves, &distance, 31, Vec::new(), 0, 0);

    print!("{}\r\n", best_pressure_released);
}
