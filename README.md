# Rust
### Variables

Are declared using let

```rust
    let a = 10;
```

Most types can be inferred, but if you need a specific type:

```rust
    let b: i64 = 2000000000000;
```

All variables are immutable by default, to allow mutation:

```rust
    let mut c = 30;
```

Any unused variables get a warning by default, to suppress that:

```rust
    let mut _d = 40;
```

variables and functions are in `snake_case`

```rust
    let my_favorite_number = 7i;
```

Whereas types are in CamelCase

```rust
    type MyInt = int;
```

It is possible to declare and then initialize later, but this practice is used less in Rust than elsewhere because it can lead to uninitialized variables. The compiler will yell at you if you have a variable that never gets initialized, it will also yell at you if the value of a variable is never read and the variable is nor prefixed with a `_`. The compiler yells a lot.

### Inference
The inference system is pretty good. It doesn't just look at the value a variable was assigned, it also looks at how that variable is used afterwards to determine its type. An example of this using a vector:

```rust
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
```

When hints are available it does a pretty good job of figuring the type itself, however if you remove these hints, or none exist, then it will yell at you asking for an explicit type because Rust does not tolerate uncertainty.

### Alias
Aliasing is done with the keyword `type` and gives a new name to an existing type.

```rust
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
```

By default all types should be in CamelCase and uppercase, as mentioned, but primitive types are an exception to this rule. Primarily this feature is used to reduce typing or provide clarity in the code.

### Expressions
Almost all statements are expressions, because this isn't always the desired behavior, you can suppress the output of an expression by ending the line with a semicolon. Blocks are also expressions whose l-value is set to the final expression evaluated inside it. If the final expression ends with a semicolon then the block receives the value `()`.

```rust
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
```

The lack of a semicolon at the end of the line gives the entire block the value of that expression, so the lack of semicolons at the end of these mean that, once evaluated, the block will return the value of the conditional that evaluated to true

### Various Variable Types

```rust
    let e: int = 1;     // int
    let f = 10i;        // int
    let g = 100u;       // unsigned int
    let h = 15i8;       // 8 bit int
    let i = 1000i32;    // 32 bit int
    let j = 1000i64;    // 64 bit int
    let k = 1e6f32;     // 32 bit float
    let l = 2.1e-10f64; // 64 bit float
    let bn = 0b01101u   // Binary
```

The types above can be declared using the suffix notation shown, there are other available variable types that are declared directly

- Signed Integers: `i8 i16 i32 i64` and a machine sized `int`
- Unsigned Integers: `u8 u16 u32 u64` and machine sized `uint`
- Floating Points: `f32 f64`
- `char` Unicode 4-byte sequences
- `bool` true and false literals
- `()` the unit type

Numbers can be underscored for readability, also known as the world's greatest feature that only two languages seem to use.

```rust
    let bignum: i64 = 6_000_000_000_000
```

true and false are literals of type bool

```rust
    let tr = true;
    let fa = false;
```

Strings are complicated, but here are some basics: Chars are four byte Unicode characters with single quotes

```rust
    let m = 'a';
```

Double-quoted strings will recognize some escape sequences:

```rust
    let n = "Hello, world!\n";
```

Raw string literals process none of these and use #hashes# at the beginning and end to define them

```rust
    let o = r##"Hello, world!\n"##;
```

The unit type, written as () has a single value of ()

```rust
    let p = ();
```

### Blocks
Are denoted by curly braces and are the standard unit of code execution. Unlike in other C-like languages the curly braces are never optional, but it seems idiomatically fine to have them on the same line, so there is no need for the Javalike extension of simple statements across 5-6 lines:

```rust
    let s = if true { 3 } else { 4 };
```

Variables are block scoped, Rust also allows variable shadowing, so variables with identical names can be declared in different scopes. In such an instance the variable that is the most local will be evaluated.

```rust
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
```

### Expressions
This is probably wrong on some fronts, but basically everything is an expression, or can be one. The semicolon denotes the end of an expression. Understanding this helps to understand when to use semicolons and when not to use them. Within a block, if you don't terminate the expression with a semicolon then the value of the expression gets passed to the block.

```rust
    fn is_four(x: int) -> bool {
      x == 4
    }

    fn signum(x: int) -> int {
      if x < 0 { -1 }
      else if x > 0 { 1 }
      else { 0 }
    }
```

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

```rust
    println!("{:0.2f}", (f as f32));
    let q: f64 = 4.0;
    let r: uint = q as uint;
```

### Syntax Extensions
Are special forms provided by the libraries rather than
being built into the language, all language extensions have names
that end with !

### Conditionals
Don't need to be surrounded by a parentheses, they are evaluated using the normal operator precedence which often doesn't require any parens.

```rust
    let s = if true { 3 } else { 4 };
```

The condition must be of type bool, no implicit conversion happens, so there is no inherent "truthiness" of certain values:

```rust
    let t = if 4 { 4 } else { 3 }; <- Doesn't work
```

If the blocks have a value (no semicolon at the end, thus passing the value to the block), then every arm of the conditional must return the same type.

### Control Flow
`loop` enters an infinite loop. `break` breaks out of the loop entirely, `continue` skips the remainder of the loop block and starts from the next iteration.

Now for something completely better. Loops can be annotated and controlled separately when dealing with nested loops.

```rust
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

```rust
    let t = 12i;
    match t {
      0       => println!("zero!"),
              1 | 2   => println!("one or two"),
              3..10   => println!("three to ten"),
              10..20  => println!("ten to twenty"),
              _       => println!("something else entirely")
    }
```

Each pattern is followed by a rocket `=>` and an expression to be executed if the pattern matches, followed by a comma. It's also legal, easier to remember, and possibly more idiomatic, to follow the pattern with a block, in which case the comma can be excluded.

```rust
    let t = 12i;
    match t {
      0       => { println!("zero!") }
              1 | 2   => { println!("one or two") }
              3..10   => { println!("three to ten") }
              10..20  => { println!("ten to twenty") }
              _       => { println!("something else entirely") }
    }
```

Of course, the statements within the block can't be terminated by semicolons or they won't pass their values to the block properly.

Match constructs must be exhaustive, meaning every possible pattern must be matched. This requires the inclusion of the wildcard statement at the end of the match. The wildcard must always come last, because it matches everything, nothing below it would ever get evaluated, you will get a warning for this if you write code like that.

### Destructuring
Coming all the way from the land of functional programming accompanied by pattern matching is destructuring.

```rust
    use std::f64;
    fn angle(vector: (f64, f64)) -> f64 {
        let pi = f64::consts::PI;
        match vector {
          (0.0, y) if y < 0.0 => 1.5 * pi,
          (0.0, _) => 0.5 * pi,
          (x, y) => (y / x).atan()
        }
    }
```

Destructuring allows you to bind a match variable to some component of a data structure. In the case above the first pattern matches on a tuple whose first element is zero and binds the second value to `y` to test if `y` is less than `0`. The second pattern matches any other pattern with `0.0` in the `x` spot of the tuple. The `_` in this context means it assigns no variable to that match because it is not needed, which basically functions the same as a wildcard anyway, because the execution doesn't depend on the value of that particular component.

Subpatterns can be matched using the `@` pattern.

```rust
    match age {
      a @ 0..20 => { println!("{} years old", a) }
      _ => { println!("Old enough to drink") }
```
Any arm of the match statement can have a "guard clause" that takes a boolean test and only executes if the pattern also passes the boolean test

```rust
    match point {
      (1.0, y) if y > 2.0 => { // Expr }
      ...
    }
```

### Loops
`loop` creates an infinite loop and is the idiomatic way of writing `while true`.

```rust
    loop {
      stuff
    }
```

`while` continues to execute while the conditional returns true.

```rust
    while items_queue > 0 {
      handle
    }
```

`for` can be used to iterate over a range of numbers:

```rust
    for n in range(0..10u) {
      println!("{}", n);
    }
```

`for` also works for anything that implements `Iterator`, and can be used for list comprehensions.

```rust
    let s = "Hello, world!";
    for c in s.chars() {
      println!("{}", c);
    }
```

### Structs
Struct types must be declared before use.

```rust
    struct Point {
        x: f64,
        y: f64
    }
```

Construction uses the same basic syntax, but you simply exclude the `struct` command.

```rust
    let mut point = Point { x: 1.0, y: 1.0 };
```
Structs inherit their mutability, so if you declare a struct var to be mutable when declared then all of its fields will be mutable as well, and vice versa:

```rust
    let mut mutPoint = Point { x: 1.0, y: 2.0 };
    let origin = Point { x: 0.0, y: 0.0 };

    mutPoint.y += 1.0; // Totally works
    origin.x += 10.0; // Totally doesn't
```

Structs can be destructured in `match` statements as well:

```rust
    match point {
      Point { x: 0.0, y: yy } => { handle }
      Point { x: xx, y: 1.0 } => { handle }
      Point { x: 2.0, .. } => { handle }
    }
```

Generally speaking the names of the struct are what's important and not the order they appear in, so you can typically call a field by name and not worry about the order of them. If there are fields you aren't interested in when matching a struct you can use `..` as a wildcard that simply accepts and ignores the values of all other attributes.

Finally, in "Forget what we just told you" news: Matching and grabbing the variables of a struct is such a common pattern that you can simply shorthand it by including the variable name and it will bind to that variable name in the match.

```rust
    match point {
      Point { x, .. } => { println!("{}", x) };
    }
```

### Enums
Are datatypes with a fixed number of representations:

```rust
    enum CardinalDirections {
      North,
      East,
      South,
      West
    }
```

In a simple enum the values of these constants are masks for integer values. In this case `North` is `0`, `East` is `1` meaning `West` would be `2`.  You can also assign values to the enum items directly:

```rust
    enum Color {
      Red = 0xff0000,
      Green = 0x00ff00,
      Blue = 0x0000ff
    }
```

Or assign them more complex values:

```rust
    enum Shape {
      Circle(Point, f64),
      Rectangle(Point, Point)
    }
```

To create a new `Circle` instance:

```rust
    let c = Circle(Point { x: 0.0, y: 0.0 }, 10.0);
```

Like most other items, `enum`s can be destructured:

```rust
    use std::f64;
    fn area(sh: Shape) -> f64 {
        match sh {
            Circle(_, size) => f64::consts::PI * size * size,
            Rectangle(Point { x, y }, Point { x: x2, y: y2 }) => (x2 - x) * (y2 - y)
        }
    }
```

To match based solely on the type you could use `Circle(..)` to ignore all the fields and match solely on being a circle, but for convenience simply calling the name of the type will do the same thing, so in this case using `Circle` as the match is equivalent.

### Tuples
Tuples are like strucuts, except that their fields are not named and cannot be accessed with dot notation.

```rust
    let mytup: (int, int, f64) = (1, 2, 3.14);
    match myup {
      (a, b, c) => println!("{}, a + b + (c as int))
    }
```

They can be accessed via pattern-matching/destructuring:

```rust
    let t: (int, int) = (2,3);
    let (_,v) = t;
    println!("{}", v);
    // 3
```

#### Tuple Structs
Tuple structs behave like both structs and tuples, in that they have names and a specific type, like a struct, but their fields still do not have names. So `Chunky(1,2)` is different from `Bacon(1,2)`

```rust
    struct MyTup(int, int, f64);
    let mytup: MyTup = MyTup(10, 20, 30.0);
    match mytup {
      MyTup(a, b, c) => println!("{}", a + b + (c as int))
    }
```

As always the values can be extracted by pattern matching.

#### Newtypes
There's also a special tuplestructs called "newtypes" which define a tuple containing a single element, as so:

```rust
    struct UserId(int);
```

The reason for such a syntax is to create a type that is distinct from the primitive type it contains and not simply a synonym for it.

```rust
    struct Inches(int);
    struct Centimeters(int);
```

This pattern is used when a type _could_ be represented by a single value, but gives it the benefits of type checking and behavioral encapsulation, which would not be achieved by simply aliasing another type. The values of these can be extracted by pattern matching.

```rust
    let length_with_unit = Inches(10);
    let Inches(integer_length) = length_with_unit;
    println!("length is {} inches", integer_length);
```

### Functions
Functions, like other static declarations, can be declared at both the top level and inside of other functions. Functional programmers rejoice. The basic form of a function definition:

```rust
    fn adder(a: int, b: int) -> int {
      return a + b;
    }
```

A few things here. The name is declared using `fn name()`, the arguments go inside the parens after the name as `name: type` pairs separated by commas. The return value is stated using an arrow and the type returned. This style is similar to Haskell, which makes it a bit more visually apparently what is happening with the function.

`return` is used to immediately return the given value from the function, but by excluding semicolons you can pass the value of an expression to the block of the function itself, and it will implicitly return the value of the final expression. So we could (and should) write the function above as:

```rust
    fn adder(a: int, b: int) -> int {
      a + b
    }
```

Note the lack of return statement and semicolon.  Explicit `return` should only be used when returning early from a function, when returning something other than the last expression evaluated, or if it's visually difficult to parse what the actual return value of the function is.

Void functions are still possible, simply declare them without a return and suppress the implicit return of values using semicolons. Technically void functions still return the unit type `()`

Function definitions support destructuring as well:

```rust
    fn main() {
      fn add_heads((a,_): (int,int), (b,_): (int,int)) -> int {
        a + b
      }

      let tupa: (int, int) = (3, 4);
      let tupb: (int, int) = (5, 6);

      println!("{}", add_heads(tupa, tupb));
      // 8
    }
```

### Destructors
Destructors are methods that destroy. Completely, recursively, without remorse. Standard heap allocation is done using `box`, and the memory is freed as soon as the boxed item goes out of scope.

```rust
    {
        // an integer allocated on the heap
        let y = box 10i;
    }
    // the destructor frees the heap memory as soon as `y` goes out of scope
```

You can write your own destructors, but the manual goes strangely silent on that topic at this point. So...

### Ownership
An objects owner is responsible for managing the lifetime and mutability of an object. It calls the destructor when the object is no longer needed. Variables are top level owners and will destroy the associated values when they go out of scope.

Ownership is recursive, so a mutable object containing other objects will make these objects mutable. Ownership of a object containing other objects entails ownership of the entire tree.  Fortunately, destructors are also recursive so calling a destructor on an object that contains other objects will also destroy the other objects.

### Unused Items
The compiler will complain about any variables that never get read, or any functions that never get used. To suppress these warnings for specific items you can use an underscore for variables or the `#[allow(dead_code)]` attribute for functions.

```rust
    fn main() {
      let _x = 10i;

      #[allow(dead_code)]
      fn unused_fn() {}
    }
```

### Modules and Visibility
Modules can contain functions, structs, traits, impl blocks, or other modules. Calling items from within a module (or nested modules) involves using their full path:

    std::io::stdio::println("Hello, world!");

The `println` function lives in the `stdio` module, in the `io` module, in the `std` create.

An example of using modules:

```rust
    fn function() {
        println!("called `function()`");
    }

    // A module named `my`
    mod my {
        // A module can contain items like functions
        #[allow(dead_code)]
        fn function() {
            println!("called `my::function()`");
        }

        // Modules can be nested
        mod nested {
            #[allow(dead_code)]
            fn function() {
                println!("called `my::nested::function()`");
            }
        }
    }
```

However, actually attempting to call most of these would generate an error, because functions and modules are private by default, to change that you need to add `pub` at the beginning of their definitions.

```rust
    fn function() {
        println!("called `function()`");
    }

    mod my {
        // A public function
        pub fn function() {
            println!("called `my::function()`");
        }

        // A private function
        fn private_function() {
            println!("called `my::private_function()`");
        }

        // Items can access other items in the same module
        pub fn indirect_access() {
            print!("called `my::indirect_access()`, that\n> ");

            // regardless of their visibility
            private_function();
        }

        // A public module
        pub mod nested {
            pub fn function() {
                println!("called `my::nested::function()`");
            }

            #[allow(dead_code)]
            fn private_function() {
                println!("called `my::nested::private_function()`");
            }
        }

        // A private module
        mod inaccessible {
            #[allow(dead_code)]
            pub fn public_function() {
                println!("called `my::inaccessible::public_function()`");
            }
        }
    }
```

### Use
`use` can be used to bind a long method address to a new name:

```rust
    fn main() {
      use puts = std::io::stdio::println;
      puts("Hello, world!");
    }
```

### Self and Super
`self` and `super` are used to remove ambiguity when calling functions. `self` refers to the current module scope and `super` refers to the parent scope. Here's a lengthy example of all that fun stuff.

```rust
    fn function() {
        println!("called `function()`");
    }

    mod my {
        pub fn indirect_call() {
            // Let's access all the functions named `function` from this scope
            print!("called `my::indirect_call()`, that\n> ");

            // `my::function` can be called directly
            function();

            {
                // This will bind to the `cool::function` in the *crate* scope
                // In this case the crate scope is the outermost scope
                use root_cool_function = cool::function;

                print!("> ");
                root_cool_function();
            }

            {
                // `self` refers to the current module scope, in this case: `my`
                use my_cool_function = self::cool::function;

                print!("> ")
                my_cool_function();
            }

            {
                // `super` refers to the parent scope, i.e. outside of the `my`
                // module
                use root_function = super::function;

                print!("> ");
                root_function();
            }
        }

        fn function() {
            println!("called `my::function()`");
        }

        mod cool {
            pub fn function() {
                println!("called `my::cool::function()`");
            }
        }
    }

    mod cool {
        pub fn function() {
            println!("called `cool::function()`");
        }
    }

    fn main() {
        my::indirect_call();
    }
```

### Using Modules in Other Files
By convention, Rust will look for a module file whose name is identical to the name of the module. Because of this you can simply include the name of the module at the top of a code file and it will look for the file containing it. An example of a project structure:

```rust
$ tree .
.
|-- my
|   |-- inaccessible.rs
|   |-- mod.rs
|   `-- nested.rs
`-- split.rs
```

Now the `split.rs` file in that project:

```rust
    // split.rs
    // This declaration will look for a file named `my.rs` or `my/mod.rs` and will
    // insert its contents inside a module named `my` under this scope
    mod my;

    fn function() {
        println!("called `function()`");
    }

    fn main() {
        my::function();

        function();

        my::indirect_access();

        my::nested::function();
    }
```

The `my/mod.rs` file:

```rust
    // my/mod.rs
    // Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
    // and `inaccessible.rs` files and insert them here under their respective
    // modules
    mod inaccessible;
    pub mod nested;

    pub fn function() {
        println!("called `my::function()`");
    }

    fn private_function() {
        println!("called `my::private_function()`");
    }

    pub fn indirect_access() {
        print!("called `my::indirect_access()`, that\n> ");

        private_function();
    }
```

`my/nested.rs`

```rust
    // my/nested.rs
    pub fn function() {
        println!("called `my::nested::function()`");
    }

    #[allow(dead_code)]
    fn private_function() {
        println!("called `my::nested::private_function()`");
    }
```

`my/inaccessible.rs`

```rust
    // my/inaccessible.rs
    #[allow(dead_code)]
    pub fn public_function() {
        println!("called `my::inaccessible::public_function()`");
    }
```

And finally the output of running `split.rs`

```rust
    $ rustc split.rs && ./split
    called `my::function()`
    called `function()`
    called `my::indirect_access()`, that
    > called `my::private_function()`
    called `my::nested::function()`
```

### Crates
