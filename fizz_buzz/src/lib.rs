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
    match num {
        n if n % 3 == 0 && n % 5 == 0 => "FizzBuzz".to_string(),
        n if n % 3 == 0 => "Fizz".to_string(),
        n if n % 5 == 0 => "Buzz".to_string(),
        _ => num.to_string()
    }
}