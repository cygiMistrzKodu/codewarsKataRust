fn main() {
    println!("Hello, world!");
}

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::count_positives_sum_negatives;

    fn dotest(a: &[i32], expected: &[i32]) {
        let actual = count_positives_sum_negatives(a.to_vec());
        assert_eq!(actual, expected, "With input = {a:?}\nExpected {expected:?} but got {actual:?}")
    }
    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15], &[10, -65]);
        dotest(&[], &[]);
        dotest(&[0, 2, 3, 0, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14], &[8, -50]);
        dotest(&[0,1,2,3,4,5], &[5, 0]);
        dotest(&[1,2,3,4,5], &[5, 0]);
        dotest(&[0,-1,-2,-3,-4,-5], &[0, -15]);
        dotest(&[-1,-2,-3,-4,-5], &[0, -15]);
        dotest(&[0,0,0,0], &[0,0]);
        dotest(&[0], &[0,0]);
    }
}

