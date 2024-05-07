use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn update_light(current: &str) -> String {
    let traffic_light_change_map = HashMap::from([
        ("green", "yellow"),
        ("yellow", "red"),
        ("red", "green"),
    ]);

    return traffic_light_change_map.get(current).unwrap().to_string();
}


#[test]
fn basic_test() {
    assert_eq!(update_light("green"), "yellow");
    assert_eq!(update_light("yellow"), "red");
    assert_eq!(update_light("red"), "green");
}


