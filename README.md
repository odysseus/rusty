# Rust
### Variables

Are declared using let

    let a = 10;

Most types can be inferred, but if you need a specific type:

    let b: i64 = 2000000000000;

All variables are immutable by default, to allow mutation:

    let mut c = 30;

Any unused variables get a warning by default, to suppress that:

    let mut _d = 40;

variables and functions are in `snake_case`

    let my_favorite_number = 7i;

Whereas types are in CamelCase

    type MyInt = int;

All statements are expressions, this is what the semicolons do they denote the end of an expression

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

The lack of a semicolon at the end of the line gives the entire block the value of that expression, so the lack of semicolons at the end of these mean that, once evaluated, the block will return the value of the conditional that evaluated to true

### Various Variable Types

    let e: int = 1;     // int
    let f = 10i;        // int
    let g = 100u;       // unsigned int
    let h = 15i8;       // 8 bit int
    let i = 1000i32;    // 32 bit int
    let j = 1000i64;    // 64 bit int
    let k = 1e6f32;     // 32 bit float
    let l = 2.1e-10f64; // 64 bit float

Numbers can be underscored for readability, also known as the world's greatest feature that only two languages seem to use.

    let bignum: i64 = 6_000_000_000_000

true and false are literals of type bool

    let tr = true;
    let fa = false;

Strings are complicated, but here are some basics: Chars are four byte Unicode characters with single quotes

    let m = 'a';

Double-quoted strings will recognize some escape sequences:

    let n = "Hello, world!\n";

Raw string literals process none of these and use #hashes# at the beginning and end to define them

    let o = r##"Hello, world!\n"##;

The unit type, written as () has a single value of ()

    let p = ();

### Blocks
Are denoted by curly braces and are the standard unit of code execution. Unlike in other C-like languages the curly braces are never optional, but it seems idiomatically fine to have them on the same line, so there is no need for the Javalike extension of simple statements across 5-6 lines:

    let s = if true { 3 } else { 4 };

### Expressions
This is probably wrong on some fronts, but basically everything is an expression, or can be one. The semicolon denotes the end of an expression. Understanding this helps to understand when to use semicolons and when not to use them. Within a block, if you don't terminate the expression with a semicolon then the value of the expression gets passed to the block.

    fn is_four(x: int) -> bool {
      x == 4
    }

    fn signum(x: int) -> int {
      if x < 0 { -1 }
      else if x > 0 { 1 }
      else { 0 }
    }

As seen above, none of the expressions within the block use a semicolon at the end and thus the values, in this case a bool and some primitives, are inherited by the block and implicitly returned as the value of the function.

### Operators
Operators are all pretty standard:
- Arithmetic: `+ - * /`
- `!` applied to an integer flips all the bits
- Bitwise: `<< >> & | ^`
- Comparison: `== != < > <= >=`
- Boolean: `&& ||` and they short circuit

Precedence of operations is the same as most C-like languages, also math and basic logic.

### Compile time casting
Using the `as` command to cast between different primitive values

    println!("{:0.2f}", (f as f32));
    let q: f64 = 4.0;
    let r: uint = q as uint;

### Syntax Extensions
Are special forms provided by the libraries rather than
being built into the language, all language extensions have names
that end with !

### Conditionals

    let s = if true { 3 } else { 4 };

The condition must be of type bool, no implicit conversion happens, so there is no inherent "truthiness" of certain values:

    let t = if 4 { 4 } else { 3 }; <- Doesn't work

If the blocks have a value (no semicolon at the end, thus passing the value to the block), then every arm of the conditional must return the same type.

### Pattern Matching
Is a generalized and more powerful version of the switch statement the first pattern that matches executes, there is no need to break. Pipes `|` can be used to match multiple conditions, so long as they match the same variables when using destructuring (See below). With numbers a range can be specified using `a..b`:

    let t = 12i;
    match t {
      0       => println!("zero!"),
              1 | 2   => println!("one or two"),
              3..10   => println!("three to ten"),
              10..20  => println!("ten to twenty"),
              _       => println!("something else entirely")
    }

Each pattern is followed by a rocket `=>` and an expression to be executed if the pattern matches, followed by a comma. It's also legal, easier to remember, and possibly more idiomatic, to follow the pattern with a block, in which case the comma can be excluded.

    let t = 12i;
    match t {
      0       => { println!("zero!") }
              1 | 2   => { println!("one or two") }
              3..10   => { println!("three to ten") }
              10..20  => { println!("ten to twenty") }
              _       => { println!("something else entirely") }
    }

Of course, the statements within the block can't be terminated by semicolons or they won't pass their values to the block properly.

Match constructs must be exhaustive, meaning every possible pattern must be matched. This requires the inclusion of the wildcard statement at the end of the match. The wildcard must always come last, because it matches everything, nothing below it would ever get evaluated, you will get a warning for this if you write code like that.

### Destructuring
Coming all the way from the land of functional programming accompanied by pattern matching is destructuring.

    use std::f64;
    fn angle(vector: (f64, f64)) -> f64 {
        let pi = f64::consts::PI;
        match vector {
          (0.0, y) if y < 0.0 => 1.5 * pi,
          (0.0, _) => 0.5 * pi,
          (x, y) => (y / x).atan()
        }
    }

Destructuring allows you to bind a match variable to some component of a data structure. In the case above the first pattern matches on a tuple whose first element is zero and binds the second value to `y` to test if `y` is less than `0`. The second pattern matches any other pattern with `0.0` in the `x` spot of the tuple. The `_` in this context means it assigns no variable to that match because it is not needed, which basically functions the same as a wildcard anyway, because the execution doesn't depend on the value of that particular component.
