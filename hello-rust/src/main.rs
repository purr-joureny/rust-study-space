use ferris_says::say;
use std::io::{stdout, BufWriter};


fn do_something() {}

fn main() {
    let stdout = stdout();
    let message = String::from("hello fellow Rustaceans");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
    println!("Hello, world!");

    for i in 0..100 {
      do_something();
  }
}
