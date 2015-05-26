fn main() {
    for num in 1..101 {
        println!("{}", fizz_buzz(num));
    }
}

fn fizz_buzz(num: i32) -> String {
    if num % 3 == 0 && num % 5 == 0 {
        "FizzBuzz".to_string()
    } else if num % 3 == 0 {
        "Fizz".to_string()
    } else if num % 5 == 0 {
        "Buzz".to_string()
    } else {
        num.to_string()
    }
}