fn main() {
  for i in range(1i,26) {
    println!("{:s}",
      if div_three(i) && div_five(i) { "FizzBuzz".to_str() }
      else if div_three(i) { "Fizz".to_str() }
      else if div_five(i) { "Buzz".to_str() }
      else { i.to_str() }
      );
  }
}

fn div_three(num: int) -> bool {
  num % 3 == 0
}

fn div_five(num: int) -> bool {
  num % 5 == 0
}

#[test]
fn test_div_three_with_bad_value() {
  if div_three(5) {
    fail!("Failed! 5 is not divisble by 3");
  }
}

// Because the above pattern is so common, we can just use assert! Which
// does the same thing as if/fail
#[test]
fn test_div_three_with_good_value() {
  assert!(div_three(6))
}

#[test]
fn test_div_five_with_bad_value() {
  assert!(!div_five(13))
}

#[test]
fn test_div_five_with_good_value() {
  assert!(div_five(15))
}
