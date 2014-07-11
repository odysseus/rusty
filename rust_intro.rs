fn main() {
  fn add_heads((a,_): (int,int), (b,_): (int,int)) -> int {
    a + b
  }

  let tupa: (int, int) = (3, 4);
  let tupb: (int, int) = (5, 6);

  println!("{}", add_heads(tupa, tupb));
}

