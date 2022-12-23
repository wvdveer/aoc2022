use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Clone)]
struct Blueprint {
    number: i32,
    ore_robot_ore_cost: i32,
    clay_robot_ore_cost: i32,
    obsidian_robot_ore_cost: i32,
    obsidian_robot_clay_cost: i32,
    geode_robot_ore_cost: i32,
    geode_robot_obsidian_cost: i32,
    max_ore_cost: i32
}

impl Blueprint {
    fn parse(src: &str) -> Blueprint {
        let parts: Vec<&str> = src.split(" ").collect();
        let num_str = parts[1];
        let number: i32 = num_str[0..num_str.len()-1].parse::<i32>().unwrap();
        let ore_robot_ore_cost: i32 = parts[6].parse::<i32>().unwrap();
        let mut max_ore_cost = ore_robot_ore_cost;
        let clay_robot_ore_cost: i32 = parts[12].parse::<i32>().unwrap();
        if clay_robot_ore_cost > max_ore_cost {
            max_ore_cost = clay_robot_ore_cost;
        }
        let obsidian_robot_ore_cost: i32 = parts[18].parse::<i32>().unwrap();
        if obsidian_robot_ore_cost > max_ore_cost {
            max_ore_cost = obsidian_robot_ore_cost;
        }
        let obsidian_robot_clay_cost: i32 = parts[21].parse::<i32>().unwrap();
        let geode_robot_ore_cost: i32 = parts[27].parse::<i32>().unwrap();
        if geode_robot_ore_cost > max_ore_cost {
            max_ore_cost = geode_robot_ore_cost;
        }
        let geode_robot_obsidian_cost: i32 = parts[30].parse::<i32>().unwrap();
        Blueprint { number: number,
                    ore_robot_ore_cost: ore_robot_ore_cost, 
                    clay_robot_ore_cost: clay_robot_ore_cost, 
                    obsidian_robot_ore_cost: obsidian_robot_ore_cost, 
                    obsidian_robot_clay_cost: obsidian_robot_clay_cost, 
                    geode_robot_ore_cost: geode_robot_ore_cost, 
                    geode_robot_obsidian_cost: geode_robot_obsidian_cost,
                    max_ore_cost: max_ore_cost
                 }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum FactoryActions {
    Nothing,
    Ore,
    Clay,
    Obsidian,
    Geode
}


#[derive(Clone, PartialEq, Eq, Hash, )]
struct Status {
    time_remaining: i32,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32
}  

impl Status {
    fn get_possible_factory_actions(self: &Self, blueprint: &Blueprint) -> Vec<FactoryActions> {
        let mut rslt: Vec<FactoryActions> = Vec::new();
        rslt.push(FactoryActions::Nothing);
        if self.ore >= blueprint.ore_robot_ore_cost && self.ore_robots < blueprint.max_ore_cost {
            rslt.push(FactoryActions::Ore);
        }
        if self.ore >= blueprint.clay_robot_ore_cost && self.clay_robots < blueprint.obsidian_robot_clay_cost {
            rslt.push(FactoryActions::Clay);
        }
        if self.ore >= blueprint.obsidian_robot_ore_cost && self.clay >= blueprint.obsidian_robot_clay_cost 
                && self.obsidian_robots < blueprint.geode_robot_obsidian_cost {
            rslt.push(FactoryActions::Obsidian);
        }
        if self.ore >= blueprint.geode_robot_ore_cost && self.obsidian >= blueprint.geode_robot_obsidian_cost {
            rslt.push(FactoryActions::Geode);
        }
        return rslt;
    }

    fn get_next_status_for(self: &Self, factory_action: FactoryActions, blueprint: &Blueprint) -> Status {
        let mut next_status: Status = self.clone();
        next_status.time_remaining = next_status.time_remaining - 1; 
        next_status.ore = next_status.ore + next_status.ore_robots;
        next_status.clay = next_status.clay + next_status.clay_robots;
        next_status.obsidian = next_status.obsidian + next_status.obsidian_robots;
        next_status.geode = next_status.geode + next_status.geode_robots;

        match factory_action {
            FactoryActions::Ore => {
                next_status.ore_robots = next_status.ore_robots + 1; 
                next_status.ore = next_status.ore - blueprint.ore_robot_ore_cost;
                return next_status;
            },
            FactoryActions::Clay => {
                next_status.clay_robots = next_status.clay_robots + 1; 
                next_status.ore = next_status.ore - blueprint.clay_robot_ore_cost;
                return next_status;
            },
            FactoryActions::Obsidian => {
                next_status.obsidian_robots = next_status.obsidian_robots + 1; 
                next_status.ore = next_status.ore - blueprint.obsidian_robot_ore_cost;
                next_status.clay = next_status.clay - blueprint.obsidian_robot_clay_cost;
                return next_status;
            },
            FactoryActions::Geode => {
                next_status.geode_robots = next_status.geode_robots + 1; 
                next_status.ore = next_status.ore - blueprint.geode_robot_ore_cost;
                next_status.obsidian = next_status.obsidian - blueprint.geode_robot_obsidian_cost;
                return next_status;
            },
            FactoryActions::Nothing => return next_status,    
        }
    }
}




fn get_state_after(blueprint: &Blueprint, current_state_in: &Status, next_robot: FactoryActions) -> Option<Status> {
    let mut current_state = current_state_in.clone();
    let mut built_robot = false;
    while current_state.time_remaining > 0 && ! built_robot {
        let possible_actions: Vec<FactoryActions> = current_state.get_possible_factory_actions(blueprint);
        let mut chosen_action: FactoryActions = FactoryActions::Nothing;
        if possible_actions.contains(&FactoryActions::Geode) && next_robot == FactoryActions::Geode {            
            chosen_action = FactoryActions::Geode;
            built_robot = true;
        } else if possible_actions.contains(&FactoryActions::Ore) && next_robot == FactoryActions::Ore {
           chosen_action = FactoryActions::Ore;
           built_robot = true;
        } else if possible_actions.contains(&FactoryActions::Clay) && next_robot == FactoryActions::Clay {
            chosen_action = FactoryActions::Clay;
            built_robot = true;
        } else if possible_actions.contains(&FactoryActions::Obsidian) && next_robot == FactoryActions::Obsidian {
            chosen_action = FactoryActions::Obsidian;
            built_robot = true;
        } 
        current_state = current_state.get_next_status_for(chosen_action, blueprint);      
    }
    if built_robot == false {
        // we couldn't build this schedule
        return Option::None;
    }

    return Some(current_state);
}


fn get_max_geodes(blueprint: &Blueprint, time: i32) -> i32 {
    let mut initial_state = Status {
        time_remaining: time,
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0 
    };
    let mut best = 0;
    let mut processing_queue: VecDeque<Status> = VecDeque::new();
    processing_queue.push_back(initial_state);
    let mut processed_set: HashSet<Status> = HashSet::new();
    while processing_queue.len() > 0 {
        let current_state = processing_queue.pop_front().unwrap();
        
        if current_state.geode > best {
            best = current_state.geode;
        }

        if processed_set.contains(&current_state) {
            continue;
        }
        processed_set.insert(current_state.clone());

        for fa in [ FactoryActions::Geode, FactoryActions::Obsidian, FactoryActions::Clay, FactoryActions::Ore ] {
            let next_state_maybe = get_state_after(blueprint, &current_state, fa);
            if next_state_maybe != Option::None {
                let next_state = next_state_maybe.unwrap();
                processing_queue.push_back(next_state);
            }
        }
    }
    return best;
}    


fn main() {
    let file_path: String = "data/input.txt".to_string();
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect(); 
    
    let mut total_quality_level = 0;

    for line in lines {
        let blueprint: Blueprint = Blueprint::parse(line); 
        if blueprint.number == 4 {
            break;
        }
        let geodes = get_max_geodes(&blueprint, 32);
        print!("{} {}\r\n", blueprint.number, geodes);
        let quality_level = blueprint.number * geodes;
        total_quality_level = total_quality_level + quality_level;
    }

    print!("{}\r\n", total_quality_level);
}
