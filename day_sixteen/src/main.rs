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

fn get_subset(base_order: &Vec<usize>, ss_num: i32) -> Vec<usize> {
    let mut rslt: Vec<usize> = Vec::new();
    let mut ss_num_remaining: i32 = ss_num;    
    for i in 0..base_order.len() {
        let mut choice = ss_num_remaining % 2;
        ss_num_remaining = (ss_num_remaining - choice) / 2;

        if choice == 1 {
            rslt.push(base_order[i]);
        }
    }

    if ss_num_remaining > 0 {
        return Vec::new();
    }

    return rslt;
}

fn get_permutation(base_order: &Vec<usize>, perm_num: i32) -> Vec<usize> {
    let mut rslt: Vec<usize> = Vec::new();
    let mut perm_num_remaining: i32 = perm_num;
    let mut mod_value: i32 = base_order.len() as i32;
    while mod_value > 1 {
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


fn set_minus(base_order: &Vec<usize>, subset: &Vec<usize>) -> Vec<usize> {
    let mut rslt: Vec<usize> = Vec::new();
    for i in 0..base_order.len() {
        if !subset.contains(&base_order[i]) {
            rslt.push(base_order[i]);
        }
    }    
    return rslt; 
}


fn get_pressure_relieved(valves: &Vec<Valve>, distance: &[[i32;60];60], start: usize, time_allowed: i32, plan: &Vec<usize>) -> i32 {
    let mut next_plan_idx: usize = 0;
    let mut pr = 0;
    let mut next_free = 0;
    let mut loc = start;
    for time_now in 0..time_allowed {
        if next_free == time_now && next_plan_idx < plan.len() {
            let target = plan[next_plan_idx];
            let new_time = time_now + distance[loc][target] + 1;
            if new_time < time_allowed {
                next_free = new_time;
                loc = target;
                pr = pr + (time_allowed - new_time) * valves[target].flow;
                next_plan_idx = next_plan_idx + 1;
            } 
        }
    }
    return pr;
}

fn get_best_pressure_relieved(valves: &Vec<Valve>, distance: &[[i32;60];60], start: usize, time_allowed: i32, plan: &Vec<usize>) -> i32 {
    let mut best_pr = 0;
    let mut perm_num = 0;
    let mut test_plan = get_permutation(plan, perm_num);
    while test_plan.len() > 0 {
        let this_pr = get_pressure_relieved(valves, distance, start, time_allowed, &test_plan);
        if this_pr > best_pr {
            best_pr = this_pr;
        }
        perm_num = perm_num + 1;
        test_plan = get_permutation(plan, perm_num);
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

    let mut worthy_valves: Vec<usize> = Vec::new();

    for i in 0..valves.len() {
        if valves[i].flow > 0 {
            worthy_valves.push(i);
        }    
    }

    let mut best_pr = 0;
    let mut ss_num = 1;
    let mut my_valves = get_subset(&worthy_valves, ss_num);
    while my_valves.len() > 0 {

        

        if my_valves.len() == 7 { 
            let ele_valves = set_minus(&worthy_valves, &my_valves);
            let my_max = get_best_pressure_relieved(&valves, &distance, 31, 26, &my_valves);
            let ele_max = get_best_pressure_relieved(&valves, &distance, 31, 26, &ele_valves);

            if my_max + ele_max > best_pr {
                best_pr = my_max + ele_max;
                print!("{}\r\n", best_pr);
            }

        }

        ss_num = ss_num + 1;
        my_valves = get_subset(&worthy_valves, ss_num);
    }

    print!("{}\r\n", best_pr);
}
