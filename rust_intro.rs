fn main() {
  println!("Hello, Rust!");

  // Variables declared using let
  let a = 10;
  // Most types can be inferred, but if you need a sepcific type:
  let b: i64 = 2000000000000;
  // All variables are immutable by default, to allow mutation:
  let mut c = 30;
  // Any unused variables get a warning by default, to suppress that:
  let mut _d = 40;
  // variables and functions are in snake_case
  let my_favorite_number = 7i;
  // Whereas types are in CamelCase
  type MyInt = int;

  // All statements are expressions, this is what the semicolons do
  // they denote the end of an expression
  let item = "ice cream";
  let price: f64 =
    if item == "salad" {
      3.50
    } else if item == "muffin" {
      2.50
    } else if item == "ice cream" {
      101.20
    } else {
      1.00
    };
  // The lack of a semicolon at the end of the line gives the entire block the
  // value of that expression, so the lack of semicolons at the end of these
  // mean that, once evaluated, the block will return the value of the
  // conditional that evaluated to true

  // Various Variable Types
  let e: int = 1;   // int
  let f = 10i;      // int
  let g = 100u;     // unsigned int
  let h = 15i8;     // 8 bit int
  let i = 1000i32;  // 32 bit int
  let j = 1000i64;  // 64 bit int
  let k = 1e6f32;    // 32 bit float
  let l = 2.1e-10f64;    // 64 bit float

  // true and false are literals of type bool
  let tr = true;
  let fa = false;

  // Strings are complicated, but here are some basics:
  // Chars are four byte Unicode characters with single quotes
  let m = 'a';
  // Double-quoted strings will recognize some escape sequences:
  let n = "Hello, world!\n";
  // Raw string literals process none of these and use #hashes#
  // at the beginning and end to define them
  let o = r##"Hello, world!\n"##;

  // The unit type, written as () has a single value of ()
  let p = ();

  // Operators are all pretty standard:
  // + - * / for arithmetic
  // - works to negate numbers as well
  // ! applied to an integer flips all the bits (bitwise NOT)
  // Bitwise operators: << >> & | ^
  // Comparison operators: == != < > <= >=
  // Boolean operators: && || and they short circuit

  // Compile time casting
  println!("{:0.2f}", (f as f32));
  let q: f64 = 4.0;
  let r: uint = q as uint;

  // Syntax Extensions
  // Are special forms provided by the libraries rather than
  // being built into the language, all language extensions have names
  // that end with !


  // Conditionals
  let s =
    if true { 3 } else { 4 };
  // The condition must be of type bool, no implicit conversion happens
  // let t = if 4 { 4 } else { 3 }; <- Doesn't work
  // If the blocks have a value, then every arm of the conditional must
  // return the same type

  // Pattern Matching
  // Is a generalized and more powerful version of the switch statement
  // the first pattern that matches executes, there is no need to break
  // _ is a wildcard that acts as an else statement, it matches anything
  // and must always be put last because any pattern beneath it will
  // never be reached. Furthermore, _ must be included to ensure that
  // something gets matched
  let t = 12i;
  match t {
    0       => println!("zero!"),
    1 | 2   => println!("one or two"),
    3..10   => println!("three to ten"),
    10..20  => println!("ten to twenty"),
    _       => println!("something else entirely")
  }
  // Each pattern must be followed by a

}

// Another example of capturing the value of an expression evaluation
fn is_four(x: int) -> bool {
  x == 4
}

// Conditional evaluation where the blocks have values
fn signum(x: int) -> int {
  if x < 0 { -1 }
  else if x > 0 { 1 }
  else { 0 }
}
