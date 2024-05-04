fn main() {
    println!("Hello, world!");
}

fn invert(values: &[i32]) -> Vec<i32> {
    let mut inverted_values: Vec<i32> = Vec::new();

    for number in values {
        inverted_values.push(number * -1)
    }
    return inverted_values;
}

#[cfg(test)]
mod tests {
    use super::invert;

    #[test]
    fn basic_tests() {
        assert_eq!(invert(&vec![1, 2, 3, 4, 5]), vec![-1, -2, -3, -4, -5]);
        assert_eq!(invert(&vec![1, -2, 3, -4, 5]), vec![-1, 2, -3, 4, -5]);
        assert_eq!(invert(&vec![]), vec![]);
    }
}

