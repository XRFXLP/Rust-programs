use std::collections::HashMap;

fn isWheel(adj: Vec<Vec<bool>>) -> bool {
    let graph : Vec<Vec<u32>> = adj.iter()
                                   .map(|x| x.iter()
                                             .enumerate()
                                             .filter(|(_i, j)| **j == true)
                                             .map(|(i, _j)| i as u32)
                                             .collect())
                                   .collect();
    let mut m_l = 0;
    let mut m_i = 0;
    for i in 0..graph.len(){
        if graph[i].len() > m_l {
            m_l = graph[i].len();
            m_i = i;
        }
    }

    if m_l != graph.len() - 1 {return false;}
    let mut q: HashMap::<&u32, u32> = HashMap::new();
    for i in graph[m_i].iter(){q.insert(i, 0);}

    for i in graph[m_i].iter() {
        for j in graph[*i as usize].iter() {
            if *i == m_i as u32 { continue; }
            if j == i {return false;}
            *q.get_mut(j).unwrap_or(&mut 0) = (q.get(j).unwrap_or(&0) + 1) as u32;
        }
    }

    for (i, j) in q {
        if j != 2{
            return false;
        }
    }
    true
}
