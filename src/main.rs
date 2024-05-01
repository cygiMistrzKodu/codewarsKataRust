fn main() {
    println!("Hello, world!");
}

fn cockroach_speed(s: f64) -> i64 {
    // let km_per_hour = s;
    // let cm_per_hour = km_per_hour * 100000f64;
    // let cm_per_second = cm_per_hour / 3600f64;

    return (s * 100000f64 / 3600f64) as i64;

    // return cm_per_second as i64;
}


#[cfg(test)]
mod sample_tests {
    use super::cockroach_speed;

    #[test]
    fn basic_tests() {
        test(1.08, 30);
        test(1.09, 30);
        test(0.0, 0);
    }

    fn test(speed: f64, expected: i64) {
        let actual = cockroach_speed(speed);
        assert_eq!(actual, expected, "\nYour result (left) did not match the expected result (right) for the speed {speed}");
    }
}
