pub mod my_buf;
pub mod my_compare;

use my_buf::Buffer;
use my_compare::compareString;
fn main() {
    // Test for My Buf
    let addup = Buffer {
        content: vec![1, 1, 4, 5, 1, 4],
    };
    println!("Sum {:?}= {}", addup, addup.sum());
    let addup = Buffer {
        content: vec![3.14159, 2.71828, 0.99],
    };
    println!("Sum {:?}= {}", addup, addup.sum());

    // Test for My String Compare
    println!("abc abcd => {}", compareString("abc", "abcd"));
    println!("114514 114514 => {}", compareString("114514", "114514"));
    println!("xyz abcdef => {}", compareString("xyz", "abcdef"));

    // Test for My Batch Increment
    let origin = vec!['a', 'b', 'c', 'd', 'e'];
    let generated: Vec<char> = origin
        .iter()
        .map(|c| char::from_u32(*c as u32 + 1).unwrap())
        .collect();
    println!("New Vec: {:?}", generated);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let addup = Buffer {
            content: vec![1, 2, 3, 4, 5, 6, 7],
        };
        assert_eq!(addup.sum(), 28);
        let addup = Buffer {
            content: vec![1.0, 2.0, 3.0],
        };
        assert!((addup.sum() as f64 - 6.0).abs() < 0.0001, "Oops.");
    }

    #[test]
    fn test_str_cmp() {
        assert_eq!(compareString("abc", "abcdefgh"), false);
        assert_eq!(compareString("aaa", "aaa"), false);
        assert_eq!(compareString("z", "aaaaa"), true);
    }
    #[test]
    fn test_closure() {
        let origin = vec!['a', 'b', 'c', 'd', 'e'];
        let generated: Vec<char> = origin
            .iter()
            .map(|c| char::from_u32(*c as u32 + 1).unwrap())
            .collect();
        assert_eq!(generated, vec!['b', 'c', 'd', 'e', 'f']);
    }
}
