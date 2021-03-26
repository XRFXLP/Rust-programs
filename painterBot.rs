/*
Little Ratiorg was tired of being bullied by other bots and mighty CodeFighters, so he joined the Ninja Bots 
Training camp. Any bot that manages to solve all the challenges becomes an ultimate master of algorithms, and that is exactly what Ratiorg wants.

Ratiorg aced the very first challenge, and is ready to begin the second. The little bot is given a canvas,
each pixel of which has some color. Ratiorg should apply several flood fill operations to the canvas, and 
show the resulting image to the judges. Each flood fill operation is given as an array of three elements
[x, y, color], where (x, y) is the coordinates of the pixel to which the operation is applied, and color 
is the color with which the pixel and its area should be painted. The area that should be painted along with the initial pixel is defined as follows:

Initially, only the pixel to which the operation was applied should be painted.
Consider all pixels that are adjacent to the initial one (i.e. directly above, below, to the left or to the 
right of it). If the difference between the color of this pixel and the color of the initial one is not greater than d, this pixel should also be painted.
For each pixel painted on this step consider all its neighbors in the same manner.
Your task is to help the judges check Ratiorg's performance. Given canvas, operations and the value of d, 
return the state of the image after all operations have been applied.
*/
use std::collections::HashSet;

fn painterBot(c: Vec<Vec<i32>>, operations: Vec<Vec<i32>>, d: i32) -> Vec<Vec<i32>> {
    let mut canvas = c;
    for i in operations{
        let mut painted = HashSet::new();
        let mut stack = vec![(i[0],i[1])];
        while stack.len() > 0{
            let (y, x) = stack.pop().unwrap();
            painted.insert((y, x));
            for (y_, x_) in [(y, x + 1), (y, x - 1), (y + 1, x), (y - 1, x)].iter(){
                if -1 < *y_ && *y_ < (canvas.len() as i32) && -1 < *x_ && *x_ < (canvas[0].len() as i32)
                    && !painted.contains(&(*y_, *x_))
                    && (canvas[*y_ as usize][*x_ as usize] - canvas[i[0] as usize][i[1] as usize]).abs() <= d{
                    stack.push((*y_, *x_));
                }
            }
        }

        for (a, b) in painted{
            canvas[a as usize][b as usize] = i[2];
        }
    }
    canvas
}
                                                                                                             
                                                                                                             
fn main(){
    let canvas = vec![vec![0, 1, 5, 2, 4, 2, 6],
          vec![5, 2, 4, 6, 2, 0, 0],
          vec![3, 3, 3, 7, 8, 3, 2],
          vec![2, 1, 1, 0, 0, 0, 0]];
    let operations = vec![vec![0, 0, 10], vec![1, 3, 3]];
    let d = 3;
    println!("{:?}", painterBot(canvas, operations, d));

}
