fn main() {
    println!("Hello, world!");
}

fn series_sum(n: u32) -> String {
    if n == 0 {
        return "0.00".to_string();
    }

    let mut sum_of_series: f64 = 1f64;
    let mut divider: f64 = 4f64;
    for _index in 0..n - 1 {
        sum_of_series = sum_of_series + 1f64 / divider;
        divider += 3f64;
    }

    return format!("{:.2}", sum_of_series);
}

#[cfg(test)]
mod tests {
    use super::series_sum;

    fn test(input: u32, expected: &str) {
        let actual = series_sum(input);
        assert_eq!(actual, expected, "Expected series_sum({input}) to be {expected}, but was {actual}");
    }

    #[test]
    fn sample_tests() {
        test(1, "1.00");
        test(2, "1.25");
        test(3, "1.39");
        test(7, "1.68");
        test(39, "2.26");
        test(0, "0.00");
    }
}


