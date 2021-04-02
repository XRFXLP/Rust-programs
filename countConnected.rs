use std::collections::HashSet;
fn walkingInTheWoods(n: i32, wmap: Vec<Vec<i32>>) -> i32{
    let mut graph = vec![vec![]; n as usize];
    for i in wmap {
        graph[i[0] as usize].push(i[1]);
        graph[i[1] as usize].push(i[0]);
    }

    let mut visited = HashSet::new();
    let mut count : i32 = 0;
    for (i, j) in graph.iter().enumerate() {
        if !visited.contains(&i) {
            count += 1;
            let mut stack = Vec::new();
            stack.push(i);
            while stack.len() > 0 {
                let current = stack.pop().unwrap();
                for j in graph[current].iter() {
                    if !visited.contains(&(*j as usize)) {
                        stack.push(*j as usize);
                    }
                }
                visited.insert(current as usize);
            }
        }
    }
    count - 1
}
