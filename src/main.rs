use rand::Rng;
use std::env;
mod quotes;
mod help;
mod version;

/*  linus -- it shows some random linus' quote.
    Copyright (C) 2022  Gabriel L. Pereira.

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

 */

/*
 *   author Gabriel L. Pereira ogabrielpereira@pm.me
 *   version 0.1.0
 *   since Mar 7, 2022
 */
// Rewrite it with clap
fn main() {
    let args: Vec<String> = env::args().collect(); // get input from cli - help

    if args.len() == 1 {
    let rnm = rand::thread_rng().gen_range(1..6); // It gets a random number
    //rnm(random number main)
    
    quotes::run(rnm); 
    } 
    else if args[1] == "--h" || args[1] == "--help" {
        help::run();
    } 
    else if args[1] == "--v" || args[1] == "--version" {
        version::run();
    } 
    else {
        help::run();
    }

}
