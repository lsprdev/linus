/*
When I write "linus" on the terminal it should return 
some Linus Torvalds' quote.(At least 10 quotes)

Crates:
1 - rand
2 - clap 
*/

use rand::Rng;
use std::env;
//use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let master = &args[1];

    if master == "help"{
        let num = rand::thread_rng().gen_range(0..4);
        println!("");
        match num {
            0 => println!("“Talk is cheap. Show me the code.” \n ― Linus Torvalds"),
            1 => println!("“Bad programmers worry about the code. Good programmers worry \n about data structures and their relationships.” \n ― Linus Torvalds"),
            2 => println!("“Intelligence is the ability to avoid doing work, yet getting the \n work done.” \n ― Linus Torvalds"),
            3 => println!("“I like offending people, because I think people who get offended should \n be offended.” \n ― Linus Torvalds"),
            4 => println!("“Only wimps use tape backup. REAL men just upload their important \n stuff on ftp and let the rest of the world mirror it.” \n ― Linus Torvalds"),
            _ => println!("Try again!"),
        }
        println!("");
    }





    
}
