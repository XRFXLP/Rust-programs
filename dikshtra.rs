use std::collections::BinaryHeap;

fn orienteeringBeginner(n: i32, roads: Vec<Vec<Vec<i32>>>) -> i32{
    let mut distance = vec![429496729; n as usize];
    let mut heap = BinaryHeap::new();
    heap.push((0, 0));
    while heap.len() > 0 {
        let (_i, c) = heap.pop().unwrap();
        if distance[c] < _i {
            continue;
        }
        for p in &roads[c]{
            let d = _i + p[1];
            if d < distance[p[0] as usize]{
                distance[p[0] as usize] = d;
                heap.push((d, p[0] as usize));
            }
        }
    }
    distance[n as usize -1]
}
  

fn main(){

    let road = vec![vec![vec![1, 3]],
         vec![vec![4, 1], vec![3, 2]],
         vec![vec![1, 0]],
         vec![vec![5, 3]],
         vec![vec![5, 5]],
         vec![]];
    let n = 6;
    println!("{}", orienteeringBeginner(n, road));
}
