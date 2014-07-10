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
}

// Another example of capturing the value of an expression evaluation
fn is_four(x: int) -> bool {
  x == 4
}
