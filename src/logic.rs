// logic contains the actual rust code we want to test, not any bindings
pub fn sum(a: u32, b: u32) -> u32 {
    a + b
}

pub fn count(word: &str) -> usize {
    word.len()
}

pub fn concat(a: &str, b: &str) -> String {
    [a, b].concat()
}

#[repr(C)]
pub struct Foo {
    pub count: i32
}

pub fn new_foo(count: i32) -> Foo {
    Foo{count}
}

impl Foo {
    pub fn multiply(&self, val: i32) -> i32 {
        self.count * val
    }

    pub fn update(&mut self, count: i32) {
        self.count = count
    }
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
        assert_eq!(concat("foo", "bar"), "foobar");
    }

    #[test]
    fn check_count() {
        assert_eq!(count("one time"), 8);
    }
}