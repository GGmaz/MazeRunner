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
    println!("{:?}", matrix);
}


fn get_input_from_txt(filePath: String) -> Vec<Vec<Node>> {
    let contents = fs::read_to_string(filePath).expect("Error reading file");

    //let mut matrix:[[Option<Node>; 9]; 5];
    //let mut matrix:[[Option<Node>; 9]; 5] = [[None; 9]; 5];
    let mut matrix: Vec<Vec<Node>> = Vec::new();
    for _ in 0..5 {
        let mut row = Vec::new();
        for _ in 0..9 {
            row.push(Node::default());
        }
        matrix.push(row);
    }
    

    for (i, line) in contents.lines().enumerate() {
        let mut lineList = line.split_whitespace();

        let mut direction = lineList.next().unwrap().chars().enumerate();
        //let doors:i32 = lineList.next().unwrap().parse::<i32>().unwrap();
        let mut doors = lineList.next().unwrap().chars().enumerate();
        let mut keyAndExit = lineList.next().unwrap().chars().enumerate();


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

        let key = keyAndExit.next().unwrap().1 == '1' && keyAndExit.next().unwrap().1 == '1';
        let exit = keyAndExit.next().unwrap().1 == '1' && keyAndExit.next().unwrap().1 == '1';
            
        let node = Node {
            position: [(i/9).try_into().unwrap(), (i%9).try_into().unwrap()],
            //doors: [doors.next().unwrap().1.to_string().parse().unwrap(), doors.next().unwrap().1.to_string().parse().unwrap(), doors.next().unwrap().1.to_string().parse().unwrap(), doors.next().unwrap().1.to_string().parse().unwrap()],
            doors: [false, false, false, false],
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
