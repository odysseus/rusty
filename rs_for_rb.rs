use std::io::println;

fn main() {
  println("Hello, world!");
  for _ in range(0u, 10) {
    spawn(proc() {
      let greeting_message = "Hello?";
      println!("{}", greeting_message);
    });
  }
}

