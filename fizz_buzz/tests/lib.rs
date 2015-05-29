extern crate fizz_buzz;

#[test]
fn one_neither_fizz_neither_buzz() {
    assert_eq!("1", fizz_buzz::fizz_buzz(1));
}

#[test]
fn three_is_fizz() {
    assert_eq!("Fizz", fizz_buzz::fizz_buzz(3));
}

#[test]
fn five_is_buzz() {
    assert_eq!("Buzz", fizz_buzz::fizz_buzz(5));
}

#[test]
fn fiveteen_is_fizzbuzz() {
    assert_eq!("FizzBuzz", fizz_buzz::fizz_buzz(15));
}