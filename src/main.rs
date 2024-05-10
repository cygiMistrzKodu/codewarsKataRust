fn main() {
    println!("Hello, world!");
}

fn validate_pin(pin: &str) -> bool {

    if (pin.chars().count() == 4 || pin.chars().count() == 6) && pin.chars().all(|x| x.is_ascii_digit()) {
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::validate_pin;

    #[test]
    fn invalid_length_tests() {
        assert_eq!(validate_pin("1"), false);
        assert_eq!(validate_pin("12"), false);
        assert_eq!(validate_pin("123"), false);
        assert_eq!(validate_pin("12345"), false);
        assert_eq!(validate_pin("1234567"), false);
        assert_eq!(validate_pin("-1234"), false);
        assert_eq!(validate_pin("1.234"), false);
        assert_eq!(validate_pin("-1.234"), false);
        assert_eq!(validate_pin("00000000"), false);
    }

    #[test]
    fn non_digit_chars_tests() {
        assert_eq!(validate_pin("a234"), false);
        assert_eq!(validate_pin(".234"), false);
    }

    #[test]
    fn valid_pin_tests() {
        assert_eq!(validate_pin("1234"), true);
        assert_eq!(validate_pin("0000"), true);
        assert_eq!(validate_pin("1111"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("098765"), true);
        assert_eq!(validate_pin("000000"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("090909"), true);
    }

    #[test]
    fn edge_case_tests() {
        assert_eq!(validate_pin("123"), false);
        assert_eq!(validate_pin("12345"), false);
        assert_eq!(validate_pin("1234567"), false);
        assert_eq!(validate_pin("1234567890"), false);
        assert_eq!(validate_pin("1234x"), false);
        assert_eq!(validate_pin("123456x"), false);
        assert_eq!(validate_pin("12.0"), false);
        assert_eq!(validate_pin("1234.0"), false);
        assert_eq!(validate_pin("123456.0"), false);
        assert_eq!(validate_pin("123"), false);
        assert_eq!(validate_pin(""), false);
        assert_eq!(validate_pin("1234"), true);
        assert_eq!(validate_pin(""), false);
        assert_eq!(validate_pin("09876"), false);
        assert_eq!(validate_pin(""), false);
        assert_eq!(validate_pin("098765"), true);

        assert_eq!(validate_pin("-111"), false);
        assert_eq!(validate_pin("111-"), false);
        assert_eq!(validate_pin("-44444"), false);
        assert_eq!(validate_pin("44444-"), false);
        assert_eq!(validate_pin("+111"), false);
        assert_eq!(validate_pin("+88888"), false);
        assert_eq!(validate_pin("+1111"), false);
        assert_eq!(validate_pin("-2018"), false);
        assert_eq!(validate_pin("+234567"), false);
        assert_eq!(validate_pin("-234567"), false);
        assert_eq!(validate_pin("123/"), false);
        assert_eq!(validate_pin("456:"), false);
        assert_eq!(validate_pin("9¾9¾"), false);
    }

    #[test]
    fn edge_case_2() {
        assert_eq!(validate_pin("9¾9¾"), false);

        assert!('¾'.is_numeric());
    }

    // #[test]
    // fn is_numeric_check() {
    //
    //     // assert!('¾'.is_numeric());
    //     // assert!('¾'.is_ascii_digit());
    //     // assert!("+111".chars().all(char::is_ascii_digit));
    //
    //     assert!("9¾9¾".chars().all(|x| x.is_ascii_digit()))
    //     // assert!("9¾9¾".chars().all(|x| x.is_numeric()))
    //
    //
    //     // assert!('r'.is_numeric());
    //     // assert!(a.iter().all(|&x| x > 0));
    //
    // }
}





