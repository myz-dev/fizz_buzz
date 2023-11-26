/// Command line application to run an extended version of the well known
/// FizzBuzz game.
#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// How many iterations of the FizzBuzz game to play .
    #[arg(short)]
    pub t: u32,
    /// Multiples of `f` are going to print out `Fizz`.
    #[arg(short)]
    pub f: u32,
    /// Multiples of `b` are going to print out `Buzz`.
    #[arg(short)]
    pub b: u32,
}
