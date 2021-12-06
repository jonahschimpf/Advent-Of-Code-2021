#[derive(Clone)]
struct Cell {
   value: i32,
   marked: bool,
}

impl Cell {
    fn mark(mut self) {
        self.marked = true;
    }
}
#[derive(Clone)]
struct Matrix {
    cells: Vec<Vec<Cell>>,
    id: i32,
}

impl Matrix {
    //expect string to look like 2 3 4
    //                           3 4 5
    //                           4 2 1 
    fn new(matrix_string: String, id: i32) -> Matrix  {
        let lines: Vec<String> = matrix_string.lines()
            .map(|x| x.to_string())
            .collect();
        let mut cells: Vec<Vec<Cell>> = Vec::new();
        for line in lines.iter(){
           let mut row : Vec<Cell> = Vec::new(); 
           line.split_whitespace().for_each(|string| {
             let cell : Cell = Cell {value: string.parse().unwrap(), marked: false};
             row.push(cell);
           });
          cells.push(row); 
        }
        let matrix: Matrix = Matrix {cells, id};
        return matrix;
    }
}
use std::fs;
fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let stuff: Vec<String> = contents.split("\n\n").map(|x| x.to_string()).collect();
    let input_line: Vec<i32> = stuff[0].split(",").map(|x| x.parse().unwrap()).collect();
    let mut matrix_vec: Vec<Matrix> = Vec::new();

    for matrix_index in 1..stuff.len() {

       let matrix_string = stuff[matrix_index].clone(); 

       let matrix:Matrix = Matrix::new(matrix_string, matrix_index as i32);

       matrix_vec.push(matrix);
    }
    for special_num in input_line {
       for matrix in matrix_vec.iter_mut() {
            let coordinate = try_mark(matrix, special_num);
            if let Some((row, col)) = coordinate {
                if verify_win(&matrix, row, col) {
                    println!("id {} won with val {}", matrix.id, win_value(&matrix, special_num));
                    return;
                }

            }
       }
    }
    println!("whooops");
}

fn try_mark(matrix: &mut Matrix, special_num: i32) -> Option<(i32, i32)> {
    for (row_index, row) in matrix.cells.iter_mut().enumerate() {
       for (col_index, cell) in row.iter_mut().enumerate() {
            if cell.value == special_num {
                cell.marked = true;
                println!("Marking row: {} col: {} for matrix index {}, \n special_num: {}",
                    row_index, col_index, matrix.id, special_num);
                return Some((row_index as i32, col_index as i32));
            }
       }
    }
    return None;
}

fn verify_win(matrix: &Matrix, row: i32, col:i32) -> bool {
    let mut row_win = true;
    let mut col_win = true;
    for cell in matrix.cells[row as usize].iter() {
       if !cell.marked {
            row_win = false;
            break;
       }
    }
    if row_win {
        return true;
    }
    for row in matrix.cells.iter() {
        if !row[col as usize].marked {
            col_win = false;;
            break;
        }
    }
    if col_win {
        return true;
    }
    return row_win || col_win;
}

fn win_value(matrix: &Matrix, special_num: i32)-> i32 {
    let mut sum = 0;
   for row in matrix.cells.iter() {
        for cell in row.iter() {
            if !cell.marked {
               sum += cell.value; 
            }
        }
   }
   return sum * special_num;
}
