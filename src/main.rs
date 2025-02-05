fn main() {
    fizz_buzz(100);
}

fn fizz_buzz(n: i32) -> () {
    if n > 0 {
        for i in 1..=n {
            if i % 3 == 0 && i % 5 == 0 {
                println!("FizzBuzz")
            } else if i % 3 == 0 {
                println!("Fizz")
            } else if i % 5 == 0 {
                println!("Buzz")
            } else {
                println!("{}", i)
            }
        }
    }
}
