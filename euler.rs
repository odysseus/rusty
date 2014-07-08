use std::io::println;

fn main() {
  println("Hello, world!");
  println!("Project Euler Problem {}: {}", "1", eu1());
  println!("Project Euler Problem {}: {}", "1", euOne());
  println!("Project Euler Problem {}: {}", "2", eu2());
  println!("Project Euler Problem {}: {}", "2", euTwo());
  println!("Project Euler Problem {}: {}", "3", eu3());
}

// Find the sum of natural numbers below 1,000 divisble by 3 or 5
fn eu1() -> int {
  let mut total = 0;
  for n in range (1i, 1000) {
    if n % 3 == 0 || n % 5 == 0 {
      total += n;
    }
  }
  return total;
}

fn euOne() -> int {
  fn oneTail(cur:int, tot:int) -> int {
    if cur >= 1000 {
      return tot;
    } else {
      if cur % 3 == 0 || cur % 5 == 0 {
        return oneTail((cur+1), (tot+cur));
      } else {
        return oneTail((cur+1),tot);
      }
    }
  }
  return oneTail(1,0);
}

// Find the sum of even valued Fibonacci numbers below 4,000,000
fn eu2() -> int {
  let mut a = 0i;
  let mut b = 1i;
  let mut total = 0i;
  while b < 4000000 {
    if b % 2 == 0 {
      total += b;
    }
    b = a + b;
    a = b - a;
  }
  return total;
}

// Tail recursive solution for Euler 2
fn euTwo() -> int {
  fn twoTail(a: int, b: int, total: int) -> int {
    if b > 4000000 {
      return total;
    } else {
      if b % 2 == 0 {
        return twoTail(b, (a+b), (total + b));
      } else {
        return twoTail(b, (a+b), total);
      }
    }
  }
  return twoTail(0, 1, 0);
}

fn eu3() -> int {
  let mut n = 600851475143;
  let r = (n as f64).sqrt() as int;
  for i in range(2i,r) {
    if i >= n { return n; }
    if n % i == 0 {
      while n % i == 0 {
        n /= i;
      }
    }
  }
  return n;
}
