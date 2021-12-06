#[derive(Clone)]
struct Cell {
   value: i32, //number of times overlap
   counted: bool,
}

impl Cell {
}
#[derive(Clone)]
struct Matrix {
    cells: Vec<Vec<Cell>>,
}

impl Matrix {
    fn new(rows: i32, cols: i32) -> Matrix {
       let mut cells: Vec<Vec<Cell>> = Vec::new();
       for _row in 0..rows + 1 {
           let mut list: Vec<Cell> = Vec::new();
           for _c in 0..cols + 1 {
               list.push(Cell{ value: 0, counted: false }); 
           }
           cells.push(list);
       }
       return Matrix {cells};

    }
}
struct Pair {
    x : i32,
    y : i32
}
use std::fs;
fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lin_vec: Vec<String> = contents.lines().into_iter().map(|x| x.to_string()).collect();
    //vertz is Vec<<Pair, Pair>>, gauranteed col1 = col2, analog for hortz
    let mut col_size = 0;
    let mut row_size = 0;
   let (vertz, hortz, diags) = get_vertz_and_hortz_and_diags(&lin_vec, &mut col_size, &mut row_size); 
println!("Found col_size {} row_size {} ", col_size, row_size);
   let mut matrix: Matrix = Matrix::new(row_size, col_size);
   let mut count = 0;
   for (p1, p2) in vertz.iter() {
      let top = if p1.y > p2.y { p1.y } else { p2.y };
      
      let bottom = if p1.y <=  p2.y { p1.y } else { p2.y };
       for y in bottom..top+1 {
           let cell = &mut matrix.cells[y as usize][p1.x as usize];
           cell.value += 1;
           if cell.value >= 2 && !cell.counted {
               println!("Counted (vertz) row {} col {}", y, p1.x);
                cell.counted = true;
                count += 1;
           }
       }
   }
   for (p1, p2) in hortz.iter() {
      let top = if p1.x > p2.x { p1.x } else { p2.x };
      
      let bottom = if p1.x <=  p2.x { p1.x } else { p2.x };
        for x in bottom..top+1 {
           let cell = &mut matrix.cells[p1.y as usize][x as usize];
           cell.value += 1;
           if cell.value >= 2 && !cell.counted {
                cell.counted = true;
               println!("Counted (hortz) row {} col {}", p1.y, x);
                count += 1;
        }
       }
   }

   for (p1, p2) in diags.iter() {
        let mut x_increase = false;
        let mut y_increase = false;

       if p2.x >  p1.x {
            x_increase = true;
       }

       if p2.y > p1.y {
            y_increase = true;
       }

       let until = (p2.y - p1.y).abs() + 2;
        
       let mut x = p1.x;
       let mut y = p1.y;
       for _i in 1..until {
            let cell = & mut matrix.cells[y as usize][x as usize];
           cell.value += 1;
           if cell.value >= 2 && !cell.counted {
                cell.counted = true;
               println!("Counted (hortz) row {} col {}", p1.y, x);
                count += 1;
        }
        if x_increase {
            x += 1;
        } else {
            x -= 1;
        }
        if y_increase {
            y += 1;
        } else {
            y -= 1;
        }

            
       }
   }



   println!("The count is {}", count);
}


fn get_vertz_and_hortz_and_diags(contents: &Vec<String>, cols_size: &mut i32, row_size: &mut i32) -> (Vec<(Pair, Pair)>, Vec<(Pair, Pair)>, Vec<(Pair, Pair)>) {
    let mut vertz: Vec<(Pair, Pair)> = Vec::new(); 

    let mut hortz: Vec<(Pair, Pair)> = Vec::new(); 

    let mut diags: Vec<(Pair, Pair)> = Vec::new();

    for line in contents.iter() {
        let p1_and_p2: Vec<String> = line.split(" -> ").map(|x| x.to_string()).collect();
        let p1: Vec<String> = p1_and_p2[0].split(",").map(|x| x.to_string()).collect();
        let p2: Vec<String> = p1_and_p2[1].split(",").map(|x| x.to_string()).collect();
        let pair1: Pair = Pair {x : p1[0].parse().unwrap(), y: p1[1].parse().unwrap()};
        let pair2: Pair = Pair {x : p2[0].parse().unwrap(), y: p2[1].parse().unwrap()};

        //check bounds of matrix
        if pair1.x > *cols_size {
            *cols_size = pair1.x;
        }
        if pair2.x > *cols_size {
            *cols_size = pair2.x;
        }

        if pair1.y > *row_size {
            *row_size = pair1.y;
        }
        if pair2.y > *row_size {
            *row_size = pair2.y;
        }
        if pair1.x == pair2.x {
            vertz.push((pair1, pair2));
        } else if pair1.y == pair2.y {
            hortz.push((pair1, pair2));
        } else if (pair1.x - pair2.x).abs() == (pair1.y - pair2.y).abs() {
            diags.push((pair1, pair2));
        }
        
    }

    return (vertz, hortz, diags);
}
