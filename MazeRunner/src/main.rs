use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use std::str::FromStr;
use std::default::Default;


#[derive(Clone, Debug)]
struct Node {
    position: [i8; 2],
    doors: [bool; 4],
    key: bool,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
    up: Option<Rc<RefCell<Node>>>,
    down: Option<Rc<RefCell<Node>>>,
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

    let head = get_input_from_txt("amandaMaze.txt".to_string());
    println!("gotov unos matrice");
    //path je vector tuple-ova koji u sebi sadrzi poziciju, broj kljuceva na toj poziciji, i da li je pokupio tada kljuc
    let path = search(Some(head), vec![([0, 0], 0)], false, Vec::new());
    println!("{:?}", path.1);
}


fn search(node: Option<Rc<RefCell<Node>>>, mut path: Vec<([i8; 2], i32)>, was_throw_door: bool, mut best_path: Vec<([i8; 2], i32)>) -> (Vec<([i8; 2], i32)>, Vec<([i8; 2], i32)>) {
    let node = node.unwrap();

    if path.len() + 1 > best_path.len() && best_path.len() > 1 {    //prekoracio je vec dozvoljenu duzinu puta
        return (path, best_path)
    }

    
    if node.borrow().exit {      //dosao je do kraja
        let new_path = path.clone();
        path.push((node.borrow().position, path.last().unwrap().1));
        return (new_path, path)
    }


    let mut keys = if node.borrow().key && path.iter().find(|(x, _)| *x == node.borrow().position) == None {
        path.last_mut().unwrap().1 + 1
    } else {
        path.last_mut().unwrap().1
    };

    if was_throw_door {
        keys -= 1;
    }

    if !path.contains(&(node.borrow().position, keys)) {         // da li sam vec bio tu
        path.push((node.borrow().position, keys));
    } else if path.len() == 1 {
        
    } else {
        return (path, best_path)
    }    



    (path, best_path) = match &node.borrow().down {
        Some(down) => {
            if down.borrow().doors[2] {
                if path[path.len()-1].1 > 0 {
                    //path.last_mut().unwrap().1 -= 1;
                    search(Some(down.clone()), path, true, best_path)
                } else {
                    (path, best_path)
                }
            } else {
                search(Some(down.clone()), path, false, best_path)
            }
        },
        None => { (path, best_path) }
    };

    (path, best_path) = match &node.borrow().left {
        Some(left) => {
            if left.borrow().doors[1] {
                if path[path.len()-1].1 > 0 {
                    search(Some(left.clone()), path, true, best_path)
                } else {
                    (path, best_path)
                }
            } else {
                search(Some(left.clone()), path, false, best_path)
            }
        },
        None => { (path, best_path) }
    };

    (path, best_path) = match &node.borrow().right {
        Some(right) => {
            if right.borrow().doors[0] {
                if path[path.len()-1].1 > 0 {
                    search(Some(right.clone()), path, true, best_path)
                } else {
                    (path, best_path)
                }
            } else {
                search(Some(right.clone()), path, false, best_path)
            }
        },
        None => { (path, best_path) }
    };

    (path, best_path) = match &node.borrow().up {
        Some(up) => {
            if up.borrow().doors[3] {
                if path[path.len()-1].1 > 0 {
                    search(Some(up.clone()), path, true, best_path)
                } else {
                    (path, best_path)
                }
            } else {
                search(Some(up.clone()), path, false, best_path)
            }
        },
        None => { (path, best_path) }
    };

    path.pop();
    (path, best_path)
}


fn get_input_from_txt(file_path: String) -> Rc<RefCell<Node>> {
    let contents = fs::read_to_string(file_path).expect("Error reading file");

    let mut matrix: Vec<Vec<Rc<RefCell<Node>>>> = Vec::new();
    for _ in 0..6 {
        let mut row = Vec::new();
        for _ in 0..9 {
            row.push(Rc::new(RefCell::new(Node::default())));
        }
        matrix.push(row);
    }
    

    for (i, line) in contents.lines().enumerate() {
        let mut line_list = line.split_whitespace();

        let mut direction = line_list.next().unwrap().chars().enumerate();
        let mut doors = line_list.next().unwrap().chars().enumerate();
        let mut key_and_exit = line_list.next().unwrap().chars().enumerate();


        let left = if direction.next() == Some((0, '1')) {
            Some(Rc::clone(&matrix[i/9][i%9 - 1]))
            } else {
            None
        };
        let right = if direction.next() == Some((1, '1')) {
            Some(Rc::clone(&matrix[i/9][i%9 + 1]))
            } else {
            None
        };
        let up = if direction.next() == Some((2, '1')) {
            Some(Rc::clone(&matrix[i/9 - 1][i%9]))
            } else {
            None
        };
        let down = if direction.next() == Some((3, '1')) {
            Some(Rc::clone(&matrix[i/9 + 1][i%9]))
            } else {
            None
        };

        let key = key_and_exit.next().unwrap().1 == '1' && key_and_exit.next().unwrap().1 == '1';
        let exit = key_and_exit.next().unwrap().1 == '1' || key_and_exit.next().unwrap().1 == '1';
        

        let mut node_mut = matrix[i/9][i%9].borrow_mut();

        node_mut.position = [(i/9).try_into().unwrap(), (i%9).try_into().unwrap()];
        node_mut.key = key;
        node_mut.doors = [doors.next().unwrap().1 == '1', doors.next().unwrap().1 == '1', doors.next().unwrap().1 == '1', doors.next().unwrap().1 == '1'];
        node_mut.exit = exit;
        node_mut.left = left.map(|n| Rc::clone(&n));
        node_mut.right = right.map(|n| Rc::clone(&n));
        node_mut.up = up.map(|n| Rc::clone(&n));
        node_mut.down = down.map(|n| Rc::clone(&n));     
    }

    matrix[0][0].clone()
}











