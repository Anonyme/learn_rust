extern crate fizz_buzz;

#[test]
fn one_neither_fizz_neither_buzz() {
    assert_eq!("1", fizz_buzz::fizz_buzz(1));
}