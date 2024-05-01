fn main() {
    println!("Hello, world!");
}

fn double_char(s: &str) -> String {
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use super::double_char;

    fn dotest(s: &str, expected: &str) {
        let actual = double_char(s);
        assert_eq!(actual, expected, "With s = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn test_hello_world() {
        dotest("Hello World", "HHeelllloo  WWoorrlldd")
    }
    #[test]
    fn test_numbers() {
        dotest("1234!_ ", "11223344!!__  ")
    }
}

