use std::fs;
use std::collections::VecDeque;
#[derive(Copy, Clone)]
struct Point {
    row: i32,
    col: i32
}

struct Cell {
    p: Point,
    value: u32,
    has_flashed: bool,
}



fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
  

    let mut matrix: Vec<Vec<Cell>> = contents.lines().enumerate().map(|(i, l)| {
        l.chars().enumerate().map(|(j, c)| {
            let p = Point {row: i as i32,col: j as i32};
            Cell{value: c.to_digit(10).unwrap(), has_flashed: false, p}
        }).collect()
    }).collect();
    let cells = matrix.len() * matrix[0].len();
    for i in 1..100000 {
        if cells as i32== get_flash(&mut matrix) {
            println!("EEL {}", i);
            break;
        }
        matrix.iter_mut().for_each(|v| v.iter_mut().for_each(|x| x.has_flashed = false));
    }
}

fn get_flash(matrix: &mut Vec<Vec<Cell>>) -> i32 {
   let mut queue: Vec<Point> = Vec::new();
    
   
       for l in matrix.iter_mut() {
        for c in l.iter_mut() {
           c.value += 1; 
           if c.value == 10{
                c.value = 0;
                c.has_flashed = true;
                queue.push(c.p);
           }
        }
    }
   
    return fun(matrix, &mut queue);
}

fn fun(matrix: &mut Vec<Vec<Cell>>, queue: &mut Vec<Point>) -> i32 {
    let mut flashes = 0;
    while !queue.is_empty() {
        println!("Queue size is {}", queue.len());
        let p = queue.pop().unwrap();
        let low_row = p.row - 1;
        let high_row =p.row + 1;
        
        let low_col = p.col - 1;
        let high_col = p.col + 1;
        let len = matrix.len() as i32;
        flashes += 1;
        for i in low_row..high_row + 1 {
           if i < 0 || i >= len {
                continue;
           } else {
                for j in low_col..high_col + 1 {
                    if j < 0 || j >= matrix[i as usize].len() as i32{
                        continue;
                    } else {
                        if matrix[i as usize][j as usize].has_flashed { 
                            continue;
                        }
                        matrix[i as usize][j as usize].value += 1;
                        if matrix[i as usize][j as usize].value == 10 {
                            matrix[i as usize][j as usize].value = 0;
                            matrix[i as usize][j as usize].has_flashed = true;
                            queue.push(matrix[i as usize][j as usize].p);
                        }
                    }
                    
                }
           }
        }
    }
    println!("Returning {} flashes", flashes);
    return flashes;
}
