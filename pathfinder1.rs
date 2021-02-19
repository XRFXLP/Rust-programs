use std::collections::HashSet;

fn path_finder( st : &str) -> bool {
    let maze = st.split_whitespace().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut visited : HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    let mut stack : Vec<(i32, i32)> = Vec::new();
    let n : i32 = maze.len() as i32;
    stack.push((0, 0));
    while stack.len() > 0{
        let (y, x) = stack.pop().unwrap_or((0, 0));
        if (y, x) == (n - 1, n - 1){
            return true;
        }

        for (y_, x_) in [(y, x + 1), (y, x - 1), (y + 1, x), (y - 1, x)].iter(){
            if -1 < *y_ && *y_ < n && -1 < *x_ && *x_ < n && maze[*y_ as usize][*x_ as usize] == '.' && !visited.contains(&(*y_, *x_)){
                stack.push((*y_, *x_));
            }
        }
        visited.insert((y, x));
    }
    false
}
