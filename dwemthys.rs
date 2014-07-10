// Dwemthy's Array

struct BasicMonster {
  health: int,
  attack: int
}

impl BasicMonster {
  fn new(health: int, attack: int) -> BasicMonster {
    BasicMonster { health:health, attack:attack }
  }

  fn attack(&self) {
    println!("The monster attacks for {:d} damage!", self.attack);
  }

  fn count() {
    println!("There are a bunch of monsters out tonight.");
  }
}

// If we want to define multiple, similar types we can use enums
enum Monster {
  ScubaArgentine(int, int, int, int),
  IndustrialRaverMonkey(int, int, int, int)
}

impl Monster {
  fn attack(&self) {
    match *self {
      ScubaArgentine(l, s, c, w) => println!("The monster attacks for {:d} damage!", w),
      IndustrialRaverMonkey(l, s, c, w) => println!("The monster attacks for {:d} damage!", w)
    }
  }
}

fn main() {
  let m = BasicMonster { health: 10, attack: 20 };

  m.attack();
  BasicMonster::count();
  BasicMonster::new(20,40).attack();

  let irm = IndustrialRaverMonkey(46, 35, 91, 2);
  irm.attack();
}
