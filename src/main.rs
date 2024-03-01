use std::env;
use carben;

pub fn main() {
    let version = carben::version();
    println!("version {version}");
    let args: Vec<String> = env::args().collect();
    let argc = args.len();
    println!("argc {argc}");
}
