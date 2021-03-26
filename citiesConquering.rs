/*
Once upon a time, in a kingdom far, far away, there lived a King Byteasar VII. Since he reigned during the Dark Times, very little is known about his reign. It is known that when he ascended the throne, there were n cities in the kingdom, connected by several two-way roads. But before the young king managed to start ruling, an enemy arrived at the gates: the evil emperor Bugoleon invaded the kingdom and started to conquer the cities, advancing day after day.

It is not known how long it took to capture each of the cities, but you've recently discovered an ancient manuscript describing that each day, Bugoleon captured all the cities that had at most 1 neighboring free city at that given moment. Knowing this fact, the number of cities n and all the roads of the kingdom, determine in how many days each of the cities was conquered.

*/

fn citiesConquering(n: i32, roads: Vec<Vec<i32>>) -> Vec<i32>{
    let mut graph = vec![vec![]; n as usize];
    for i in roads.iter(){
        graph[i[0] as usize].push(i[1]);
        graph[i[1] as usize].push(i[0]);
    }
    let mut repo = vec![-1; n as usize];
    let mut day = 1;
    loop {
        let mut to_remove : Vec<i32> = Vec::new();
        for i in 0..graph.len(){
            if graph[i] != vec![-1] && graph[i].len() < 2{
                to_remove.push(i as i32);
            }
        }
        for i in 0..graph.len(){
            graph[i] = graph[i].iter()
                               .map(|x| *x)
                               .filter(|y| !to_remove.contains(y))
                               .collect::<Vec<_>>();
        }
        for i in (&to_remove).iter().rev(){
            repo[(*i) as usize] = day;
            graph[(*i) as usize] = vec![-1];
        }
        if to_remove.len() == 0{
            break;
        }
        day+=1;
    }
    repo
}
fn main(){


    let roads : Vec<Vec<i32>> = vec![vec![1, 0], vec![1, 2], vec![8, 5], vec![9, 7],
         vec![5, 6], vec![5, 4], vec![4, 6], vec![6, 7]];
    let size : i32 = 10;
    println!("{:?}", citiesConquering(size, roads));

}
