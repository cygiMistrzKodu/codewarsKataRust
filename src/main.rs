fn main() {
    println!("Hello, world!");
}

fn update_light(current: &str) -> String {
    // Code goes here..
    return "".to_string();
}


#[test]
fn basic_test() {
    assert_eq!(update_light("green"), "yellow");
    assert_eq!(update_light("yellow"), "red");
    assert_eq!(update_light("red"), "green");
}


