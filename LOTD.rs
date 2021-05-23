use std::collections::HashMap;
fn livingOnTheRoads(roadRegister: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let graph : Vec<Vec<u32>> = roadRegister.iter()
                                   .map(|x| x.iter()
                                             .enumerate()
                                             .filter(|(_i, j)| **j == true)
                                             .map(|(i, _j)| i as u32)
                                             .collect())
                                   .collect();
    let mut edges = Vec::new();
    for (i, j) in graph.iter().enumerate() {
        for n in j.iter() {
            let m = *n as usize;
            let curr = (if m < i {m} else {i}, if m < i {i} else {m});
            if !edges.contains(&curr){edges.push(curr);}
        }
    }
    edges.sort();
    let mut m = HashMap::new();
    for (i, j) in edges.iter().enumerate() {
        m.insert(j, i);
    }

    //words?

    let mut adj : Vec<Vec<bool>> = (0..edges.len()).map(|_i| (0..edges.len()).map(|_j| false).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>();
    for (i, j) in graph.iter().enumerate() {
        for k in j.iter() {
            for l in j.iter() {
                if k == l { continue; }
                let a = *k as usize;
                let b = *l as usize;
                let c = i as usize;
                let node1 = if a < c {(a,c)} else {(c,a)};
                let node2 = if b < c {(b, c)} else {(c,b)};
                adj[m[&node1]][m[&node2]] = true;
            }
        }
    }
    adj
}
