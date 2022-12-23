use std::fs;
use std::ops;

#[derive(Copy, Clone)]
struct Voxel {
    x: i32,
    y: i32,
    z: i32
}

impl Voxel {
    fn new() -> Voxel {
        Voxel { x: 0, y: 0, z: 0 }
    }

    fn parse(src: &str) -> Voxel {
        let coords_str: Vec<&str> = src.split(",").collect();
        let x: i32 = coords_str[0].parse::<i32>().unwrap();
        let y: i32 = coords_str[1].parse::<i32>().unwrap();
        let z: i32 = coords_str[2].parse::<i32>().unwrap();
        Voxel { x: x, y: y, z: z }
    }
   
}

impl ops::Add<Voxel> for Voxel {
    type Output = Voxel;

    fn add(self: Voxel, other: Voxel) -> Voxel {
        Voxel { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

fn has_voxel( voxel_list: &Vec<Voxel>, voxel: &Voxel) -> bool {
    for item in voxel_list {
        if item.x == voxel.x && item.y == voxel.y && item.z == voxel.z {
            return true;
        }
    }
    return false;
}

fn main() {
    let file_path: String = "data/input.txt".to_string();
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect(); 
    
    let neighbour_deltas: [Voxel;6] = [
        Voxel{ x: -1, y:  0, z:  0 },
        Voxel{ x:  1, y:  0, z:  0 },
        Voxel{ x:  0, y: -1, z:  0 },
        Voxel{ x:  0, y:  1, z:  0 },
        Voxel{ x:  0, y:  0, z: -1 },
        Voxel{ x:  0, y:  0, z:  1 }    
    ];

    let voxels: Vec<Voxel> = lines.iter().map(|l|{
        return Voxel::parse(l);
    }).collect();

    
    let mut outside: Vec<Voxel> = Vec::new();
    for i in 0..=21 {
        for j in 0..=21 {
            let xl = Voxel { x:  0, y:  i, z:  j };
            let xh = Voxel { x: 21, y:  i, z:  j };
            let yl = Voxel { x:  i, y:  0, z:  j };
            let yh = Voxel { x:  i, y: 21, z:  j };
            let zl = Voxel { x:  i, y:  j, z:  0 };
            let zh = Voxel { x:  i, y:  j, z: 21 };

            // this will have duplicates on edges, but it doesn't matter
            outside.push(xl);
            outside.push(xh);
            outside.push(yl);
            outside.push(yh);
            outside.push(zl);
            outside.push(zh);
        }
    }

    let mut found_more_outside: bool = true;
    while found_more_outside {
        found_more_outside = false;

        for lx in 1..=20 {
            for ly in 1..=20 {
                for lz in 1..=20 {
                    let voxel = Voxel { x: lx, y: ly, z: lz };
                    if !has_voxel(&outside, &voxel) && !has_voxel(&voxels, &voxel) {
                        for i in 0..6 {
                            let neighbour = voxel + neighbour_deltas[i];
                            if has_voxel(&outside, &neighbour) {
                                outside.push(voxel);
                                found_more_outside = true;
                            }
                        } 
                    }     
                }
            }
        }
    }

    let mut sa: i32 = 0;

    for voxel in &voxels {
        for i in 0..6 {
            let neighbour = *voxel + neighbour_deltas[i];
            if has_voxel(&outside, &neighbour) {
                sa = sa + 1;
            }
        }
    }


    print!("{}\r\n", sa);
}
