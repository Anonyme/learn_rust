/// This function returns "Fizz" if num is a multiple of three,
/// "Buzz" if num is a multiple of five and "FizzBuzz" if num
/// is a multiple of three and five.
///
/// # Examples
///
/// ```
/// use fizz_buzz::fizz_buzz;
///
/// assert_eq!("FizzBuzz", fizz_buzz(15));
/// ```
pub fn fizz_buzz(num: i32) -> String {
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