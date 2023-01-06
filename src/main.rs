use std::env;
use std::io;
mod arguments;
mod iptables;
mod rfw;

fn run() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    arguments::validate(&args)?;
    rfw::run(&args[1], &args[2])?;
    return Ok(());
}
fn main() {
    match run() {
        Ok(_) => println!("Done"),
        Err(e) => eprintln!("{}", e),
    };
}
