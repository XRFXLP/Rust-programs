use std::cmp::min;
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
    for i in 0..words.len()-1{
        let k = mesh(words[i], words[i+1]);
        if k.len() > 0{
            s.push_str(&k);
        }
        else {
            return None;
        }
    }
    Some(s)
}
