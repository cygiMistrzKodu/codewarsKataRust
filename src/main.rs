
fn main() {
    println!("Hello, world!");
}

fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    let free_space_in_bus = cap - on;
    let people_cant_take = wait - free_space_in_bus;

    return if people_cant_take > 0 {
        people_cant_take
    } else { 0 };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enough() {
        assert_eq!(enough(10, 5, 5), 0, "enough(10, 5, 5) should return 0");
        assert_eq!(enough(100, 60, 50), 10, "enough(100, 60, 50) should return 10");
        assert_eq!(enough(20, 5, 5), 0, "enough(20, 5, 5) should return 0");
    }
}





