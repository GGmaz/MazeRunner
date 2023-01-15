use std::cell::RefCell;
use std::{fs, thread};
use std::rc::Rc;
use std::default::Default;
use std::sync::{Arc, Mutex, RwLock};
use rayon::prelude::*;


#[derive(Clone, Debug)]
struct Node {
    position: [i8; 2],
    doors: [bool; 4],
    key: bool,
    left: Option<Arc<RwLock<Node>>>,
    right: Option<Arc<RwLock<Node>>>,
    up: Option<Arc<RwLock<Node>>>,
    down: Option<Arc<RwLock<Node>>>,
    exit: bool,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            position: [0, 0],
            doors: [false, false, false, false],
            key: false,
            left: None,
            right: None,
            up: None,
            down: None,
            exit: false,
        }
    }
}

fn main() {
    let head = get_input_from_txt("amandaMaze.txt".to_string());
                    //path je vector tuple-ova koji u sebi sadrzi poziciju, broj kljuceva na toj poziciji, i da li je pokupio tada kljuc
    let res = Arc::new(Mutex::new(vec![]));
    search(Some(head), vec![([0, 0], 0)], false, res.clone());
    println!("{:?}", res.clone());
    let res_guard = res.lock().unwrap();
    print_result_matrix(res_guard.to_vec());
}


fn search(node: Option<Arc<RwLock<Node>>>, mut path: Vec<([i8; 2], i32)>, was_throw_door: bool, best_path: Arc<Mutex<Vec<([i8; 2], i32)>>>) {
    let node = node.unwrap();
    let node_guard = node.read().unwrap();

    let mut best = best_path.lock().unwrap();
    if path.len() + 1 > best.len() && best.len() > 1 {    //prekoracio je vec dozvoljenu duzinu puta
        return 
    }

    
    if node_guard.exit {      //dosao je do kraja
        path.push((node_guard.position, path.last().unwrap().1));
        *best = path;
        return 
    }
    drop(best);


    let mut keys = if node_guard.key && path.iter().find(|(x, _)| *x == node_guard.position) == None {
        path.last_mut().unwrap().1 + 1
    } else {
        path.last_mut().unwrap().1
    };

    if was_throw_door {
        keys -= 1;
    }

    if !path.contains(&(node_guard.position, keys)) {         // da li sam vec bio tu
        path.push((node_guard.position, keys));
    } else if path.len() == 1 {
        
    } else {
        return 
    }    
    //drop(node_guard);

    let neighbors = vec![node_guard.right.clone(), node_guard.left.clone(), node_guard.down.clone(), node_guard.up.clone()];
    //drop(node_guard);

    neighbors.into_par_iter().enumerate().for_each(|(i, neighbor)| {
        if let Some(neighbor) = neighbor {
            let cloned_path = path.clone();
            let best_path_clone = best_path.clone();

            let neighbor_guard = neighbor.read().unwrap();
            if neighbor_guard.doors[i] {
                drop(neighbor_guard);
                if path[path.len()-1].1 > 0 {
                    search(Some(neighbor), cloned_path, true, best_path_clone);
                }
            } else {
                drop(neighbor_guard);
                search(Some(neighbor), cloned_path, false, best_path_clone);
            }
        }
    });


    // match &node_guard.down {
    //     Some(down) => {
    //         let node_clone = down.clone();
    //         let cloned_path = path.clone();
    //         let best_path_clone = best_path.clone();
    //         let down_guard = down.read().unwrap();
    //         if down_guard.doors[2] {
    //             if path[path.len()-1].1 > 0 {                    
    //                 thread::spawn(move || search(Some(node_clone), cloned_path, true, best_path_clone));
    //             } 
    //         } else {
    //             thread::spawn(move || search(Some(node_clone), cloned_path, false, best_path_clone));
    //         }
    //     },
    //     None => {  }
    // };

    // match &node_guard.left {
    //     Some(left) => {
    //         let node_clone = left.clone();
    //         let cloned_path = path.clone();
    //         let best_path_clone = best_path.clone();
    //         let left_guard = left.read().unwrap();
    //         if left_guard.doors[1] {
    //             if path[path.len()-1].1 > 0 {
    //                 thread::spawn(move || search(Some(node_clone), cloned_path, true, best_path_clone));
    //             }
    //         } else {
    //             thread::spawn(move || search(Some(node_clone), cloned_path, true, best_path_clone));
    //         }
    //     },
    //     None => {  }
    // };

    // match &node_guard.right {
    //     Some(right) => {
    //         let node_clone = right.clone();
    //         let cloned_path = path.clone();
    //         let best_path_clone = best_path.clone();
    //         let right_guard = right.read().unwrap();
    //         if right_guard.doors[0] {
    //             if path[path.len()-1].1 > 0 {
    //                 thread::spawn(move || search(Some(node_clone), cloned_path, true, best_path_clone));
    //             } 
    //         } else {
    //             thread::spawn(move || search(Some(node_clone), cloned_path, true, best_path_clone));
    //         }
    //     },
    //     None => {  }
    // };

    // match &node_guard.up {
    //     Some(up) => {
    //         let node_clone = up.clone();
    //         let cloned_path = path.clone();
    //         let best_path_clone = best_path.clone();
    //         let up_guard = up.read().unwrap();
    //         if up_guard.doors[3] {
    //             if path[path.len()-1].1 > 0 {
    //                 thread::spawn(move || search(Some(node_clone), cloned_path, true, best_path_clone));
    //             }
    //         } else {
    //             thread::spawn(move || search(Some(node_clone), cloned_path, true, best_path_clone));
    //         }
    //     },
    //     None => {  }
    // };
    
}


fn get_input_from_txt(file_path: String) -> Arc<RwLock<Node>> {
    let contents = fs::read_to_string(file_path).expect("Error reading file");

    let mut matrix: Vec<Vec<Arc<RwLock<Node>>>> = Vec::new();
    for _ in 0..6 {
        let mut row = Vec::new();
        for _ in 0..9 {
            row.push(Arc::new(RwLock::new(Node::default())));
        }
        matrix.push(row);
    }
    

    for (i, line) in contents.lines().enumerate() {
        let mut line_list = line.split_whitespace();

        let mut direction = line_list.next().unwrap().chars().enumerate();
        let mut doors = line_list.next().unwrap().chars().enumerate();
        let mut key_and_exit = line_list.next().unwrap().chars().enumerate();


        let left = if direction.next() == Some((0, '1')) {
            Some(Arc::clone(&matrix[i/9][i%9 - 1]))
            } else {
            None
        };
        let right = if direction.next() == Some((1, '1')) {
            Some(Arc::clone(&matrix[i/9][i%9 + 1]))
            } else {
            None
        };
        let up = if direction.next() == Some((2, '1')) {
            Some(Arc::clone(&matrix[i/9 - 1][i%9]))
            } else {
            None
        };
        let down = if direction.next() == Some((3, '1')) {
            Some(Arc::clone(&matrix[i/9 + 1][i%9]))
            } else {
            None
        };

        let key = key_and_exit.next().unwrap().1 == '1' && key_and_exit.next().unwrap().1 == '1';
        let exit = key_and_exit.next().unwrap().1 == '1' || key_and_exit.next().unwrap().1 == '1';
        

        // let mut node_mut = matrix[i/9][i%9].clone();
        // let mut node_mut = Arc::get_mut(&mut matrix[i/9][i%9]).unwrap();
        let mut node_mut = matrix[i/9][i%9].write().unwrap();

        (*node_mut).position = [(i/9).try_into().unwrap(), (i%9).try_into().unwrap()];
        (*node_mut).key = key;
        (*node_mut).doors = [doors.next().unwrap().1 == '1', doors.next().unwrap().1 == '1', doors.next().unwrap().1 == '1', doors.next().unwrap().1 == '1'];
        (*node_mut).exit = exit;
        (*node_mut).left = left.map(|n| Arc::clone(&n));
        (*node_mut).right = right.map(|n| Arc::clone(&n));
        (*node_mut).up = up.map(|n| Arc::clone(&n));
        (*node_mut).down = down.map(|n| Arc::clone(&n));     
    }

    matrix[0][0].clone()
}






fn print_result_matrix(path: Vec<([i8; 2], i32)>) {
    for i in 0..6 {
        for j in 0..9 {
            if path.iter().find(|(pos, _)| pos[0] == i && pos[1] == j).is_some() {
                print!("1 ");
            } else {
                print!("0 ");
            }
        }
        println!();
    }

    println!("Lenght: {}", path.len());
}