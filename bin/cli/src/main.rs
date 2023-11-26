use args::Config;
use clap::Parser;
use fizz_buzz::error::Result;

use crate::traditional::play_traditional;

mod args;
mod traditional;

fn main() -> Result<()> {
    let args = Config::parse();

    println!("Running FizzBuzz with following configuration:\n{args:#?}\n\n");

    let output = play_traditional(args)?;
    println!("{output}");
    Ok(())
}
