#[allow(non_snake_case)]
pub fn compareString(x: &str, y: &str) -> bool {
    let chars1 = x.chars().collect::<Vec<char>>();
    let chars2 = y.chars().collect::<Vec<char>>();

    for (c1, c2) in chars1.iter().zip(chars2.iter()) {
        if c1 < c2 {
            return false;
        } else if c1 > c2 {
            return true;
        }
    }

    chars1.len() > chars2.len()
}
