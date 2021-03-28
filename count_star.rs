fn countStars(adj: Vec<Vec<bool>>) -> i32{
    let graph : Vec<Vec<u32>> = adj.iter()
                                   .map(|x| x.iter()
                                             .enumerate()
                                             .filter(|(i, j)| **j == true)
                                             .map(|(i, j)| i as u32)
                                             .collect())
                                   .collect();
    graph.iter().enumerate()
                .filter(|(i, j)| j.len() > 0 && j.iter().filter(|&&x| graph[x as usize].len() == 1).count() == j.len())
                .map(|(i, x)| if x.len() == 1 { 0.5_f32 } else {1 as f32})
                .sum::<f32>() as i32
}
                  
