fn main() {
    println!("Hello, world!");
}

fn descending_order(x: u64) -> u64 {
    let number_int_string = x.to_string();
    let mut chars: Vec<char> = number_int_string.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));

    let sorted_by_digits_string = chars.into_iter().collect::<String>();

    return sorted_by_digits_string.parse::<u64>().unwrap();
}


#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}





