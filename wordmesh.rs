use std::cmp::min;
use itertools::Itertools;
fn mesh(a: &str, b: &str) -> String {
    let lena = a.len();
    for i in (0..=min(b.len(), a.len()) - 1).rev() {
        if &a[lena-1-i..] == &b[..i+1] {
            return String::from(&b[..i+1]);
        }
    }
    "".to_string()
}


fn word_mesh(words :  &[&str]) -> Option<String> {
    let mut s = "".to_string();
    for k in words.iter().tuple_windows().map(|(x, y)| mesh(x,y)).collect::<Vec<String>>().iter(){
        if k.len() > 0{
            s.push_str(&k);
        }
        else {
            return None;
        }
    }
    Some(s)
}
