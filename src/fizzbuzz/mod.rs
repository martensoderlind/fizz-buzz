pub fn fizzbuzz(n: i32) -> String {
    let mut result = String::new();
    if n > 0 {
        for i in 1..=n {
            match (i % 3, i % 5) {
                (0, 0) => result.push_str("FizzBuzz"),
                (0, _) => result.push_str("Fizz"),
                (_, 0) => result.push_str("Buzz"),
                (_, _) => result.push_str(&i.to_string()),
            }
            result.push_str("\n");
        }
    }
    return result;
}
