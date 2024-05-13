fn main() {
    println!("Hello, world!");
}

fn expanded_form(n: u64) -> String {
    let mut number_in_expanded_form: Vec<String> = Vec::new();
    let mut number_to_expand = n as i64;


    while number_to_expand > 0 {

        let count_number_of_zeros = number_to_expand.to_string().chars().count() - 1;
        let number_of_zeros =  count_number_of_zeros as u32;
        let one_with_right_zeros = i64::pow(10, number_of_zeros);
        let first_digit_on_left= number_to_expand / one_with_right_zeros;
        let first_digit_with_zeros = first_digit_on_left * one_with_right_zeros;
        number_in_expanded_form.push((first_digit_with_zeros).to_string());
        number_to_expand = number_to_expand - first_digit_with_zeros;
    }

    return number_in_expanded_form.join(" + ");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(expanded_form(12), "10 + 2");
        assert_eq!(expanded_form(42), "40 + 2");
        assert_eq!(expanded_form(70304), "70000 + 300 + 4");
    }
}



