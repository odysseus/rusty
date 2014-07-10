use std::io::println;
use std::comm::{channel, Sender, Receiver};

fn main() {
  // Hello, world!
  println("Hello, world!");

  // Spawning concurrent processes to print "Hello"
  for _ in range(0u, 10) {
    spawn(proc() {
      println!("Hello");
    });
  }

  // Opening a channel and port to communicate with a spawned proceess
  let (chan, port) = channel();
  spawn(proc() {
    chan.send(10u);
  });
  println!("{:s}", port.recv().to_str());

  // Creating channels to send a value, add one to it, then send it back
  let (fromParentSender, fromParentReceiver) = channel();
  let (fromChildSender, fromChildReceiver) = channel();
  spawn(proc() {
    plus_one(&fromChildSender, &fromParentReceiver);
  });
  for i in range(20i,25) {
    fromParentSender.send(i);
  }
  fromParentSender.send(0);
  for _ in range(0u,5) {
    let answer = fromChildReceiver.recv();
    println!("{:s}", answer.to_str());
  }

  let x = box 10i;
  println!("{:d}", inc(x));

}

fn plus_one(sender: &Sender<int>, receiver: &Receiver<int>) {
  let mut value: int;
  loop {
    value = receiver.recv();
    sender.send(value + 1);
    if value == 0 { break; }
  }
}

// Example of using a borrowed pointer
fn inc(x: &int) -> int {
  *x + 1
}
