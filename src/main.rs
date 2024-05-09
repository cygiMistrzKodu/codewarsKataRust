
fn main() {
    println!("Hello, world!");
}

fn feast(beast: &str, dish: &str) -> bool {

    let first_beast_letter = beast.chars().nth(0).unwrap();
    let last_beast_letter = beast.chars().nth(beast.len() - 1).unwrap();

   if dish.starts_with(first_beast_letter) && dish.ends_with(last_beast_letter) {
       return true;
   }

    return false;
}


#[test]
fn sample_test_cases() {
    assert_eq!(feast("great blue heron", "garlic naan"), true);
    assert_eq!(feast("chickadee", "chocolate cake"), true);
    assert_eq!(feast("brown bear", "bear claw"), false);
}





