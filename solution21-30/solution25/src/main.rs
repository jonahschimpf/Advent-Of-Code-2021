use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::ascii::AsciiExt;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, PointStyle};
#[derive(PartialEq, Hash, Eq, Clone, Debug)]
struct Point {
    x : i32,
    y : i32
}

fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).unwrap();
    let mut o = contents.split("\n\n").into_iter();
    let mut matrix: HashMap<Point, bool> = HashMap::new();
                o.next().unwrap().lines().into_iter()
                .for_each(|t| {
                    let mut split = t.split(",");
                    let x = split.next().unwrap().parse().unwrap();
                    let y = split.next().unwrap().parse().unwrap();
                    matrix.insert(Point {x, y}, true);
                });
    let mut directions: Vec<(char, i32)> = Vec::new();
    o.next().unwrap().lines().into_iter()
        .for_each(|t| {
            let mut split = t.split("=");
            let axis = split.next().unwrap().chars().last().unwrap();
            let resolve = split.next().unwrap().parse().unwrap();
            directions.push((axis, resolve));
        });
    //fill out the rest of the matrix fuck you
    let (max_col, max_row) = get_dims(&matrix);
    for i in 0..max_col {
        for j in 0..max_row {
            let p = Point {x:i as i32, y:j as i32} ;
            if !matrix.contains_key(&p) {
                matrix.insert(p, false);
            }
        }
    }
    println!("{:?}", get_dims(&matrix));
    for direction in directions {
        matrix = fold(&matrix, direction.0, direction.1);

    }
    ///////////////////////////////
    //dafkdjaf
    //
    // Youre jek
let (mc, mr) = get_dims(&matrix);
   let mut data: Vec<(f64, f64)> = Vec::new(); 
   let mut black: Vec<(f64, f64)> = Vec::new();
   for (p, dot) in &matrix {
            let y = (p.y - mr).abs() as f64;
        if *dot {
            data.push((p.x as f64,y ));     
        } else {
            black.push((p.x as f64,y));     
        }
   }
   for row in 0..mr {
        for col in 0..mc {
            if *matrix.get(&Point {x: col, y:row}).unwrap() {
               print!("#"); 
            } else {
                print!(".");
            }
        }
        print!("\n");
   }
}

fn fold(matrix: &HashMap<Point ,bool>, axis: char, number: i32) ->HashMap<Point, bool>{
    let mut copy: HashMap<Point, bool> = HashMap::new();
    let (width, length) = get_dims(matrix);
    let mut d = 0;
    if axis == 'x' {
       for (p, dot) in matrix {
            if p.x < number {
                let corrps_x = number + (number - p.x);
                let other_true = matrix.get(&Point {x: corrps_x, y: p.y}).unwrap();
                
                let new_x = corrps_x - number - 1;
                let new_po = Point{ x: new_x, y: p.y};
                let has_dot = dot | other_true;
                copy.insert(new_po, has_dot);
                if has_dot {
                    d += 1;
                }
           }
       }
    } else {
        for (p, dot) in matrix {
        if p.y < number {
            let other_true = matrix.get(&Point {x: p.x, y: number + (number - p.y)}).unwrap();
            let has_dot = dot | other_true;
            copy.insert(p.clone(), has_dot);
            if has_dot {
                d += 1;
                println!("Point {:?}", p);
            }
        }
        }
    }
    println!("There are dots {}", d);
   return copy; 
}

fn get_dims(matrix: &HashMap<Point, bool>) -> (i32,i32) {
   let mut max_col = -1;
   let mut max_row = -1;
   for key in matrix.keys() {
        if max_col < key.x {
            max_col = key.x;
        }
        if max_row < key.y {
            max_row = key.y;
        }
   }
   return (max_col + 1, max_row + 1);
}
