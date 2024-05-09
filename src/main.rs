use regex::Regex;

fn main() {
    println!("Hello, world!");
}

fn abbrev_name(name: &str) -> String {

    let first_word_letters_regex = Regex::new(r"\b[a-zA-z]").unwrap();

    let mut initials = String::with_capacity(0);
    for capture_first_word_letter in first_word_letters_regex.captures_iter(name) {
        let letter = &capture_first_word_letter[0];
        initials.push_str(&*letter.to_uppercase());
        initials.push_str(".");
    }
    initials.pop();

    return initials;
}

#[test]
fn sample_tests() {
    assert_eq!(abbrev_name("Sam Harris"), "S.H");
    assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
    assert_eq!(abbrev_name("Evan Cole"), "E.C");
    assert_eq!(abbrev_name("P Favuzzi"), "P.F");
    assert_eq!(abbrev_name("David Mendieta"), "D.M");
    assert_eq!(abbrev_name("george clooney"), "G.C");
    assert_eq!(abbrev_name("Sam x"), "S.X");
}



