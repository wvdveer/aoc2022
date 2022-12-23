use std::fs;

struct DirEntry {
    path: String,
    is_dir: bool,
    file_size: i32
} 

struct DirEntries<> {
    dir_entries: Vec<DirEntry>
}

impl DirEntries<> {
    fn new() -> DirEntries<> {
        DirEntries{ dir_entries: vec![] }
    }

    fn add(self: &mut Self, path: &String, is_dir: bool, file_size: i32) {
        let b= DirEntry {path: path.to_string(), is_dir: is_dir, file_size: file_size};

        for i in 0..self.dir_entries.len() {
            if &self.dir_entries[i].path == path {
                self.dir_entries[i] = b;
                return;
            }
        }
        
        self.dir_entries.push(b);
    } 

    fn get_folder_size(self: &Self, path: &str) -> i32 {
        let mut total_size = 0;
        for ent in &self.dir_entries {
            if ! ent.is_dir {
                if path.len() < ent.path.len() && &ent.path[0..path.len()] == path {
                    total_size = total_size + ent.file_size;
                }
            }
        }
        return total_size;
    } 
}

fn main() {
    let file_path: String = String::from("data/input.txt");
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut curpath: String = "/".to_string();    
    let commands: Vec<&str> = contents.split("\r\n$ ").collect();   

    let mut dir_entries: DirEntries = DirEntries::new();

    for cmd in &commands {
        if cmd == &"" {
            continue;
        }    
        let lines: Vec<&str> = cmd.split("\r\n").collect();  
        let parts: Vec<&str> = lines[0].split(" ").collect();
        if parts[0] == "cd" {
            if parts[1] == "/" {
                curpath = "/".to_string();
            } else if parts[1] == ".." {
                let pos = curpath.rfind('/').unwrap();
                curpath = (curpath.as_str()[0..pos]).to_string();
                if curpath == "" {
                    curpath = "/".to_string();
                }
            } else {
                if curpath == "/" {
                    curpath = "".to_string();
                }
                curpath = format!("{}/{}", curpath, parts[1]);  
            }
            print!("{} -> {}\r\n", lines[0], curpath);
            
            dir_entries.add(&curpath, true, 0);
        } else { // ls 
            for line_num in 1..lines.len() {
                let parts: Vec<&str> = lines[line_num].split(" ").collect();
                let file_path = format!("{}/{}", curpath, parts[1]);
                let is_dir = parts[0] == "dir";
                let file_size = if is_dir { 0 } else { parts[0].to_string().parse::<i32>().unwrap() };

                dir_entries.add(&file_path, is_dir, file_size);

                print!("{} {}\r\n", parts[1], file_size);
            }
        }
    }

    let mut folder_list: Vec<&str> = vec![];
    
    for ent in &dir_entries.dir_entries {
        if ent.is_dir {
            folder_list.push(ent.path.as_str());
        } 
    };

    let tf = dir_entries.get_folder_size("/");

    let fs = 70000000 - tf;

    print!("Free space: {}\r\n", fs);

    let rd = 30000000 - fs;

    print!("req delete: {}\r\n", rd);

    let mut min_fs = fs;

    for folder_name in folder_list {
        let fs = dir_entries.get_folder_size(folder_name);
        if fs >=  rd {
            print!("{} {}\r\n", fs, folder_name);
            if fs < min_fs {
                min_fs = fs;
            } 
        }
    }

    print!("{}\r\n", min_fs);

    //print!("{}", dir_entries.get_folder_size("/"));
}
