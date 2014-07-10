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

It is possible to declare and then initialize later, but this practice is used less in Rust than elsewhere because it can lead to uninitialized variables. The compiler will yell at you if you have a variable that never gets initialized, it will also yell at you if the value of a variable is never read and the variable is nor prefixed with a `_`. The compiler yells a lot.

### Inference
The inference system is pretty good. It doesn't just look at the value a variable was assigned, it also looks at how that variable is used afterwards to determine its type. An example of this using a vector:

    fn main() {
        // Using local inference, the compiler knows that `elem` has type u8
        let elem = 5u8;

        // Create an empty vector (a growable array)
        let mut vec = Vec::new();
        // At this point the compiler doesn't know the exact type of `vec`, it
        // just knows that it's a vector of something (`Vec<_>`)

        // Insert `elem` in the vector
        vec.push(elem);
        // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
        // TODO ^ Try commenting out the `vec.push(elem)` line

        println!("{}", vec);
    }

When hints are available it does a pretty good job of figuring the type itself, however if you remove these hints, or none exist, then it will yell at you asking for an explicit type because Rust does not tolerate uncertainty.

### Alias
Aliasing is done with the keyword `type` and gives a new name to an existing type.

    // `NanoSecond` is a new name for `u64`
    type NanoSecond = u64;
    type Inch = u64;

    // Use an attribute to silence warning
    #[allow(non_camel_case_types)]
    type uint64_t = u64;
    // TODO ^ Try removing the attribute

    fn main() {
        // `NanoSecond` = `Inch` = `uint64_t` = `u64`
        let nanoseconds: NanoSecond = 5 as uint64_t;
        let inches: Inch = 2 as uint64_t;

        // Note that type aliases *don't* provide any extra type safety, because
        // aliases are *not* new types
        println!("{} nanoseconds + {} inches = {} unit?",
                 nanoseconds,
                 inches,
                 nanoseconds + inches);
    }

By default all types should be in CamelCase and uppercase, as mentioned, but primitive types are an exception to this rule. Primarily this feature is used to reduce typing or provide clarity in the code.

### Expressions
Almost all statements are expressions, because this isn't always the desired behavior, you can suppress the output of an expression by ending the line with a semicolon. Blocks are also expressions whose l-value is set to the final expression evaluated inside it. If the final expression ends with a semicolon then the block receives the value `()`.

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
    let bn = 0b01101u   // Binary

The types above can be declared using the suffix notation shown, there are other available variable types that are declared directly

- Signed Integers: `i8 i16 i32 i64` and a machine sized `int`
- Unsigned Integers: `u8 u16 u32 u64` and machine sized `uint`
- Floating Points: `f32 f64`
- `char` Unicode 4-byte sequences
- `bool` true and false literals
- `()` the unit type

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

Variables are block scoped, Rust also allows variable shadowing, so variables with identical names can be declared in different scopes. In such an instance the variable that is the most local will be evaluated.

    fn main() {
      let shadow = 5i;
      println!("Shadow: {}", shadow);
      // 5
      {
        let shadow = 10i;
        println!("Shadow: {}", shadow);
        // 10
        {
          println!("Shadow: {}", shadow);
          // 10
        }
      }
      println!("Shadow: {}", shadow);
      // 5
    }

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

Precedence of operations is the same as most C-like languages. Math follows the order of operations and has highest precedence, followed by binary and comparison operators, followed last by assignment operators. The behavior is what you would expect so remembering this is unnecessary.

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
Don't need to be surrounded by a parentheses, they are evaluated using the normal operator precedence which often doesn't require any parens.

    let s = if true { 3 } else { 4 };

The condition must be of type bool, no implicit conversion happens, so there is no inherent "truthiness" of certain values:

    let t = if 4 { 4 } else { 3 }; <- Doesn't work

If the blocks have a value (no semicolon at the end, thus passing the value to the block), then every arm of the conditional must return the same type.

### Loop and Control Flow
`loop` enters an infinite loop. `break` breaks out of the loop entirely, `continue` skips the remainder of the loop block and starts from the next iteration.

Now for something completely better. Loops can be annotated and controlled separately when dealing with nested loops.

```clojure
    fn main() {
        'outer: loop {
            println!("Entered the outer loop");
            'inner: loop {
                println!("Entered the inner loop");
                // This would break only the inner loop
                //break;
                // This breaks the outer loop
                break 'outer;
            }
            println!("This point will never be reached");
        }
        println!("Exited the outer loop");
    }
```

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
