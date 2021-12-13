use std::time::{Instant};


//const COL_SIZE: usize = 100;
//const ROW_SIZE: usize = 100;
//#[derive(Copy, Clone, Debug)]
#[derive(Debug)]
struct FromTo{
    from: String,
    to: String
}

#[derive(Clone, Debug)]
struct VisitedSmallNode{
    name : String,
    counter: u32
}

fn main() {
    let start = Instant::now();
    //let data = include_str!("data.txt");
    let data = include_str!("data_small.txt");

    let mut vec_from_to : Vec<FromTo> = Vec::new();

    //Fill from_to
    for line in data.lines(){
        let split_line:Vec<&str> = line.split("-").collect();
        vec_from_to.push(FromTo{from:split_line[0].to_string(), to:split_line[1].to_string()});
    }

    let mut vec_visited_small_nodes : Vec<VisitedSmallNode> = Vec::new();
    let mut vec_paths : Vec<String> = Vec::new();

    let start_node: String = String::from("start");
    let path : String = String::from("start");

    find_path_recursive(&vec_from_to, &path, &start_node, &mut vec_visited_small_nodes, &mut vec_paths);

    let duration = start.elapsed();  
    
    //println!("paths: {:?}", vec_paths);
    println!("{}", vec_paths.len());
    println!("duration {:?}", duration);
}


fn is_in_visited_small_nodes(vec_visited_small_nodes: &Vec<VisitedSmallNode>, node_name: &str) -> bool{
    if !node_name.chars().all(|c| c.is_ascii_lowercase()){
        return false//return early if uppercase
    }

    let any_small_node_count_gt_1 = vec_visited_small_nodes.iter().any(|c| c.counter>1);

    let mut found_in_visited_small_nodes = false;
    for visited_small_node in vec_visited_small_nodes.iter(){
        if visited_small_node.name == node_name{
            //println!("found in found_in_visited_small_nodes : {}", node_name);
            if any_small_node_count_gt_1{
                found_in_visited_small_nodes = true;
            } 
        }
    }
    found_in_visited_small_nodes
}

fn upsert_small_node_to_visited_small_nodes(vec_visited_small_nodes: &mut Vec<VisitedSmallNode>, new_node: VisitedSmallNode){
    let mut updated = false;
    for visited_small_node in vec_visited_small_nodes.into_iter(){
        if visited_small_node.name == new_node.name{
            visited_small_node.counter += 1;
            updated = true;
        }
    }
    if !updated{
        vec_visited_small_nodes.push(new_node);
    }  
}


fn get_all_tos_from_here(vec_from_to: &Vec<FromTo>, vec_visited_small_nodes: &Vec<VisitedSmallNode>, curr_node: &str) -> Vec<String>{//maybe refactor to use lifetimes and return Vec<&str>?
    //println!("get_all_tos_from_here here: {}", curr_node);
    //println!("get_all_tos_from_here vec_visited_small_nodes: {:?}", vec_visited_small_nodes);
    //println!("get_all_tos_from_here vec_from_to: {:?}", vec_from_to);
    let mut tos: Vec<String> = Vec::new();
    for from_to in vec_from_to.iter(){
        if from_to.to == curr_node {
            if curr_node == "start"{
                tos.push(from_to.from.clone())
            }
            else{
                if from_to.from != "start" && !is_in_visited_small_nodes(vec_visited_small_nodes, &from_to.from){
                    tos.push(from_to.from.clone())
                }
            }
        }
        else if from_to.from == curr_node{
            if curr_node == "start"{
                tos.push(from_to.to.clone())
            }
            else{
                if from_to.to != "start" && !is_in_visited_small_nodes(vec_visited_small_nodes, &from_to.to){
                    tos.push(from_to.to.clone())
                }
            }            
        }
    }
    //println!("get_all_tos_from_here returning tos: {:?}", tos);
    tos
}

fn find_path_recursive(vec_from_to: &Vec<FromTo>, path_to_here: &str, curr_node: &str, 
            vec_visited_small_nodes: &Vec<VisitedSmallNode>, vec_paths: &mut Vec<String>){
    //println!("path_to_here: {:?}", path_to_here);
    //println!("paths: {:?}", vec_paths);
 
    let vec_tos_from_here = get_all_tos_from_here(&vec_from_to, vec_visited_small_nodes, &curr_node);

    for to_from_here in vec_tos_from_here{
        if to_from_here != "end"{
            //println!("Calling find_path_recursive again");
            let new_path :String = format!("{}{}{}", path_to_here.clone(), ",", to_from_here);
            let mut new_vec_visited_small_nodes = vec_visited_small_nodes.clone();
            let temp : String = to_from_here.clone();//should not be necessary?
            if temp.chars().all(|c| c.is_ascii_lowercase()){
                //new_vec_visited_small_nodes.push(VisitedSmallNode{name:to_from_here.clone(), counter: 1});
                upsert_small_node_to_visited_small_nodes(&mut new_vec_visited_small_nodes, VisitedSmallNode{name:to_from_here.clone(), counter: 1});
            }
            let new_curr_node = to_from_here.clone();
            find_path_recursive(&vec_from_to, &new_path, &new_curr_node, &new_vec_visited_small_nodes, vec_paths);
        }
        else if to_from_here == "end"{
            let new_path :String = format!("{}{}{}", path_to_here, ",", to_from_here);
            //println!("End detected pushing new_path {}", new_path);
            vec_paths.push(new_path.clone());
        }
    }
}