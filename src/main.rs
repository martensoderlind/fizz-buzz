fn main() {
    fizz_buzz(100);
}

fn fizz_buzz(n: i32) {
    if n > 0 {
        for i in 1..=n {
            match (i % 3, i % 5) {
                (0, 0) => println!("fizzbuzz"),
                (0, _) => println!("fizz"),
                (_, 0) => println!("buzz"),
                (_, _) => println!("{}", i),
            }
        }
    }
}
