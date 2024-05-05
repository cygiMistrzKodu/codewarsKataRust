fn main() {
    println!("Hello, world!");
}

fn add_binary(a: u64, b: u64) -> String {
    return format!("{:b}", a + b);
}

#[cfg(test)]
mod tests {
    use super::add_binary;

    fn dotest(a: u64, b: u64, expected: &str) {
        let actual = add_binary(a, b);
        assert_eq!(actual, expected, "With a = {a}, b = {b}\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn sample_tests() {
        dotest(1, 1, "10");
        dotest(0, 1, "1");
        dotest(1, 0, "1");
        dotest(2, 2, "100");
        dotest(51, 12, "111111");
    }
}


