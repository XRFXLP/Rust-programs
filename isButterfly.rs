fn isButterfly(adj: Vec<Vec<bool>>) -> bool {
    let mut p = adj.iter().map(|x| x.iter().filter(|y|**y==true).count()).collect::<Vec<usize>>();
    let mut mid = p.iter().position(|&x| x == 4).unwrap_or(5);
    if mid == 5 || adj.iter().enumerate().filter(|&(i, y)| if i == mid {y[mid] == false} else {y[mid] == true}).count() != 5 {
        return false;
    }
    p.sort();
    p == vec![2, 2, 2, 2, 4] && adj.iter().enumerate().filter(|&(i, y)| y[i] == true).count() == 0
}
