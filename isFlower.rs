use std::collections::HashMap;

fn isFlower(adj: Vec<Vec<bool>>) -> bool {
    let graph : Vec<Vec<i32>> = adj.iter()
                                   .map(|x| x.iter()
                                             .enumerate()
                                             .filter(|(_i, j)| **j == true)
                                             .map(|(i, _j)| i as i32)
                                             .collect())
                                   .collect();
    let center = graph.iter().enumerate().filter(|(_p, x)| x.len() as i32 == graph.len() as i32 - 1).next().unwrap_or((graph.len(), &vec![])).0;
    if center == graph.len() {return false; }
    let lengths = graph.iter().enumerate().filter(|&(p, _x)| p != center).map(|(_i, x)| x.len()).collect::<Vec<usize>>();
    for &i in lengths.iter(){
        if i != lengths[0] { return false; }
    }

    let mut petals = vec![];
    let mut seen = HashMap::new();
    for (i, j) in graph.iter().enumerate() {
        if i == center { continue; }
        if seen.contains_key(&i) {
            let mut temp = j.clone();
            temp.push(i as i32);
            temp.sort();
            if temp != petals[seen[&i]] {
                return false;
            }
        }
        else{
            seen.insert(i, petals.len());
            let mut temp = j.clone();
            temp.push(i as i32);
            temp.sort();
            petals.push(temp);
            for k in graph[i].iter() {
                seen.insert(*k as usize, seen[&i]);
            }
        }
    }
    true
}
