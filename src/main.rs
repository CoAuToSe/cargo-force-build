// <<<<<<< HEAD
// use std::env;

// fn main() {
//     println!("Hello, world! (WIP)");
//     // need to get the env for the folder "target"
//     // need to clean partialy the target dir with default value being all level available (debug, release, ..)
//     //// possibilities to choose the level to clear
//     // then run the "build" command
// =======
// use std::{
//     env::{self, *},
//     path::{self, *},
// };

// fn main() {
//     println!("Hello, world! (WIP)");
//     // need to get the env for the folder "target"
//     // need to clean partialy the target dir with default value being all level available (debug, release, ..)
//     //// possibilities to choose the level to clear
//     // then run the "build" command
// }
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(parse(from_os_str))]
    infile: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    outfile: Option<PathBuf>,
    // >>>>>>> 67d3a381b0216326540a244672070f2beb7e9286
}

fn main() {
    let opts = Opts::from_args();

    println!("{:?}", opts);
}
