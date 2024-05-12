fn main() {
    println!("Hello, world!");
}

fn find_short(s: &str) -> u32 {

    let words: Vec<&str> = s.split(" ").into_iter().collect();
    let mut shortest_word = words.get(0).unwrap_or(&"").chars().count() as u32;

    for word in words {
        let counted_word = word.chars().count() as u32;

        if counted_word < shortest_word {
            shortest_word = counted_word;
        }
    }

    return shortest_word;
}


#[cfg(test)]
mod tests {
    use super::find_short;

    fn dotest(s: &str, expected: u32) {
        let actual = find_short(s);
        assert_eq!(actual, expected, "With s = \"{s}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn sample_tests() {
        dotest("bitcoin take over the world maybe who knows perhaps", 3);
        dotest("turns out random test cases are easier than writing out basic ones", 3);
        dotest("lets talk about javascript the best language", 3);
        dotest("i want to travel the world writing code one day", 1);
        dotest("Lets all go on holiday somewhere very cold", 2);
        dotest("Let's travel abroad shall we", 2);
    }
}





