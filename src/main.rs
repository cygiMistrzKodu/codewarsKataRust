fn main() {
    println!("Hello, world!");
}

fn set_alarm(employed: bool, vacation: bool) -> bool {
    return if employed == true && vacation == false {
        true
    } else {
        false
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_alarm() {
        assert_eq!(set_alarm(true, true), false, "Fails when input is true, true");
        assert_eq!(set_alarm(false, true), false, "Fails when input is false, true");
        assert_eq!(set_alarm(false, false), false, "Fails when input is false, false");
        assert_eq!(set_alarm(true, false), true, "Fails when input is true, false");
    }
}

