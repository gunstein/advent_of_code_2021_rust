use std::time::{Instant};
use std::cmp;

const ROW_SIZE: usize = 1000;
const COL_SIZE: usize = 1000;

type Map = [[u32; COL_SIZE]; ROW_SIZE];

#[derive(Copy, Clone, Debug)]
struct Point{
    x:u32,
    y:u32
}


fn main() {
    let start = Instant::now();
    let data = include_str!("data.txt");
    //let data = include_str!("data_small.txt");

    let mut map : Map = [[0; COL_SIZE]; ROW_SIZE];

    for line in data.lines(){
        //println!("line {}", line);
        
        let make_point = |coord_pair:&str| {
            let coord : Vec<&str> = coord_pair.split(",").collect();
            //println!("coord {:?}", coord);
            Point{x:coord[0].parse::<u32>().unwrap(), y:coord[1].parse::<u32>().unwrap()}
        };
        let coord_pairs : Vec<&str> = line.split(" -> ").collect();
        let p1 = make_point(coord_pairs[0]);
        //println!("p1 {:?}", p1);
        let p2 = make_point(coord_pairs[1]);
        //println!("p2 {:?}", p2);

        add_line_to_map(&mut map, &p1, &p2 );
        //println!("map {:?}", map);
    }

    let duration = start.elapsed();    

    let overlapping_points = count_overlapping_points(&map);

    //println!("map {:?}", map);

    println!("{}", overlapping_points);
    println!("duration {:?}", duration);
}

fn add_line_to_map(map: &mut Map, p1: &Point, p2: &Point){
    if p1.x == p2.x{
        for y in cmp::min(p1.y, p2.y)..cmp::max(p1.y, p2.y) + 1{
            map[p1.x as usize][y as usize] += 1;
        }        
    }
    else if p1.y == p2.y{
        for x in cmp::min(p1.x, p2.x)..cmp::max(p1.x, p2.x) + 1{
            map[x as usize][p1.y as usize] += 1;
        }
    }
}

fn count_overlapping_points(map: &Map) -> u32{
    let mut num_overlapping = 0;
    for x in 0..ROW_SIZE{
        for y in 0..COL_SIZE{
            if map[x as usize][y as usize] > 1 {
                num_overlapping += 1;
            }
        }
    }
    num_overlapping
}