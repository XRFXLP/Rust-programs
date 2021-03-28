fn isBook(adj: Vec<Vec<bool>>) -> bool {
    let graph : Vec<Vec<u32>> = adj.iter()
                                   .map(|x| x.iter()
                                             .enumerate()
                                             .filter(|(_i, j)| **j == true)
                                             .map(|(i, _j)| i as u32)
                                             .collect())
                                   .collect();
    let bases : Vec<u32> = graph.iter()
                     .enumerate()
                     .filter(|(a,b)| b.len() == graph.len() - 1 && !b.contains(&(*a as u32)))
                     .map(|(a,_b)| a as u32)
                     .collect();

    if graph.len() != 3 && bases.len() != 2 { return false; }
    for i in 0..graph.len() {
        if !bases.contains(&(i as u32)){
            if graph[i].len() != 2{
                return false;
            }
            if !bases.contains(&graph[i][0]) || !bases.contains(&graph[i][1]) {
                return false;
            }
        }
    }
    true
}
