fn main() {
  let shadow = 5i;
  let mut blocky = 10u16;
  println!("Shadow: {}", shadow);
  println!("Blocky: {}", blocky);
  {
    let shadow = 10i;
    blocky = 12;
    println!("Shadow: {}", shadow);
    {
      println!("Shadow: {}", shadow);
    }
  }
  println!("Shadow: {}", shadow);
  println!("Blocky: {}", blocky);
}

