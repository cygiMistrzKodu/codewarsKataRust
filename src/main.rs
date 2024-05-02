fn main() {
    println!("Hello, world!");
}

fn double_char(s: &str) -> String {

   let split_text_by_sign: Vec<&str> =  s.split("").collect();

    let mut double_sign_text: String = "".to_string();
    for sign in split_text_by_sign{
        double_sign_text += &*(sign.to_owned() + sign);
    }

    return double_sign_text;
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

