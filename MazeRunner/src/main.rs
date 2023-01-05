use std::fs;
use std::str::FromStr;
use std::default::Default;


#[derive(Clone, Debug)]
struct Node {
    position: [i8; 2],
    doors: [bool; 4],
    key: bool,
    left: Option<Box<Node>>,        // uzmi samo pozicije tih okolnih
    right: Option<Box<Node>>,
    up: Option<Box<Node>>,
    down: Option<Box<Node>>,
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
    println!("Hello, world!");

    let matrix = get_input_from_txt("amandaMaze.txt".to_string());
    println!("gotov unos matrice");

    let path = search(Some(Box::new(matrix[0][0].clone())), Vec::new(), false, Vec::new());
    println!("{:?}", path);
}

fn search(node: Option<Box<Node>>, mut path: Vec<([i8; 2], i32)>, was_throw_door: bool, mut best_path: Vec<([i8; 2], i32)>) -> (Vec<([i8; 2], i32)>, Vec<([i8; 2], i32)>) {
    let node = node.unwrap();

    if path.len() + 1 > best_path.len() && best_path.len() > 0 {    //prekoracio je vec dozvoljenu duzinu puta
        return (path, best_path)
    }

    
    if node.exit {      //dosao je do kraja (vratim mu path bez tog poslednjeg koraka -> treba da ga dodam posle rucno)
        return (path.clone(), path)
    }


    let mut keys = if node.key {
        path.last_mut().unwrap().1 + 1
    } else {
        path.last_mut().unwrap().1
    };

    if was_throw_door {
        keys -= 1;
    }

    if !path.contains(&(node.position, keys)) {         // da li sam vec bio tu
        path.push((node.position, keys));
    } else {
        return (path, best_path)
    }



    (path, best_path) = match node.down {
        Some(down) => {
            if down.doors[2] {
                if path[path.len()-1].1 > 0 {
                    //path.last_mut().unwrap().1 -= 1;
                    search(Some(down), path, true, best_path)
                } else {
                    (path, best_path)
                }
            } else {
                search(Some(down), path, false, best_path)
            }
        },
        None => { (path, best_path) }
    };

    (path, best_path) = match node.left {
        Some(left) => {
            if left.doors[2] {
                if path[path.len()-1].1 > 0 {
                    search(Some(left), path, true, best_path)
                } else {
                    (path, best_path)
                }
            } else {
                search(Some(left), path, false, best_path)
            }
        },
        None => { (path, best_path) }
    };

    (path, best_path) = match node.right {
        Some(right) => {
            if right.doors[2] {
                if path[path.len()-1].1 > 0 {
                    search(Some(right), path, true, best_path)
                } else {
                    (path, best_path)
                }
            } else {
                search(Some(right), path, false, best_path)
            }
        },
        None => { (path, best_path) }
    };

    (path, best_path) = match node.up {
        Some(up) => {
            if up.doors[2] {
                if path[path.len()-1].1 > 0 {
                    search(Some(up), path, true, best_path)
                } else {
                    (path, best_path)
                }
            } else {
                search(Some(up), path, false, best_path)
            }
        },
        None => { (path, best_path) }
    };


    // path = match node.up {
    //     Some(up) => {
    //         search(Some(up), path)
    //     },
    //     None => { path }
    // };

    (path, best_path)
}


fn get_input_from_txt(file_path: String) -> Vec<Vec<Node>> {
    let contents = fs::read_to_string(file_path).expect("Error reading file");
    
    let mut matrix: Vec<Vec<Node>> = Vec::new();
    for _ in 0..6 {
        let mut row = Vec::new();
        for _ in 0..9 {
            row.push(Node::default());
        }
        matrix.push(row);
    }
    

    for (i, line) in contents.lines().enumerate() {
        let mut line_list = line.split_whitespace();

        let mut direction = line_list.next().unwrap().chars().enumerate();
        let mut doors = line_list.next().unwrap().chars().enumerate();
        let mut key_and_exit = line_list.next().unwrap().chars().enumerate();


        let left = if direction.next() == Some((0, '1')) {
            Some(Box::new(matrix[i/9][i%9 - 1].clone()))
            } else {
            None
        };
        let right = if direction.next() == Some((1, '1')) {
            Some(Box::new(matrix[i/9][i%9 + 1].clone()))
            } else {
            None
        };
        let up = if direction.next() == Some((2, '1')) {
            Some(Box::new(matrix[i/9 - 1][i%9].clone()))
            } else {
            None
        };
        let down = if direction.next() == Some((3, '1')) {
            Some(Box::new(matrix[i/9 + 1][i%9].clone()))
            } else {
            None
        };

        let key = key_and_exit.next().unwrap().1 == '1' && key_and_exit.next().unwrap().1 == '1';
        let exit = key_and_exit.next().unwrap().1 == '1' && key_and_exit.next().unwrap().1 == '1';
            
        let node = Node {
            position: [(i/9).try_into().unwrap(), (i%9).try_into().unwrap()],
            doors: [doors.next().unwrap().1 == '1', doors.next().unwrap().1 == '1', doors.next().unwrap().1 == '1', doors.next().unwrap().1 == '1'],
            key: key,
            left: left,
            right: right,
            up: up,
            down: down,
            exit: exit,
        };


        matrix[i/9][i%9] = node;        
    }

    matrix
}
