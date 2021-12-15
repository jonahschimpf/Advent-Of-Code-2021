use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::ascii::AsciiExt;
struct Node {
    v: String,
    has_lower: bool
}


//number of paths s-t = sum (number of paths u -t where u in v _adj) 

fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);
    let mut graph : HashMap<String, Vec<String>>= HashMap::new();
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|l| {
          let mut x = l.split("-");  
          let v1 = x.next().unwrap().to_string();
          let v2 = x.next().unwrap().to_string();
          graph.entry(v1.clone()).or_insert(Vec::new()).push(v2.clone());
          graph.entry(v2.clone()).or_insert(Vec::new()).push(v1.clone());
        });
    for node in graph.keys() {
        println!("node: {}", node);
        println!("List: {:?}", graph.get(node).unwrap());
    }
     println!("Ekk {}", dfs(&graph, Node {v: "start".to_string(), has_lower: false}, &mut HashSet::new(), 0));
}

fn lowercase(s: &String) -> bool{
    return s != "end" && s.chars().all(|x| x.is_lowercase());
}

fn dfs(graph: &HashMap<String, Vec<String>>, start: Node, seen: &mut HashSet<String>, level: i32) -> u64 {
   if start.v == "end" {
       println!("Level: {} end", level);
        return 1;
   } else {
        let mut sum = 0;
        println!("Level: {} popped {}", level, start.v);

        for neigh in graph.get(&start.v).unwrap() {
            if neigh == "start" {
                continue;
            }
            let mut seen_clone = seen.clone();
            let mut cave = start.has_lower;
            if lowercase(neigh) {
                if seen.contains(neigh) && start.has_lower {
                    continue;
                } else {
                    let not_seen_before = seen_clone.insert(neigh.clone());
                    cave |= !not_seen_before;

                }
            }
            sum += dfs(graph, Node{v : neigh.to_string(), has_lower: cave}, &mut seen_clone, level + 1);
        }
        return sum;
    }
}
