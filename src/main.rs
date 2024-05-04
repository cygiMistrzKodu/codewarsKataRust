fn main() {
    println!("Hello, world!");
}

fn string_to_array(s: &str) -> Vec<String> {
    return s.split(" ").map(|v| v.to_string()).collect();
}

#[cfg(test)]
mod tests {
    use super::string_to_array;

    fn dotest(s: &str, expected: &[&str]) {
        let actual = string_to_array(s);
        assert_eq!(actual, expected, "Test failed with s = \"{s}\"\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest("Robin Singh", &["Robin", "Singh"]);
        dotest("CodeWars", &["CodeWars"]);
        dotest("I love arrays they are my favorite", &["I", "love", "arrays", "they", "are", "my", "favorite"]);
        dotest("1 2 3", &["1", "2", "3"]);
    }
}

