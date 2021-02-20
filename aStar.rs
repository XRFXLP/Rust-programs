use std::collections::BinaryHeap;
use std::collections::HashMap;

fn distance(a: (i32, i32), b : (i32, i32)) -> i32 {
    let dy = (a.0 - b.0) as i32;
    let dx = (a.1 - b.1) as i32;

    - dy * dy - dx * dx
}

fn moves(y: i32, x: i32, maze: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut v = Vec::new();
    let h = maze.len() as i32;
    let w = maze[0].len() as i32;
    for (y_, x_) in [(y, x + 1), (y, x - 1), (y + 1, x), (y - 1, x), (y + 1, x + 1), (y + 1, x - 1), (y - 1, x - 1), (y - 1, x + 1)].iter(){
        if -1 < *y_ && *y_ < h && -1 < *x_ && *x_ < w && maze[*y_ as usize][*x_ as usize] != 'X'{
            v.push((*y_, *x_));
        }
    }
    v
}


fn reconstruct_path(current: (i32, i32), came_from: HashMap<(i32, i32), (i32, i32)>, maze: &mut Vec<Vec<char>>){
    let mut other = came_from[&current];
    while came_from.contains_key(&other){
        maze[other.0 as usize][other.1 as usize] = 'P';
        other = came_from[&other];
    }
}


fn a_star(m: String) -> String {
    let mut maze = m.trim()
                .split("\n")
                .map(|x| x.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();
            
    let h = maze.len();
    let w = maze[0].len();


    let mut start = (0 as i32, 0 as i32); let mut end = (0 as i32, 0 as i32);
    for i in 0..h {
        for j in 0..w {
            if maze[i][j] == 'S'{start = (i as i32, j as i32);};
            if maze[i][j] == 'G'{end = (i as i32, j as i32)};
        }
    }

    let mut pq = BinaryHeap::new();
    pq.push((distance(start, end), start));

    let mut fscore = HashMap::new();
    let mut gscore = HashMap::new();
    let mut came_from = HashMap::new();

    gscore.insert(start, 0);
    fscore.insert(start, distance(start, end));
    while pq.len() > 0{
        let (_, (y, x)) = pq.pop().unwrap_or((0, (0, 0)));
        if (y, x) == end{
            reconstruct_path(end, came_from, &mut maze);
            return maze.iter().map(|w| w.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("\n");
        }
        for p in moves(y, x, &maze).iter(){
            let tentative_gscore = gscore[&(y, x)] - distance((y, x), *p);
            if tentative_gscore < *gscore.get(p).unwrap_or(&(1<<24)){
                came_from.insert(*p, (y, x));
                fscore.insert(*p, tentative_gscore - distance((y, x), end));
                gscore.insert(*p, tentative_gscore);
                pq.push((fscore[p], *p)); 
            }
        }
    }
    "Oh for crying out loud...".to_string()
}


fn main(){
    let s = "
XX.S.XXX..
XXXXPX..XX
...XPXX...
XX..PXXX.X
...PXXX...
XXXXPP.XXX
X...XXP..X
X...X..PXX
XXXXXXXXPX
GPPPPPPP.X
".to_string();
   println!("{}", a_star(s));

}
