use rand::Rng;
use std::env;
use std::process;
mod quotes;
mod help;

/*
 *   author Gabriel ogabrielpereira@pm.me
 *   version 1.0.0
 *   since Mar 7, 2022
 */

fn main() {
    let args: Vec<String> = env::args().collect(); // get input from cli - help
    
    if args.len() != 2 {
        help::run();
        process::exit(1);
    }

    let master = &args[1];
    
    let rnm = rand::thread_rng().gen_range(1..6);

    if master.trim() == "help" {
        quotes::run(rnm);
    } else {
        help::run();
    }

}
