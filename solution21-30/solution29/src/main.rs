//Something smells fishy...
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).unwrap();

    let mut array: Vec<Vec<u64>> = contents
                               .lines()
                               .map(|l| {
                                    l.chars()
                                    .map(|c| c.to_digit(10).unwrap() as u64)
                                    .collect()
                               })
                               .collect();

    let original_len = array.len();
    let origanl_col = array[0].len();
    //make first "row"
    for i in 1..5 {
        for row in 0..original_len {
            for col in 0..origanl_col {
                let new_value = if array[row][col] + (i as u64) >= 10 { //10 -> 1, 11 -> 2, 12 -> 3
                    array[row][col] + (i as u64) - 9
                } else {
                    array[row][col] + (i as u64)
                };
                if (new_value > 9) {
                    panic!("ahhh");
                }
                array[row].push(new_value);
            }
        }
    }
    for i in 1..5 {
        //make rows of tiles
        for new_row in (i * original_len)..(i + 1) * (original_len){
            array.push(Vec::new());
                for new_col in 0..origanl_col * 5 {
                   let new_value = if array[new_row - original_len][new_col] < 9 {
                       array[new_row - original_len][new_col] + 1
                   } else {
                       1
                   };
                   array[new_row].push(new_value); 
            }
        }
    }
    
    for i in &array {
        for j in i {
            print!("{}", j);
        }
        println!("\n");
    }

    let mut heap: BinaryHeap<Visit<Vertex>> = BinaryHeap::new();
    let mut graph: HashMap<Vertex, Vec<Vertex>>  = HashMap::new(); //vertex : adjlist of vertex
    let mut distances: HashMap<Vertex, u64> = HashMap::new();
    let mut visited: HashSet<Vertex> = HashSet::new();

    //add vertices
    for (row, v) in array.iter().enumerate() {
        for (col, x) in v.iter().enumerate() {
            let t = (row, col);;
            let tup = Vertex::new(t);
            graph.insert(tup, Vec::new());
        }
    }
    //add edges
    for (node, edges) in &mut graph {
       let (row, col) = node.id; 
       if (row > 0) {
            let up_id = (row - 1, col);
            edges.push(Vertex::new(up_id));
       }
       if (row < array.len() - 1) {
            let up_id = (row + 1, col);
            edges.push(Vertex::new(up_id));
       }
       if (col > 0) {
            let up_id = (row , col - 1);
            edges.push(Vertex::new(up_id));
       }
       if (col <  array[0].len() - 1) {
            let up_id = (row, col + 1);
            edges.push(Vertex::new(up_id));
       }
    }
    let start = Visit {vertex: Vertex::new((0,0,)), distance: 0}; 
    heap.push(start);

    while let Some(Visit {vertex, distance}) = heap.pop() {
        if visited.contains(&vertex) {
            continue; //already saw
        }
        visited.insert(vertex);
        for neighbor in graph.get(&vertex).unwrap() {
            let new_weight = distance + array[neighbor.id.0][neighbor.id.1];
            let should_change = distances.get(&neighbor).map_or(true, |&current_weight| new_weight < current_weight);
            if should_change {
                distances.insert(*neighbor , new_weight);
            }
            heap.push(Visit{vertex: *neighbor, distance: *distances.get(&neighbor).unwrap()});
        }
        
    }
    println!("Dist is {:?}", distances.get(&Vertex{ id: ( array.len() - 1, array[0].len() - 1)}));

}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vertex {
    id: (usize, usize),
}

impl Vertex {
    fn new(id:  (usize, usize)) -> Vertex {
        Vertex { id }
    }
}

#[derive(Debug)]
struct Visit<V> {
    vertex: V,
    distance: u64,
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}



