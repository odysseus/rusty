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

}
