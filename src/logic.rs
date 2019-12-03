// logic contains the actual rust code we want to test, not any bindings
pub fn sum(a: u32, b: u32) -> u32 {
    a + b
}

pub fn count(word: &str) -> usize {
    word.len()
}

pub fn concat(a: &[u8], b: &[u8]) -> Vec<u8> {
    [a, b].concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sum() {
        assert_eq!(sum(2, 2), 4);
    }

    #[test]
    fn check_concat() {
        assert_eq!(concat(b"foo", b"bar"), b"foobar");
    }

    #[test]
    fn check_count() {
        assert_eq!(count("one time"), 8);
    }
}