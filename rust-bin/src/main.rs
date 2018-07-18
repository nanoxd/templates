#[macro_use]
extern crate quicli;
use quicli::prelude::*;

#[derive(Debug, StructOpt)]
struct Cli {
  #[structopt(flatten)]
  verbosity: Verbosity,
}

main!(|args: Cli, log_level: verbosity| {
  let world = "world";
  println!("Hello {}", world);
});
