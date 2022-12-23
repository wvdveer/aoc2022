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

fn get_permutation(base_order: &Vec<usize>, perm_num: i32, num_to_choose: i32) -> Vec<usize> {
    let mut rslt: Vec<usize> = Vec::new();
    let mut perm_num_remaining: i32 = perm_num;
    let mut mod_value: i32 = base_order.len() as i32;
    let num_to_leave: i32 = (base_order.len() as i32) - num_to_choose + 1;
    while mod_value > num_to_leave {
        let mut choice = perm_num_remaining % mod_value;
        perm_num_remaining = (perm_num_remaining - choice) / mod_value;

        let mut place: i32 = 0;
        while rslt.contains(&base_order[place as usize]) {
            place = place + 1;
        } 
        while choice > 0 {
            place = place + 1;
            while rslt.contains(&base_order[place as usize]) {
                place = place + 1;
            }  
            choice = choice - 1;
        }

        rslt.push(base_order[place as usize]);

        mod_value = mod_value - 1;
    }

    if perm_num_remaining > 0 {
        return Vec::new();
    }

    for i in 0..base_order.len() {
        if !rslt.contains(&base_order[i]) {
            rslt.push(base_order[i]);
            break;
        }
    }

    return rslt;
}



/*fn get_best_pressure(valves: &Vec<Valve>, distance: &[[i32;60];60], current: usize, visited: Vec<usize>, time_so_far: i32, pr_so_far: i32, pass_num: i32) -> PlanAndResult {
    let mut best_pr: PlanAndResult = PlanAndResult { visited: visited.clone(), pr: pr_so_far };
    for i in 0..valves.len() {
        if !visited.contains(&i) && valves[i].flow > 0 {
            let new_time = time_so_far + distance[current][i] + 1;
            if new_time <= 30 {
                let mut new_visited = visited.clone();
                new_visited.push(i);
                let extra_pr = (30 - new_time) * valves[i].flow;
                let mut pr_plan = get_best_pressure(valves, distance, i, new_visited.clone(), new_time, pr_so_far + extra_pr, pass_num);
                if pass_num == 1 {
                    pr_plan = get_best_pressure(valves, distance, 0, pr_plan.visited.clone(), 4, pr_plan.pr, 2);
                }
                if pr_plan.pr > best_pr.pr {
                    best_pr = pr_plan;
                }
            }    
        }    
    } 
    return best_pr;
}*/

fn get_best_next(valves: &Vec<Valve>, distance: &[[i32;60];60], visited: &Vec<usize>, loc: usize, time_rem: i32) -> usize {
    let mut best_next = 1000;
    let mut best_value: f32 = 0.0;
    for i in 0..valves.len() {
        if !visited.contains(&i) && valves[i].flow > 0 {
            let time_to = distance[loc][i] + 1;
            let pr = (time_rem - time_to) * valves[i].flow;
            let this_value: f32 = (pr as f32) / (time_to as f32);
            if this_value > best_value {
                best_value = this_value;
                best_next = i;
            }
        }
    }
    return best_next;
}


fn get_pressure_relieved(valves: &Vec<Valve>, distance: &[[i32;60];60], start: usize, time_allowed: i32) -> i32 {
    let mut visited: Vec<usize> = Vec::new();
    let mut pr = 0;
    let mut me_next_free = 0;
    let mut ele_next_free = 0;
    let mut me_loc = start;
    let mut ele_loc = start;
    for time_now in 0..time_allowed {
        if me_next_free == time_now {
            let target = get_best_next(valves, distance, &visited, me_loc, time_allowed - time_now);
            if target < 1000 {
                let new_time = time_now + distance[me_loc][target] + 1;
                me_next_free = new_time;
                me_loc = target;
                pr = pr + (time_allowed - new_time) * valves[target].flow;
                visited.push(target);
            } 
        }
        /*if ele_next_free == time_now {
            let target = get_best_next(valves, distance, &visited, me_loc, time_allowed - time_now);
            if target < 1000 {
                let new_time = time_now + distance[ele_loc][target] + 1;
                ele_next_free = new_time;
                ele_loc = target;
                pr = pr + (time_allowed - new_time) * valves[target].flow;
                visited.push(target);
            } 
        }*/
    }
    return pr;
}


fn main() {
    let file_path: String = "data/test.txt".to_string();
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

    let mut best_pr = get_pressure_relieved(&valves, &distance, 0, 30);

    print!("{}\r\n", best_pr);
}
